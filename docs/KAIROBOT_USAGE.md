//! docs/KAIROBOT_USAGE.md

# KAIROBOT Usage Guide

This document describes how to use KAIROBOT, the autonomous task execution engine of the KAIRO system.

---

## 🧠 Architecture Overview
KAIROBOT is structured in three layers:
- **Core**: Main loop and task state management
- **API**: Receives tasks via HTTP POST
- **Plugin**: Executes tasks via shell or other future methods

---

## 🚀 Running KAIROBOT

### Launch:
```bash
cargo run --bin kairobot
```

### Default behavior:
- Starts HTTP server at `http://localhost:4040`
- Listens for `/add_task` POST requests
- Runs shell command from the received JSON

---

## 📨 Sending a Task (PowerShell Example)
```powershell
$body = @{
  id = "task-001";
  name = "Hello KAIROBOT";
  command = "echo Hello from an external task!";
  status = "Pending"
}
$jsonData = $body | ConvertTo-Json -Compress
curl -X POST -H "Content-Type: application/json" -d $jsonData http://localhost:4040/add_task
```

---

## ✅ Expected Output (Example)
```
KAIROBOT Core: Main loop started.
API: Received new task -> Hello KAIROBOT
Executing task: Hello KAIROBOT (task-001)
Plugin(Shell): Running 'echo Hello from an external task!'
Hello from an external task!
Task task-001 finished.
```

---

## 🔒 Safety Note
All tasks are sandboxed within the shell context of the host OS. Ensure that commands are sanitized and authorized by operator.

---

## 📁 File Location
- Source: `src/bot/`
- Entry Point: `src/bot/main.rs`
- API: `src/bot/api/receiver.rs`
- Core: `src/bot/core/mod.rs`
- Plugin: `src/bot/plugin/shell.rs`

---

## 📌 See also
Add this line to README.md:
```markdown
🤖 KAIROBOTの使い方は [docs/KAIROBOT_USAGE.md](docs/KAIROBOT_USAGE.md) を参照。
```
