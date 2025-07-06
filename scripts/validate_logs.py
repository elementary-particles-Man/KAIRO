import argparse
import json
import sys
import re
from pathlib import Path
from typing import List
import glob

UUID_RE = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
TIMESTAMP_RE = re.compile(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z)?")
HASH_RE = re.compile(r"[0-9a-fA-F]{64}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate VoV and Coordination Node logs")
    parser.add_argument(
        "--check",
        nargs="+",
        required=True,
        help="List of log files or glob patterns to validate",
    )
    return parser.parse_args()


def expand_files(patterns: List[str]) -> List[str]:
    files: List[str] = []
    for p in patterns:
        matched = glob.glob(p)
        if not matched:
            print(f"No files matched: {p}")
            sys.exit(1)
        files.extend(matched)
    return files


def validate_line(path: str, line: str, line_no: int, errors: List[str]) -> None:
    try:
        data = json.loads(line)
    except json.JSONDecodeError as exc:
        errors.append(f"{path}:{line_no}: invalid JSON: {exc}")
        return

    expected_keys = ["uuid", "timestamp", "hash"]
    for key in expected_keys:
        if key not in data:
            errors.append(f"{path}:{line_no}: missing key '{key}'")
            return

    actual_order = list(data.keys())
    if actual_order[:3] != expected_keys:
        errors.append(
            f"{path}:{line_no}: key order {actual_order[:3]} does not match {expected_keys}"
        )

    if not UUID_RE.fullmatch(str(data["uuid"])):
        errors.append(f"{path}:{line_no}: invalid UUID '{data['uuid']}'")

    if not TIMESTAMP_RE.fullmatch(str(data["timestamp"])):
        errors.append(f"{path}:{line_no}: invalid timestamp '{data['timestamp']}'")

    if not HASH_RE.fullmatch(str(data["hash"])):
        errors.append(f"{path}:{line_no}: invalid hash '{data['hash']}'")


def main() -> int:
    args = parse_args()
    files = expand_files(args.check)

    errors: List[str] = []
    for path in files:
        if not Path(path).is_file():
            errors.append(f"File not found: {path}")
            continue
        with open(path, "r", encoding="utf-8") as fh:
            for i, line in enumerate(fh, 1):
                line = line.strip()
                if not line:
                    continue
                validate_line(path, line, i, errors)

    if errors:
        for err in errors:
            print(err)
        return 1

    print("Validation Passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
