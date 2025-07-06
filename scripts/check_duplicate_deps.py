#!/usr/bin/env python3
"""Detect duplicated dependencies in Cargo.lock."""

from collections import defaultdict
from pathlib import Path
import re

LOCK = Path(__file__).resolve().parents[1] / "Cargo.lock"


def main() -> None:
    packages = defaultdict(set)
    name_re = re.compile(r"^name = \"(?P<name>[^\"]+)\"")
    version_re = re.compile(r"^version = \"(?P<ver>[^\"]+)\"")

    current_name = None
    with LOCK.open() as fh:
        for line in fh:
            line = line.strip()
            m = name_re.match(line)
            if m:
                current_name = m.group("name")
                continue
            m = version_re.match(line)
            if m and current_name:
                packages[current_name].add(m.group("ver"))
                current_name = None

    duplicates = {name: vers for name, vers in packages.items() if len(vers) > 1}
    if not duplicates:
        print("No duplicated dependencies found.")
        return

    print("Duplicated dependencies detected:")
    for name, vers in duplicates.items():
        vers_list = ", ".join(sorted(vers))
        print(f"  {name}: {vers_list}")
    print("Consider aligning these versions across crates.")


if __name__ == "__main__":
    main()
