from __future__ import annotations

import json
import os
import shutil
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, Optional

from .queue import StakQueue
from .utils import pid_detect
from .chat import windows as winchat
from .ocr import windows_ocr


DEFAULT_SETTINGS = {
    "poll_interval_sec": 5,
    "response_timeout_sec": 20,
    "inbox_dir": "inbox",
    "processed_dir": "processed",
    "default_address_key": "chatgpt_app",
    "auto_detect_pids": True,
}


def load_json(path: Path) -> Optional[Dict[str, Any]]:
    if not path.exists():
        return None
    try:
        with path.open("r", encoding="utf-8") as f:
            return json.load(f)
    except Exception as e:
        print(f"[nexus] JSON読み込み失敗: {path} error={e}")
        return None


def ensure_dirs(*paths: Path) -> None:
    for p in paths:
        p.mkdir(parents=True, exist_ok=True)


def resolve_addresses(settings: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    base_dir = Path.cwd()
    cfg_dir = base_dir / "config"
    addr_path = cfg_dir / "addresses.json"
    addr_example = cfg_dir / "addresses.json.example"

    addresses: Dict[str, Dict[str, Any]] = {}

    file_data = load_json(addr_path) or load_json(addr_example)
    if isinstance(file_data, dict):
        addresses.update(file_data)

    if settings.get("auto_detect_pids", True):
        found = pid_detect.detect_all()
        # メモリ上で上書き（ファイルには書かない）
        if "chatgpt_app" in found:
            addresses.setdefault("chatgpt_app", {})["pid"] = found["chatgpt_app"].get("pid")
        if "gemini_chrome" in found:
            addresses.setdefault("gemini_chrome", {})["pid"] = found["gemini_chrome"].get("pid")
        if found:
            print(f"[nexus] 自動検出PIDs: {found}")

    return addresses


def choose_destination(addresses: Dict[str, Dict[str, Any]], key: str) -> Optional[Dict[str, Any]]:
    dest = addresses.get(key)
    if not dest:
        print(f"[nexus] 宛先キーが見つかりません: {key}")
        return None
    if not dest.get("pid"):
        print(f"[nexus] 宛先PIDが未設定です: key={key} value={dest}")
        return None
    return dest


def archive(src: Path, processed_dir: Path, ok: bool, note: str = "") -> None:
    sub = "ok" if ok else "error"
    ensure_dirs(processed_dir / sub)
    ts = datetime.now().strftime("%Y%m%d-%H%M%S")
    dst_name = f"{src.stem}-{ts}{'-'+note if note else ''}{src.suffix}"
    dst = processed_dir / sub / dst_name
    try:
        shutil.move(str(src), str(dst))
    except Exception as e:
        print(f"[nexus] アーカイブ失敗: {src} -> {dst} error={e}")


def parse_task(content: str) -> Dict[str, Any]:
    lines = [ln.strip("\ufeff").rstrip() for ln in content.splitlines()]
    non_empty = [ln for ln in lines if ln.strip()]
    if not non_empty:
        return {"type": "empty"}
    first = non_empty[0]
    if first.startswith("@stak"):
        payload = first[len("@stak"):].strip()
        rest = "\n".join(non_empty[1:]).strip()
        if rest:
            payload = f"{payload}\n{rest}" if payload else rest
        return {"type": "stak", "data": payload}
    return {"type": "message", "data": "\n".join(non_empty)}


def main() -> int:
    if os.name != "nt":
        print("[nexus] Windows専用です。終了します。")
        return 1

    base = Path.cwd()
    cfg_dir = base / "config"
    settings = DEFAULT_SETTINGS.copy()
    loaded = load_json(cfg_dir / "settings.json") or load_json(cfg_dir / "settings.json.example")
    if isinstance(loaded, dict):
        settings.update(loaded)

    inbox = base / settings.get("inbox_dir", "inbox")
    processed = base / settings.get("processed_dir", "processed")
    ensure_dirs(inbox, processed)
    ensure_dirs(processed / "ok", processed / "error")

    addresses = resolve_addresses(settings)
    dest_key = settings.get("default_address_key", "chatgpt_app")
    destination = choose_destination(addresses, dest_key)

    if not destination:
        print("[nexus] 宛先が設定できませんでした。config/addresses.json を編集するか、auto_detect_pids を有効にして再実行してください。")
        return 2

    pid = int(destination.get("pid"))
    poll = float(settings.get("poll_interval_sec", 5))
    resp_timeout = float(settings.get("response_timeout_sec", 20))
    submit_mode = str(settings.get("submit_mode", "enter_double"))
    type_pause = float(settings.get("type_pause_sec", 0.03))
    pre_submit = float(settings.get("pre_submit_delay_sec", 0.25))
    input_method = str(settings.get("input_method", "paste"))
    ocr_enabled = bool(settings.get("ocr_enabled", False))
    tesseract_path = str(settings.get("tesseract_path", "") or "")
    ocr_lang = str(settings.get("ocr_lang", "eng"))
    ocr_after_wait = bool(settings.get("ocr_after_wait", True))

    stak = StakQueue()
    print(f"[nexus] 起動: inbox={inbox} processed={processed} poll={poll}s dest={dest_key} pid={pid}")

    try:
        while True:
            # 1) inbox 監視
            to_process = sorted([p for p in inbox.glob("*") if p.is_file()])
            if not to_process:
                time.sleep(poll)
                continue

            for f in to_process:
                try:
                    content = f.read_text(encoding="utf-8")
                except Exception as e:
                    print(f"[nexus] 読み込み失敗: {f} error={e}")
                    archive(f, processed, ok=False, note="readerr")
                    continue

                task = parse_task(content)
                if task["type"] == "empty":
                    print(f"[nexus] 空ファイル: {f.name}")
                    archive(f, processed, ok=False, note="empty")
                    continue

                if task["type"] == "stak":
                    stak.push(task["data"])  # メモリキューに積むだけ
                    print(f"[nexus] @stak 受付: len={len(stak)} file={f.name}")
                    archive(f, processed, ok=True, note="stak")
                    continue

                # 2) メッセージ送信
                msg = task["data"]
                print(f"[nexus] 送信: {f.name} -> pid={pid} bytes={len(msg.encode('utf-8'))}")
                ok = winchat.send_text_to_pid(pid, msg, press_enter=True, submit_mode=submit_mode, type_pause=type_pause, pre_submit_delay_sec=pre_submit, input_method=input_method)

                # 3) 待機→OCR回収（任意）
                if ocr_after_wait:
                    try:
                        time.sleep(resp_timeout)
                    except KeyboardInterrupt:
                        raise
                ocr_text = None
                if ocr_enabled and ok:
                    ocr_text = windows_ocr.ocr_window(pid, tesseract_path=tesseract_path, lang=ocr_lang)

                # 4) 成否に応じてアーカイブ + OCR保存
                if ok:
                    # 先にアーカイブしてパスを確定
                    dst_dir = processed / "ok"
                    ensure_dirs(dst_dir)
                    ts = datetime.now().strftime("%Y%m%d-%H%M%S")
                    dst_name = f"{f.stem}-{ts}{f.suffix}"
                    dst_path = dst_dir / dst_name
                    try:
                        shutil.move(str(f), str(dst_path))
                    except Exception as e:
                        print(f"[nexus] アーカイブ失敗: {f} -> {dst_path} error={e}")
                    # OCRのサイドカー保存
                    if ocr_text:
                        ocr_sidecar = dst_path.with_suffix("")
                        ocr_sidecar = ocr_sidecar.parent / (ocr_sidecar.name + "-ocr.txt")
                        try:
                            ocr_sidecar.write_text(ocr_text, encoding="utf-8")
                            print(f"[nexus] OCR保存: {ocr_sidecar.name} chars={len(ocr_text)}")
                        except Exception as e:
                            print(f"[nexus] OCR保存失敗: {e}")
                else:
                    archive(f, processed, ok=False, note="senderr")

            # 短いスリープで次ループへ
            time.sleep(0.1)

    except KeyboardInterrupt:
        print("[nexus] 中断されました。")
        return 0


if __name__ == "__main__":
    sys.exit(main())
