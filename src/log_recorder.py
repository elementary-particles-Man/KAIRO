import json
import os
import uuid
from dataclasses import dataclass, asdict
from datetime import datetime, timedelta
from hashlib import sha256
import hmac


@dataclass
class LogEntry:
    uuid: str
    timestamp: str
    hash: str
    source_ip: str
    signature: str
    deny_flag: bool


@dataclass
class ErrorEntry(LogEntry):
    transaction_id: str
    error_type: str


class LogRecorder:
    """Record logs in JSON Lines format with key rotation."""

    def __init__(self, log_file: str):
        self.log_file = log_file
        self._key = os.urandom(32)
        self._key_start = datetime.utcnow()

    def _rotate_key_if_needed(self):
        if datetime.utcnow() - self._key_start > timedelta(hours=24):
            self._key = os.urandom(32)
            self._key_start = datetime.utcnow()

    def _sign(self, data: str) -> str:
        return hmac.new(self._key, data.encode(), sha256).hexdigest()

    def log(self, source_ip: str, deny_flag: bool) -> LogEntry:
        self._rotate_key_if_needed()
        uid = str(uuid.uuid4())
        timestamp = datetime.utcnow().isoformat()
        base_record = {
            "uuid": uid,
            "timestamp": timestamp,
            "source_ip": source_ip,
            "deny_flag": deny_flag,
        }
        payload = json.dumps(base_record, sort_keys=True)
        signature = self._sign(payload)
        base_record["signature"] = signature
        hash_val = sha256(
            json.dumps(base_record, sort_keys=True).encode()
        ).hexdigest()
        base_record["hash"] = hash_val
        entry = LogEntry(**base_record)
        with open(self.log_file, "a", encoding="utf-8") as fh:
            fh.write(json.dumps(asdict(entry)) + "\n")
        return entry

    def log_error(
        self, transaction_id: str, error_type: str, source_ip: str
    ) -> ErrorEntry:
        """Record a failure event for VoV auditing."""
        self._rotate_key_if_needed()
        uid = str(uuid.uuid4())
        timestamp = datetime.utcnow().isoformat()
        base_record = {
            "uuid": uid,
            "timestamp": timestamp,
            "transaction_id": transaction_id,
            "error_type": error_type,
            "source_ip": source_ip,
            "deny_flag": True,
        }
        payload = json.dumps(base_record, sort_keys=True)
        signature = self._sign(payload)
        base_record["signature"] = signature
        hash_val = sha256(
            json.dumps(base_record, sort_keys=True).encode()
        ).hexdigest()
        base_record["hash"] = hash_val
        entry = ErrorEntry(**base_record)
        with open(self.log_file, "a", encoding="utf-8") as fh:
            fh.write(json.dumps(asdict(entry)) + "\n")
        return entry
