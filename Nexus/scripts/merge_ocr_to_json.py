from __future__ import annotations

import argparse
import json
from pathlib import Path
from datetime import datetime


def process_dir(dir_path: Path) -> int:
    count = 0
    for json_path in sorted(dir_path.glob("*.json")):
        try:
            data = json.loads(json_path.read_text(encoding="utf-8"))
        except Exception:
            continue

        # 既に受信がある場合はスキップ
        if isinstance(data, dict) and isinstance(data.get("received"), dict) and data["received"].get("text"):
            continue

        stem = json_path.with_suffix("")
        ocr_sidecar = stem.parent / (stem.name + "-ocr.txt")
        if not ocr_sidecar.exists():
            continue

        try:
            ocr_text = ocr_sidecar.read_text(encoding="utf-8")
        except Exception:
            continue

        try:
            if not isinstance(data, dict):
                data = {"raw": data}
            data["received"] = {"text": ocr_text, "ts": datetime.now().isoformat()}
            json_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
            count += 1
        except Exception:
            pass

    return count


def main() -> int:
    ap = argparse.ArgumentParser(description="Merge OCR sidecar text into processed JSON files")
    ap.add_argument("--dir", default="processed_json/ok", help="Directory containing processed JSON files")
    args = ap.parse_args()

    base = Path.cwd()
    target = base / args.dir
    target.mkdir(parents=True, exist_ok=True)

    updated = process_dir(target)
    print(f"[merge-ocr] updated={updated} dir={target}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

