#!/usr/bin/env python3
"""Regenerate FlatBuffers code from the schema."""

import os
import subprocess
import shutil
from pathlib import Path


SCHEMA = Path(__file__).resolve().parents[1] / "schema" / "ai_tcp_packet.fbs"
OUT_DIR = Path(__file__).resolve().parents[1] / "rust-core" / "src"


def main() -> None:
    if os.environ.get("READ_ONLY") == "1":
        print("Repository is read-only; run this script locally to regenerate the schema.")
        return

    flatc = shutil.which("flatc")
    if not flatc:
        print("Error: flatc compiler not found. Please install FlatBuffers and ensure `flatc` is on PATH.")
        return

    cmd = [flatc, "--rust", "-o", str(OUT_DIR), str(SCHEMA)]
    subprocess.run(cmd, check=True)
    print("FlatBuffers code regenerated.")


if __name__ == "__main__":
    main()
