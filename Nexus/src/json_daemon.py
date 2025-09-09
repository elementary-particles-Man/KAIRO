from __future__ import annotations

import json
import os
import shutil
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, Optional

from pywinauto.application import Application

from .chat import windows as winchat
from .utils import pid_detect


# ===== デフォルト設定 =====
DEFAULT_SETTINGS = {
    # 監視
    "poll_interval_sec": 5,
    "inbox_json_dir": "inbox_json",
    "processed_json_dir": "processed_json",

    # 宛先自動検出
    "auto_detect_pids": True,

    # 送信方法
    "submit_mode": "enter_single",     # "enter_single" / "enter_double"
    "type_pause_sec": 0.02,
    "pre_submit_delay_sec": 0.6,
    "input_method": "paste",           # "paste" / "type" / "set"(ValuePatternが使えるなら推奨)

    # 応答キャプチャ（UIA）
    "response_capture_timeout_sec": 60,
    "response_stability_wait_sec": 1.2,
    "response_poll_interval_sec": 0.3,
    "min_growth_chars": 80,            # 送信直後テキストからの増分がこの文字数以上になって初めて安定判定に入る

    # OCR（通常は不要、UIAで拾えない環境のみ true）
    "embed_ocr_in_json": False,
    "tesseract_path": "",
    "ocr_lang": "eng",
    "post_send_wait_sec": 30.0,        # 送信後に必ず待つ秒数
}


# ===== ユーティリティ =====
def ensure_dirs(*paths: Path) -> None:
    for p in paths:
        p.mkdir(parents=True, exist_ok=True)


def load_json(path: Path) -> Optional[Dict[str, Any]]:
    if not path.exists():
        return None
    try:
        with path.open("r", encoding="utf-8") as f:
            return json.load(f)
    except Exception as e:
        print(f"[jsond] JSON読込失敗: {path} error={e}")
        return None


def read_text_smart(p: Path) -> str:
    """
    BOM(utf-8/utf-16) 検出＋ cp932 フォールバックを含む安全読取。
    PowerShellの Out-File 既定(UTF-16LE)でも文字化けしない。
    """
    b = p.read_bytes()
    # UTF-8 BOM
    if b.startswith(b"\xef\xbb\xbf"):
        return b[3:].decode("utf-8", errors="replace")
    # UTF-16 LE/BE
    if b.startswith(b"\xff\xfe"):
        return b[2:].decode("utf-16-le", errors="replace")
    if b.startswith(b"\xfe\xff"):
        return b[2:].decode("utf-16-be", errors="replace")
    # 既定はUTF-8、失敗ならCP932
    try:
        return b.decode("utf-8")
    except UnicodeDecodeError:
        return b.decode("cp932", errors="replace")


def is_mature_file(p: Path, wait_sec: float = 0.2) -> bool:
    """
    書きかけファイルを避けるため、短時間でサイズが安定しているか確認。
    """
    try:
        s1 = p.stat().st_size
        time.sleep(wait_sec)
        s2 = p.stat().st_size
        return s1 == s2 and s1 > 0
    except Exception:
        return False


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
        if found:
            for k, v in found.items():
                if isinstance(v, dict) and "pid" in v:
                    addresses.setdefault(k, {})["pid"] = v.get("pid")
            print(f"[jsond] 自動検出PIDs: {found}")

    return addresses


def validate_message(obj: Dict[str, Any]) -> Optional[str]:
    """
    { "from": "...", "to": "...", "intent": "chat", "payload": { "text": "..." }, "trace": { "id": "..." } }
    """
    for k in ("from", "to", "intent", "payload", "trace"):
        if k not in obj:
            return f"missing:{k}"
    if obj.get("intent") != "chat":
        return f"intent:{obj.get('intent')}"
    if not isinstance(obj.get("payload"), dict) or not isinstance(obj.get("payload", {}).get("text"), str):
        return "payload.text"
    if not isinstance(obj.get("trace"), dict) or not isinstance(obj.get("trace", {}).get("id"), str):
        return "trace.id"
    return None


