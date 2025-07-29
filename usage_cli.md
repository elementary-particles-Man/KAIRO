# KAIRO CLI Usage Guide (for AI / LLM Integration)

This document describes how to use the core CLI tools for KAIRO. It is written for both human users and AI agents (e.g., Gemini, GPT, Codex) that interact with the system.

## 🔰 General Rule

All CLI tools follow the pattern:

```bash
cargo run --bin <binary_name> -- [ARGUMENTS...]
```

---

## 🧱 Core Binaries

### 1. `setup_agent`

Creates a new agent (node identity).

**Usage:**

```bash
cargo run --bin setup_agent -- --name Agent1
```

**Effect:**

- Generates: `agent_configs/Agent1.json`
- Stores P address and identity keys

**Notes:**

- Do NOT wrap agent name in extra quotes (`"Agent1"` ❌)

---

### 2. `signed_sender`

Sends a signed payload from an agent to another.

**Usage:**

```bash
cargo run --bin signed_sender -- --from Agent1 --to 10.0.0.2 --payload "Hello"
```

**Arguments:**

- `--from`: ID of sender agent (must match agent\_config)
- `--to`: P address of receiver
- `--payload`: Message content
- `--config`: (optional) config file path
- `--allow-mismatch`: bypass agent\_id check

---

### 3. `mesh_node`

Launches the mesh node for AI-TCP routing.

**Usage:**

```bash
cargo run --bin mesh_node -- --join-address 127.0.0.1:9000 --id mesh_01
```

**Effect:**

- Joins the P2P mesh
- Listens on port specified in config or command

---

### 4. `kairo_daemon`

Central P-address router and service hub.

**Usage:**

```bash
cargo run --package kairo_daemon
```

**Effect:**

- Listens on port 8080 by default
- Responds to agent requests

---

## 📄 Auxiliary Scripts

### `start_services.py`

Starts all core components and waits for readiness.

```bash
python start_services.py
```

---

## 📚 Important Notes

- Do not include quotes inside arguments (e.g., `"Agent1"` is invalid)
- If you see errors like `InvalidFilename`, check for hidden quotes or escapes
- All logs are UTF-8 plain text; avoid emoji in automated contexts

---

## 🧠 For AI agents (LLM/Gemini/Codex)

To understand KAIRO CLI behavior:

- Always scan `agent_configs/` to confirm valid identities
- When calling `setup_agent`, verify output file existence
- For `signed_sender`, check if `kairo_daemon` is running on port 8080
- Retry logic should detect port unavailability or connection refused
- Never assume success unless output or file system confirms it

---

## 📌 See also

Add this line in `README.md`:

```markdown
📘 CLIの使い方は [docs/USAGE_CLI.md](docs/USAGE_CLI.md) を参照。
```

