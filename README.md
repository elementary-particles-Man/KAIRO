# KAIRO  
**AI-TCP Step2**

---

## Overview

KAIRO is the client component used by **AI‑TCP** deployments.  
It provides a simple way to connect to the main AI‑TCP server and acts as the  
**"Vantage of View (VoV)" observer**.  
The client collects runtime metrics and forwards them to the AI‑TCP network  
while keeping a local log for immutable auditability.

---

## Directory structure

```
/         - repository root
logs/     - local logs produced by the VoV observer (ignored by Git)
src/      - client source code (Python)
rust-core/ - optional Rust core for cryptographic operations
go-p2p/   - optional Go P2P node for distributed relay
scripts/  - helper scripts (e.g., PCAP generation)
```

Only the `logs` directory is created automatically; source code and scripts  
are expected to live in the remaining directories as the project evolves.

---

## Basic usage

1. Ensure **Python 3.11** (or later) is available.
2. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```
3. Run the VoV client:
   ```bash
   python src/main.py
   ```

On startup, KAIRO launches the VoV observer and begins sending data to your  
configured AI‑TCP server. Runtime logs are written to `logs/vov.log` by default.

---

## VoV observer

The **VoV (Vantage of View) observer** monitors local events such as AI model  
inference results or network anomalies. It writes summarized JSON entries to the log  
after each observation cycle. Logs are in **JSON Lines format** (`.jsonl`).  
Rotate or archive these files regularly to prevent uncontrolled growth.

---

## Using KAIRO as a submodule

If you want to embed KAIRO in the main AI‑TCP repository, add it as a Git submodule:

```bash
cd /path/to/AI-TCP
git submodule add https://github.com/elementary-particles-Man/KAIRO protocols/kairo-client
git submodule update --init --recursive
```

To pull the latest KAIRO updates:
```bash
git submodule update --remote protocols/kairo-client
```

---

## Log Collection (VoV Layer)

Logs are recorded by `src/log_recorder.py`.  
A helper script `src/collect_logs.py` aggregates logs from multiple nodes and appends  
them into `vov/log.jsonl`. The signing key rotates every 24 hours automatically.

Example: Generate logs from two example nodes
```bash
python -m src.collect_logs 192.168.1.10 192.168.1.11 --count 5
```

See `vov/README.md` for detailed log schema.

---

## Rust Core and Go P2P Components

### ✅ **Rust Core**

Handles ephemeral key generation, signature verification,  
FlatBuffers packet parsing, and **physical-layer force disconnect** logic.

Build it with:
```bash
cd rust-core
cargo build --release
```
Artifacts are placed in `rust-core/target/`.

---

### ✅ **Go P2P Node**

Manages P2P node discovery, relays VoV logs, and provides API endpoints  
for distributed coordination.

Build it with:
```bash
cd go-p2p
go build -o bin/p2p ./...
```
Executable output: `go-p2p/bin/`.

---

## Coordination Node Skeleton (AITCP-CORE-001)

The directory `AI-TCP/core/kairo_coord_node/` contains a Rust prototype of  
the self-governing Coordination Node. It manages peer public keys and assigns  
virtual Mesh IPs in the `100.64.0.0/16` range without relying on external services.

### UUID, Mesh IP and Key Management Flow

1. On startup, the node uses `KAIRO_NODE_ID` if provided or generates a 128-bit UUID.
2. Each peer gets a unique UUID and Mesh IP (`100.64.0.x`).
3. All creation and removal events are logged under `logs/CoordinationNode_YYYYMMDD.log`  
   with JST/ISO8601 timestamps.
4. Placeholder APIs for REST/gRPC are defined in the crate.

Run it with:
```bash
cd AI-TCP/core/kairo_coord_node
cargo run
```

---

## Developer-facing Errors

Client libraries raise structured exceptions with a `transaction_id`  
so failures can be correlated with VoV logs.

Common errors:
- `AuthenticationError`
- `TimeoutError`
- `ConnectionLostError`

Cryptographic faults are logged via `LogRecorder.log_error`  
but not exposed directly to external callers.

---

## Security Note

All AI-TCP packets handled by KAIRO are designed to be fully binary using **FlatBuffers**,  
with **Ed25519** signatures and **ChaCha20-Poly1305** encryption.  
Sequence management uses encrypted `sequence_id` fields, enabling KAIRO to detect packet loss  
and securely retransmit **entirely inside its Rust core**, remaining a complete **black box**  
to any external observer.

The only human-auditable record is the **VoV JSONL log**,  
which preserves immutable UUIDs, timestamps, and integrity hashes.

---

## Sample PCAP

See [samples/README.md](samples/README.md) for an example packet capture.
Regenerate the capture with `python scripts/generate_kairo_pcap.py`.

## OpenAI API Transition

See `docs/openai_api_compatibility_plan.md` for the phased deprecation plan  
and `cli-migrate` helper.

---

東京のタイムスタンプ（日本標準時）：2025-07-06 08:41
