import json
import os
import time
from datetime import datetime, timedelta

class LogRecorder:
    def __init__(self, log_file="vov/log.jsonl", signing_key_path="vov/signing_key.txt"):
        self.log_file = log_file
        self.signing_key_path = signing_key_path
        self._ensure_log_file_exists()
        self._ensure_signing_key_exists()
        self._key = self._read_signing_key()
        self._key_start = datetime.utcnow()

    def _ensure_log_file_exists(self):
        os.makedirs(os.path.dirname(self.log_file), exist_ok=True)
        if not os.path.exists(self.log_file):
            with open(self.log_file, "w") as f:
                pass  # Create empty file

    def _ensure_signing_key_exists(self):
        if not os.path.exists(self.signing_key_path):
            # In a real scenario, this would generate a secure key
            with open(self.signing_key_path, "w") as f:
                f.write("dummy_signing_key")

    def _read_signing_key(self):
        with open(self.signing_key_path, "r") as f:
            return f.read().strip()

    def _generate_signature(self, payload):
        # This is a dummy signature for demonstration purposes.
        # In a real system, this would involve cryptographic signing.
        return f"dummy_signature_{hash(payload)}_{self._key}"

    def log(self, source_ip, deny_flag):
        # Simplified log for testing purposes
        uuid = str(time.time())
        payload = "test_payload"
        timestamp = datetime.utcnow().isoformat() + "Z"
        data_to_sign = f"{uuid}{timestamp}{source_ip}{deny_flag}{payload}"
        signature = self._generate_signature(data_to_sign)
        log_entry = {
            "uuid": uuid,
            "timestamp": timestamp,
            "hash": f"dummy_hash_{hash(data_to_sign)}",
            "source_ip": source_ip,
            "signature": signature,
            "deny_flag": deny_flag,
            "payload": payload,
        }
        with open(self.log_file, "a") as f:
            f.write(json.dumps(log_entry) + "\n")
        return log_entry

    def log_event(self, uuid, source_ip, deny_flag, payload):
        timestamp = datetime.utcnow().isoformat() + "Z"
        # In a real scenario, hash would be of the signed payload
        data_to_sign = f"{uuid}{timestamp}{source_ip}{deny_flag}{payload}"
        signature = self._generate_signature(data_to_sign)
        log_entry = {
            "uuid": uuid,
            "timestamp": timestamp,
            "hash": f"dummy_hash_{hash(data_to_sign)}",
            "source_ip": source_ip,
            "signature": signature,
            "deny_flag": deny_flag,
            "payload": payload,
        }
        with open(self.log_file, "a") as f:
            f.write(json.dumps(log_entry) + "\n")

    def log_error(self, uuid, source_ip, transaction_id, error_type, payload):
        timestamp = datetime.utcnow().isoformat() + "Z"
        data_to_sign = f"{uuid}{timestamp}{source_ip}{transaction_id}{error_type}{payload}"
        signature = self._generate_signature(data_to_sign)
        log_entry = {
            "uuid": uuid,
            "timestamp": timestamp,
            "hash": f"dummy_hash_{hash(data_to_sign)}",
            "source_ip": source_ip,
            "signature": signature,
            "deny_flag": True,
            "transaction_id": transaction_id,
            "error_type": error_type,
            "payload": payload,
        }
        with open(self.log_file, "a") as f:
            f.write(json.dumps(log_entry) + "\n")

    def rotate_signing_key(self):
        # In a real scenario, this would generate a new secure key
        with open(self.signing_key_path, "w") as f:
            f.write(f"new_dummy_signing_key_{int(time.time())}")
        self._key = self._read_signing_key()
        self._key_start = datetime.utcnow()
        print("Signing key rotated.")