from __future__ import annotations

import time
from typing import Optional


def send_text_to_pid(
    pid: int,
    text: str,
    press_enter: bool = True,
    focus_timeout_sec: float = 3.0,
    submit_mode: str = "enter_double",  # enter | enter_double | ctrl_enter | enter_then_ctrl_enter
    type_pause: float = 0.03,
    pre_submit_delay_sec: float = 0.25,
    input_method: str = "type",
) -> bool:
    try:
        from pywinauto import Application, keyboard
        try:
            from pywinauto import clipboard as _pwaclip
        except Exception:
            _pwaclip = None
    except Exception:
        print("[nexus.chat.windows] pywinauto が見つかりません。送信をスキップします。")
        return False

    try:
        app = Application(backend="uia").connect(process=pid, timeout=focus_timeout_sec)
        top = app.top_window()
        top.set_focus()
        # ゆっくりタイプ（特殊文字のエスケープは最小限）
        time.sleep(0.12)
        if input_method == "paste" and '_pwaclip' in locals() and _pwaclip is not None:
            _pwaclip.SetData(text)
            keyboard.send_keys("^v")
        else:
            for char in text:
                keyboard.send_keys(char, pause=type_pause)
        time.sleep(pre_submit_delay_sec)
        if press_enter:
            # IME確定などで最初のEnterが消費される場合に備えた送信戦略
            if submit_mode == "enter":
                keyboard.send_keys("{ENTER}")
            elif submit_mode == "enter_double":
                keyboard.send_keys("{ENTER}", pause=0.08)
                time.sleep(0.08)
                keyboard.send_keys("{ENTER}")
            elif submit_mode == "ctrl_enter":
                keyboard.send_keys("^({ENTER})")
            elif submit_mode == "enter_then_ctrl_enter":
                keyboard.send_keys("{ENTER}")
                time.sleep(0.12)
                keyboard.send_keys("^({ENTER})")
            else:
                keyboard.send_keys("{ENTER}")
        return True
    except Exception as e:
        print(f"[nexus.chat.windows] 送信失敗: pid={pid} error={e}")
        return False


def wait_for_response_stub(timeout_sec: float) -> Optional[str]:
    # MVP: 応答取得は未実装。タイムアウトのみ。
    deadline = time.time() + timeout_sec
    while time.time() < deadline:
        time.sleep(0.2)
    return None
