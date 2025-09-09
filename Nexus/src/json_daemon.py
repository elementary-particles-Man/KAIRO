from __future__ import annotations
import json
import os
import shutil
import time
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, Optional

from pywinauto.application import Application

from .chat import windows as winchat
from .utils import pid_detect

# --- グローバル設定 ---
BASE_DIR = Path(__file__).parent.resolve()
SETTINGS_FILE = BASE_DIR.parent / "config" / "settings.json"
ADDRESS_FILE = BASE_DIR.parent / "config" / "addresses.json"
INBOX_DIR = BASE_DIR.parent / "inbox_json"
PROCESSED_DIR = BASE_DIR.parent / "processed_json"

def load_json_config(path: Path, default: Dict = {}) -> Dict:
    """設定ファイルを安全に読み込む"""
    if not path.exists():
        print(f"[NEXUS-WARN] 設定ファイルが見つかりません: {path}")
        return default
    try:
        with path.open("r", encoding="utf-8") as f:
            return json.load(f)
    except Exception as e:
        print(f"[NEXUS-ERROR] 設定ファイルの読み込みに失敗しました: {path}, error={e}")
        return default

SETTINGS = load_json_config(SETTINGS_FILE, default={
    "poll_interval_sec": 5.0,
    "pre_submit_delay_sec": 0.5,
    "response_capture_timeout_sec": 60.0,
    "response_stability_wait_sec": 1.5,
    "response_poll_interval_sec": 0.3,
    "min_growth_chars": 50,
})

def resolve_addresses() -> Dict[str, Dict[str, Any]]:
    """PIDの自動検出と手動設定をマージする"""
    addresses = load_json_config(ADDRESS_FILE)
    
    try:
        found = pid_detect.detect_all()
        for key, value in found.items():
            addresses.setdefault(key, {})["pid"] = value.get("pid")
        print(f"[NEXUS] 自動検出されたPID: {found}")
    except Exception as e:
        print(f"[NEXUS-WARN] PIDの自動検出に失敗しました: {e}")
        
    print(f"[NEXUS] 現在のアドレス帳: {addresses}")
    return addresses

def read_text_smart(p: Path) -> str:
    """BOMを検出し、複数のエンコーディングを試行してファイルを安全に読み込む"""
    b = p.read_bytes()
    if b.startswith(b"\xef\xbb\xbf"):
        return b[3:].decode("utf-8", errors="replace")
    if b.startswith(b"\xff\xfe"):
        return b[2:].decode("utf-16-le", errors="replace")
    if b.startswith(b"\xfe\xff"):
        return b[2:].decode("utf-16-be", errors="replace")
    try:
        return b.decode("utf-8")
    except UnicodeDecodeError:
        return b.decode("cp932", errors="replace")

def is_mature_file(p: Path, wait_sec: float = 0.2) -> bool:
    """ファイルが書き込み中でないことを確認する"""
    try:
        s1 = p.stat().st_size
        time.sleep(wait_sec)
        s2 = p.stat().st_size
        return s1 == s2 and s1 > 0
    except FileNotFoundError:
        return False

def capture_response_from_pid(pid: int, timeout: float, stability_wait: float, poll_interval: float, min_growth: int) -> Optional[str]:
    """UIからの出力を安定してキャプチャする（堅牢化版）"""
    print(f"[NEXUS] PID:{pid}からの応答キャプチャを開始 (timeout={timeout}s)")
    try:
        app = Application(backend="uia").connect(process=pid, timeout=10)
        main_window = app.top_window()
    except Exception as e:
        print(f"[NEXUS-ERROR] PIDへの接続に失敗: {e}")
        return None

    def get_text_from_window() -> str:
        # Documentコントロールを最優先で探し、最大のものを取得
        try:
            docs = main_window.descendants(control_type="Document")
            if docs:
                target = max(docs, key=lambda w: w.rectangle().width() * w.rectangle().height())
                return getattr(target.iface_value, 'CurrentValue', target.window_text())
        except Exception:
            pass # 見つからない場合は次に進む
        
        # Editコントロールを次に試す
        try:
            edits = main_window.descendants(control_type="Edit")
            if edits:
                target = max(edits, key=lambda w: w.rectangle().width() * w.rectangle().height())
                return getattr(target.iface_value, 'CurrentValue', target.window_text())
        except Exception:
            pass

        return main_window.window_text()

    start_time = time.time()
    baseline_text = get_text_from_window()
    last_text = baseline_text
    response_started = False
    stable_since = 0

    while time.time() - start_time < timeout:
        current_text = get_text_from_window()

        if not response_started:
            # 応答がまだ始まっていない場合、ベースラインからの十分な増加を待つ
            if len(current_text) > len(baseline_text) + min_growth:
                print("[NEXUS] 応答の開始を検出。")
                response_started = True
        
        if response_started:
            # 応答が始まったら、内容が安定するのを待つ
            if current_text == last_text:
                if stable_since == 0:
                    stable_since = time.time()
                elif time.time() - stable_since >= stability_wait:
                    print(f"[NEXUS] 応答が安定しました。")
                    # 応答部分のみを返す
                    return current_text[len(baseline_text):].strip()
            else:
                stable_since = 0 # テキストが変化したのでリセット

        last_text = current_text
        time.sleep(poll_interval)

    print(f"[NEXUS-WARN] タイムアウトしました。キャプチャされた最新の応答を返します。")
    return last_text[len(baseline_text):].strip() if len(last_text) > len(baseline_text) else "Capture timed out."


