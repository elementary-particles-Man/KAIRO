#!/usr/bin/env python3
"""Skeleton script for generating a simple PCAP capture.

Adjust `_build_packet` to craft the packets you need. The script writes
`samples/template.pcap` by default and skips file writes when `READ_ONLY=1`.
"""

import os
import struct
import time
import uuid
from pathlib import Path


def _build_packet(seq_id: int) -> bytes:
    """Return raw bytes for one IPv6 packet. Replace stub data as needed."""
    uid = uuid.uuid4()
    payload = f"payload-{seq_id}".encode()
    option_data = uid.bytes + payload
    option_header = struct.pack("!BB", 0x63, len(option_data))
    dest_opts = struct.pack("!BB", 59, len(option_data) + 2) + option_header + option_data

    payload_len = len(dest_opts)
    ipv6_header = struct.pack(
        "!IHBB16s16s",
        (6 << 28),
        payload_len,
        60,
        64,
        bytes.fromhex("20010db8000000000000000000000001"),
        bytes.fromhex("20010db8000000000000000000000002"),
    )
    return ipv6_header + dest_opts


def main() -> None:
    out_path = Path(__file__).resolve().parents[1] / "samples" / "template.pcap"
    if os.environ.get("READ_ONLY") == "1":
        print("Repository is read-only; run this script locally to generate the sample.")
        return

    out_path.parent.mkdir(exist_ok=True)
    packets = [_build_packet(1)]

    with out_path.open("wb") as fh:
        fh.write(struct.pack("<IHHIIII", 0xA1B2C3D4, 2, 4, 0, 0, 65535, 101))
        ts = int(time.time())
        for i, pkt in enumerate(packets):
            fh.write(struct.pack("<IIII", ts + i, 0, len(pkt), len(pkt)))
            fh.write(pkt)

    print(f"Template pcap written to {out_path}")


if __name__ == "__main__":
    main()
