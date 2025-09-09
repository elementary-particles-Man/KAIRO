import json
from datetime import datetime, timedelta
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from src.log_recorder import LogRecorder


def test_log_creation(tmp_path):
    log_file = tmp_path / "log.jsonl"
    logger = LogRecorder(str(log_file))
    entry = logger.log("1.1.1.1", True)
    assert log_file.exists()
    lines = log_file.read_text().strip().split("\n")
    assert len(lines) == 1
    record = json.loads(lines[0])
    for field in [
        "uuid",
        "timestamp",
        "hash",
        "source_ip",
        "signature",
        "deny_flag",
    ]:
        assert field in record
    assert record["source_ip"] == "1.1.1.1"
    assert record["deny_flag"] is True
    assert record["uuid"] == entry["uuid"]


def test_key_rotation(tmp_path):
    log_file = tmp_path / "log.jsonl"
    logger = LogRecorder(str(log_file))
    first_key = logger._key
    # Simulate time passing beyond 24 hours
    logger.rotate_signing_key()
    logger.log("2.2.2.2", False)
    assert logger._key != first_key
    assert logger._key_start > datetime.utcnow() - timedelta(minutes=1)

def test_signature_changes_after_rotation(tmp_path):
    log_file = tmp_path / "log.jsonl"
    logger = LogRecorder(str(log_file))
    first = logger.log("3.3.3.3", False)
    logger.rotate_signing_key()
    second = logger.log("4.4.4.4", False)
    assert first["signature"] != second["signature"]
