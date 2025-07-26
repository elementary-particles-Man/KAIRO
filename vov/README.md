# VOV Logs

This directory stores JSON Lines (``.jsonl``) logs produced by `LogRecorder`.

Each log entry follows the schema:

| Field | Description |
|-------|-------------|
| `uuid` | Unique identifier for the event |
| `timestamp` | UTC ISO‑8601 time when the log was generated |
| `hash` | SHA‑256 hash of the signed payload |
| `source_ip` | IP address of the source that generated the log |
| `signature` | HMAC signature of the log payload |
| `deny_flag` | Boolean indicating if access was denied |

Failure logs created via `LogRecorder.log_error` additionally include the
fields `transaction_id` and `error_type` so that issues can be traced while the
VoV layer records the corresponding `uuid`, `timestamp` and `hash`.

Logs are appended to ``vov/log.jsonl`` and the signing key rotates every 24
hours. Example logs can be found in ``vov/example_log.jsonl``.
