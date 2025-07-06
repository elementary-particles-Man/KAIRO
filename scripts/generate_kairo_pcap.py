#!/usr/bin/env python3
"""Generate a sample pcap with IPv6 KAIRO destination options."""

import hashlib
import struct
import time
import uuid
from pathlib import Path


def _build_packet(seq_id: int) -> bytes:
    """Return raw bytes for a single IPv6 packet."""
    uid = uuid.uuid4()
    sha = hashlib.sha256(f"packet{seq_id}".encode()).digest()
    sig = hashlib.sha256(f"signature{seq_id}".encode()).digest()

    option_data = uid.bytes + struct.pack("!I", seq_id) + sha + sig
    option_header = struct.pack("!BB", 0x63, len(option_data))
    dest_opts = struct.pack("!BB", 59, 10) + option_header + option_data

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
    out_path = Path(__file__).resolve().parents[1] / "samples" / "kairo_ipv6_sample.pcap"
    out_path.parent.mkdir(exist_ok=True)

    packets = [_build_packet(i) for i in range(1, 4)]

    with out_path.open("wb") as fh:
        fh.write(struct.pack("<IHHIIII", 0xA1B2C3D4, 2, 4, 0, 0, 65535, 101))
        ts = int(time.time())
        for i, pkt in enumerate(packets):
            fh.write(struct.pack("<IIII", ts + i, 0, len(pkt), len(pkt)))
            fh.write(pkt)

    print(f"Sample pcap written to {out_path}")


if __name__ == "__main__":
    main()