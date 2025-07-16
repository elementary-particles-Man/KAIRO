"""Example script for collecting logs from distributed nodes."""
import argparse
from pathlib import Path
from typing import List
from time import sleep

from src.log_recorder import LogRecorder


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Collect logs from nodes")
    parser.add_argument("nodes", nargs="+", help="List of node IP addresses")
    parser.add_argument(
        "--deny",
        action="store_true",
        help="Mark logs as denied events",
    )
    parser.add_argument(
        "--count", type=int, default=1, help="Number of logs to create per node"
    )
    parser.add_argument(
        "--dest", default=str(Path("vov/log.jsonl")), help="Path to output log file"
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    Path("vov").mkdir(exist_ok=True)
    logger = LogRecorder(args.dest)

    for _ in range(args.count):
        for ip in args.nodes:
            logger.log(ip, args.deny)
            sleep(0.1)


if __name__ == "__main__":
    main()