def archive_and_save(src_path: Path, data: Dict, processed_dir: Path, ok: bool, note: str = ""):
    """
    結果JSONを書き出し、元ファイルを削除（エラー時は緊急退避）。
    """
    sub = "ok" if ok else "error"
    ensure_dirs(processed_dir / sub)
    ts = datetime.now().strftime("%Y%m%d-%H%M%S")
    dst_name = f"{src_path.stem}-{ts}{('-'+note if note else '')}{src_path.suffix}"
    dst_path = processed_dir / sub / dst_name

    try:
        with dst_path.open("w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
        src_path.unlink(missing_ok=True)
    except Exception as e:
        print(f"[jsond] アーカイブ失敗: {src_path} -> {dst_path} error={e}")
        try:
            shutil.move(str(src_path), str(processed_dir / "error" / src_path.name))
        except Exception as move_e:
            print(f"[jsond] 緊急アーカイブ失敗: {move_e}")


# ===== 応答キャプチャ（UIA） =====
def _pick_text_from_window(main_window) -> str:
    """
    Document/Edit を優先してテキストを抽出。
    ValuePattern(CurrentValue) が使えない場合は window_text() を使用。
    """
    try:
        # Document 優先（最も大きい矩形を採用）
        docs = main_window.descendants(control_type="Document")
        if docs:
            target = max(docs, key=lambda w: (w.rectangle().width() * w.rectangle().height()))
            try:
                return target.iface_value.CurrentValue  # type: ignore[attr-defined]
            except Exception:
                return target.window_text()

        # Edit 次点
        edits = main_window.descendants(control_type="Edit")
        if edits:
            target = max(edits, key=lambda w: (w.rectangle().width() * w.rectangle().height()))
            try:
                return target.iface_value.CurrentValue  # type: ignore[attr-defined]
            except Exception:
                return target.window_text()

        # フォールバック（全子孫結合）
        all_texts = [elem.window_text() for elem in main_window.descendants() if elem.window_text()]
        return "\n".join(all_texts)
    except Exception as e:
        print(f"[jsond] テキスト抽出中にエラー: {e}")
        return ""


def capture_response_from_pid(
    pid: int,
    timeout: float,
    stability_wait: float,
    poll_interval: float,
    min_growth: int,
) -> Optional[str]:
    """
    送信直後のベースラインを取得 → 文字数増分(min_growth)が発生してから
    “内容不変の継続時間=stability_wait” を満たした時点で確定。
    """
    print(f"[jsond] 応答キャプチャ開始: pid={pid} timeout={timeout}s")
    try:
        app = Application(backend="uia").connect(process=pid, timeout=10)
        main_window = app.top_window()
    except Exception as e:
        print(f"[jsond] PIDへの接続失敗: {e}")
        return None

    start = time.time()
    baseline = _pick_text_from_window(main_window)
    last = baseline
    changed = False
    stable_time = 0.0

    while time.time() - start < timeout:
        cur = _pick_text_from_window(main_window)

        if not changed:
            # ベースラインから一定以上の増加が起きて初めて「返信が来た」とみなす
            if len(cur) >= len(baseline) + max(1, min_growth) and cur != last:
                changed = True
                stable_time = 0.0
        else:
            # すでに増加後：安定待ち
            if cur == last:
                stable_time += poll_interval
                if stable_time >= stability_wait:
                    print(f"[jsond] 応答安定検出。chars={len(cur)} (+{len(cur)-len(baseline)})")
                    return cur
            else:
                stable_time = 0.0

        last = cur
        time.sleep(poll_interval)

    print(f"[jsond] タイムアウト。増加={len(last)-len(baseline)} chars で確定")
    return last


# ===== メインループ =====
def main() -> int:
    if os.name != "nt":
        print("[jsond] Windows専用です。終了します。")
        return 1

    base = Path.cwd()
    cfg_dir = base / "config"

    # 設定ロード
    settings = DEFAULT_SETTINGS.copy()
    loaded = load_json(cfg_dir / "settings.json") or load_json(cfg_dir / "settings.json.example")
    if isinstance(loaded, dict):
        settings.update(loaded)

    # ディレクトリ
    inbox = base / settings.get("inbox_json_dir", "inbox_json")
    processed = base / settings.get("processed_json_dir", "processed_json")
    ensure_dirs(inbox, processed, processed / "ok", processed / "error")

    # 宛先テーブル
    addresses = resolve_addresses(settings)

    # 時間・送信系
    poll = float(settings.get("poll_interval_sec", 5))
    submit_mode = str(settings.get("submit_mode", "enter_double"))
    type_pause = float(settings.get("type_pause_sec", 0.03))
    pre_submit = float(settings.get("pre_submit_delay_sec", 0.25))
    input_method = str(settings.get("input_method", "paste"))

    # キャプチャ系
    resp_timeout = float(settings.get("response_capture_timeout_sec", 60))
    resp_stability = float(settings.get("response_stability_wait_sec", 2.0))
    resp_poll = float(settings.get("response_poll_interval_sec", 0.5))
    min_growth = int(settings.get("min_growth_chars", 80))

    # OCR（任意）
    embed_ocr = bool(settings.get("embed_ocr_in_json", False))
    tesseract_path = str(settings.get("tesseract_path", "") or "")
    ocr_lang = str(settings.get("ocr_lang", "eng"))

    print(f"[jsond] 起動: inbox={inbox} processed={processed} poll={poll}s")

    try:
        while True:
            files = sorted([p for p in inbox.glob("*.json") if p.is_file()])
            if not files:
                time.sleep(poll)
                continue

            for f in files:
                # 書きかけ回避
                if not is_mature_file(f, wait_sec=0.2):
                    continue

                # 1) 読込（BOM検出+cp932フォールバック）
                try:
                    content = read_text_smart(f)
                    data = json.loads(content)
                except Exception as e:
                    print(f"[jsond] 読込失敗: {f.name} error={e}")
                    archive_and_save(f, {}, processed, ok=False, note="readerr")
                    continue

                # 2) 検証
                err = validate_message(data)
                if err:
                    print(f"[jsond] 検証失敗: {f.name} note={err}")
                    archive_and_save(f, data, processed, ok=False, note=f"invalid-{err}")
                    continue

                # 3) 宛先/送信元 PID 解決
                to_key = str(data.get("to", ""))
                from_key = str(data.get("from", ""))

                to_pid_val = addresses.get(to_key, {}).get("pid")
                from_pid_val = addresses.get(from_key, {}).get("pid")
                terminal_pid_val = addresses.get("terminal", {}).get("pid")  # fallback 用

                if not to_pid_val:
                    print(f"[jsond] PID未設定(to): key={to_key}")
                    archive_and_save(f, data, processed, ok=False, note="no-to-pid")
                    continue

                if not from_pid_val and not terminal_pid_val:
                    print(f"[jsond] PID未設定(from/fallback): from={from_key}")
                    archive_and_save(f, data, processed, ok=False, note="no-from-pid")
                    continue

                try:
                    to_pid = int(to_pid_val)
                    reply_pid = int(from_pid_val) if from_pid_val else int(terminal_pid_val)
                except Exception:
                    print(f"[jsond] PID不正: to={to_pid_val} from={from_pid_val} term={terminal_pid_val}")
                    archive_and_save(f, data, processed, ok=False, note="bad-pid")
                    continue

                # 4) 送信
                msg = str(data.get("payload", {}).get("text", ""))
                print(f"[jsond] 送信: {f.name} ({from_key} -> {to_key}) pid={to_pid}")
                send_ok = winchat.send_text_to_pid(
                    to_pid,
                    msg,
                    press_enter=True,
                    submit_mode=submit_mode,
                    type_pause=type_pause,
                    pre_submit_delay_sec=pre_submit,
                    input_method="paste",          # ★ 設定に関わらず paste を強制
                )
                if not send_ok:
                    print(f"[jsond] 送信失敗: {f.name}")
                    archive_and_save(f, data, processed, ok=False, note="senderr")
                    continue

                # ★★★ ここを追加：送信後の最低待機（既定30秒） ★★★
                post_wait = float(settings.get("post_send_wait_sec", 30.0))
                if post_wait > 0:
                    print(f"[jsond] 送信後待機: {post_wait}s")
                    time.sleep(post_wait)

                # 5) 応答キャプチャ（UIA）
                response_text = capture_response_from_pid(
                    to_pid,
                    timeout=resp_timeout,
                    stability_wait=resp_stability,
                    poll_interval=resp_poll,
                    min_growth=min_growth,
                )

                if response_text is None:
                    print(f"[jsond] 応答キャプチャ失敗: {f.name}")
                    data["nexus_log"] = "capture-failed"
                    archive_and_save(f, data, processed, ok=False, note="capture-failed")
                    continue

                # 6) 必要ならOCRも添付（通常は無効）
                if embed_ocr:
                    try:
                        from .ocr import windows_ocr  # 遅延import（環境に無い場合でも動作可）
                        ocr_text = windows_ocr.ocr_window(to_pid, tesseract_path=tesseract_path, lang=ocr_lang)
                    except Exception as _:
                        ocr_text = None
                else:
                    ocr_text = None

                data["received"] = {"text": response_text, "ts": datetime.now().isoformat()}
                if ocr_text:
                    data["received"]["ocr"] = ocr_text

                # 7) 送り主へ返信（無ければ terminal にフォールバック）
                print(f"[jsond] 返信: {f.name} ({to_key} -> {from_key or 'terminal'}) pid={reply_pid}")
                reply_ok = winchat.send_text_to_pid(
                    reply_pid,
                    response_text,
                    press_enter=True,
                    submit_mode=submit_mode,
                    type_pause=type_pause,
                    pre_submit_delay_sec=pre_submit,
                    input_method=input_method,
                )
                if not reply_ok:
                    print(f"[jsond] 返信失敗: {f.name}")
                    data["nexus_log"] = "reply-failed"
                    archive_and_save(f, data, processed, ok=False, note="reply-failed")
                    continue

                # 8) アーカイブ
                print(f"[jsond] サイクル完了: {f.name}")
                archive_and_save(f, data, processed, ok=True, note="ok")

            # 負荷軽減
            time.sleep(0.05)

    except KeyboardInterrupt:
        print("[jsond] 終了します。")
        return 0


if __name__ == "__main__":
    sys.exit(main())
