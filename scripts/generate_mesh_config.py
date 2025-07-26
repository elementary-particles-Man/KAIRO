#!/usr/bin/env python3
"""Generate mesh role configuration as TOML files."""
import argparse
import os
from pathlib import Path

DEFAULT_PERMS = {
    "coord": ["register", "issue_ip"],
    "relay": ["forward"],
    "observer": ["read"],
}

def build_config(role: str) -> str:
    perms = DEFAULT_PERMS.get(role, [])
    parts = [f'role = "{role}"']
    if perms:
        joined = ", ".join(f'"{p}"' for p in perms)
        parts.append(f'permissions = [{joined}]')
    return "\n".join(parts) + "\n"


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate mesh role config")
    parser.add_argument("role")
    parser.add_argument("--out-dir", default="mesh-node/config")
    args = parser.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    content = build_config(args.role)
    path = out_dir / f"{args.role}.toml"

    if os.environ.get("READ_ONLY") == "1":
        print(content, end="")
        return

    path.write_text(content, encoding="utf-8")
    print(f"Config written to {path}")


if __name__ == "__main__":
    main()