def archive_task(src_path: Path, status: str, data: Dict, note: str = ""):
    """タスクファイルを結果と共にアーカイブする"""
    archive_dir = PROCESSED_DIR / status
    archive_dir.mkdir(parents=True, exist_ok=True)
    ts = datetime.now().strftime("%Y%m%d-%H%M%S")
    dst_name = f"{src_path.stem}-{ts}{('-'+note if note else '')}.json"
    dst_path = archive_dir / dst_name

    try:
        with dst_path.open("w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
        src_path.unlink(missing_ok=True)
    except Exception as e:
        print(f"[NEXUS-ERROR] アーカイブに失敗: {src_path} -> {dst_path}, error={e}")

def main():
    print("[NEXUS] KAIRO-Nexus デーモン起動...")
    INBOX_DIR.mkdir(exist_ok=True)
    PROCESSED_DIR.mkdir(exist_ok=True)
    
    addresses = resolve_addresses()

    try:
        while True:
            files = sorted([p for p in INBOX_DIR.glob("*.json") if p.is_file()])
            if not files:
                time.sleep(POLL_INTERVAL)
                continue

            for f in files:
                if not is_mature_file(f):
                    continue

                try:
                    content = read_text_smart(f)
                    data = json.loads(content)
                    
                    from_key = data.get("from")
                    to_key = data.get("to")
                    payload_text = data.get("payload", {}).get("text", "")
                    
                    if not all([from_key, to_key, payload_text]):
                        raise ValueError("不正なタスク形式です。")

                    to_info = addresses.get(to_key)
                    if not to_info or not to_info.get("pid"):
                         raise ValueError(f"宛先'{to_key}'のPIDが見つかりません。")

                    print(f"\n[NEXUS] タスク実行: {f.name} ({from_key} -> {to_key})")
                    
                    # 1. 送信
                    send_ok = winchat.send_text_to_pid(
                        to_info["pid"], payload_text, press_enter=True,
                        input_method=SETTINGS.get("input_method", "paste")
                    )
                    if not send_ok:
                        raise RuntimeError("送信に失敗しました。")

                    # 2. キャプチャ
                    response_text = capture_response_from_pid(
                        to_info["pid"],
                        timeout=SETTINGS.get("response_capture_timeout_sec"),
                        stability_wait=SETTINGS.get("response_stability_wait_sec"),
                        poll_interval=SETTINGS.get("response_poll_interval_sec"),
                        min_growth=SETTINGS.get("min_growth_chars"),
                    )
                    if response_text is None:
                        raise RuntimeError("応答のキャプチャに失敗しました。")

                    # 3. 返信
                    from_info = addresses.get(from_key)
                    if not from_info or not from_info.get("pid"):
                        raise ValueError(f"返信先'{from_key}'のPIDが見つかりません。")

                    reply_payload = json.dumps({
                        "from": to_key,
                        "to": from_key,
                        "intent": "chat_response",
                        "payload": {"text": response_text}
                    }, ensure_ascii=False)

                    reply_ok = winchat.send_text_to_pid(
                        from_info["pid"], reply_payload, press_enter=True,
                        input_method=SETTINGS.get("input_method", "paste")
                    )
                    if not reply_ok:
                        raise RuntimeError("返信に失敗しました。")

                    data["nexus_log"] = {"status": "ok", "response": response_text}
                    archive_task(f, "ok", data)

                except Exception as e:
                    print(f"[NEXUS-ERROR] タスク処理エラー: {f.name}, error={e}")
                    data["nexus_log"] = {"status": "error", "message": str(e)}
                    archive_task(f, "error", data)

    except KeyboardInterrupt:
        print("\n[NEXUS] シャットダウンします。")
    
if __name__ == "__main__":
    main()
