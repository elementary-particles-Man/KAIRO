from __future__ import annotations

import argparse
import json
import sys
import uuid
from datetime import datetime, timezone
from pathlib import Path


def load_json(path: Path):
    try:
        if path.exists():
            return json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        pass
    return None


def main() -> int:
    ap = argparse.ArgumentParser(description="Nexus JSON メッセージ投入CLI (原子的書き込み)")
    ap.add_argument("--to", required=True, help="送信先キー (config/addresses のキー名)")
    ap.add_argument("--text", help="本文テキスト。未指定ならSTDINを使用")
    ap.add_argument("--sender", default="terminal", help="from フィールド")
    ap.add_argument("--trace", default="", help="trace.id。未指定は自動生成")
    ap.add_argument("--inbox", default="", help="投入先フォルダ。未指定は設定(inbox_json_dir)から")
    args = ap.parse_args()

    base = Path.cwd()
    settings = load_json(base / "config" / "settings.json") or load_json(base / "config" / "settings.json.example") or {}
    inbox_dir = Path(args.inbox or settings.get("inbox_json_dir") or "inbox_json")
    inbox_dir = base / inbox_dir
    inbox_dir.mkdir(parents=True, exist_ok=True)

    text = args.text
    if text is None:
        text = sys.stdin.read()
    text = text or ""

    trace_id = args.trace or f"cli-{datetime.now(timezone.utc).strftime('%Y%m%d%H%M%S')}-{uuid.uuid4().hex[:8]}"

    msg = {
        "from": args.sender,
        "to": args.to,
        "intent": "chat",
        "payload": {"text": text, "meta": {"lang": "ja-JP"}},
        "nexus": {"response_timeout_sec": settings.get("response_timeout_sec", 20)},
        "trace": {"id": trace_id, "parent": None},
    }

    tmp = inbox_dir / f"{trace_id}.json.tmp"
    dst = inbox_dir / f"{trace_id}.json"
    tmp.write_text(json.dumps(msg, ensure_ascii=False, indent=2), encoding="utf-8")
    tmp.replace(dst)
    print(str(dst))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
