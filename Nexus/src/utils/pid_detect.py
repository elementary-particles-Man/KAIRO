from __future__ import annotations

import json
import subprocess
from typing import Any, Dict, Optional


def _run_powershell(script: str) -> str:
    completed = subprocess.run(
        [
            "powershell.exe",
            "-NoProfile",
            "-Command",
            script,
        ],
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        return ""
    return completed.stdout.strip()


def detect_chatgpt() -> Optional[Dict[str, Any]]:
    ps = r"$p=Get-Process ChatGPT -ErrorAction SilentlyContinue | Where-Object {$_.MainWindowHandle -ne 0} | Select-Object -First 1 Id,ProcessName,MainWindowTitle; if($p){$p|ConvertTo-Json}else{''}"
    out = _run_powershell(ps)
    try:
        return json.loads(out) if out else None
    except json.JSONDecodeError:
        return None


def detect_gemini_chrome() -> Optional[Dict[str, Any]]:
    ps = r"$p=Get-Process chrome -ErrorAction SilentlyContinue | Where-Object {$_.MainWindowHandle -ne 0 -and $_.MainWindowTitle -match 'Gemini'} | Select-Object -First 1 Id,ProcessName,MainWindowTitle; if($p){$p|ConvertTo-Json}else{''}"
    out = _run_powershell(ps)
    try:
        return json.loads(out) if out else None
    except json.JSONDecodeError:
        return None


def detect_all() -> Dict[str, Dict[str, Any]]:
    result: Dict[str, Dict[str, Any]] = {}
    gpt = detect_chatgpt()
    if gpt:
        result["chatgpt_app"] = {"pid": gpt.get("Id"), "title": gpt.get("MainWindowTitle"), "name": gpt.get("ProcessName")}
    gm = detect_gemini_chrome()
    if gm:
        result["gemini_chrome"] = {"pid": gm.get("Id"), "title": gm.get("MainWindowTitle"), "name": gm.get("ProcessName")}
    return result

