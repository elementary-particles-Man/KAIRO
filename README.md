# KAIRO

KAIRO is the client component used by **AI‑TCP** deployments. It provides a simple
way to connect to the main AI‑TCP server and acts as the "Vantage of View" (VoV)
observer. The client collects runtime metrics and forwards them to the AI‑TCP
network while keeping a local log.

## Directory structure

```
/         - repository root
logs/     - local logs produced by the VoV observer (ignored by Git)
src/      - client source code
scripts/  - helper scripts for running or packaging the client
```

Only the `logs` directory is created automatically; source code and scripts are
expected to live in the remaining directories as the project evolves.

## Basic usage

1. Ensure Python 3.11 (or later) is available.
2. Install required dependencies: `pip install -r requirements.txt` (if a
   requirements file is provided).
3. Run the client:

```bash
python src/main.py
```

On startup KAIRO will launch the VoV observer and begin sending data to your
configured AI‑TCP server. Runtime logs are written to `logs/vov.log` by default.

## VoV observer

The **VoV (Vantage of View) observer** monitors local events such as AI model
inference results or network changes. It writes a summarized JSON entry to the
log file after each observation cycle. Logs can be found in
`logs/vov.log`. Rotate or archive this file to prevent uncontrolled growth.

## Using KAIRO as a submodule

If you want to embed KAIRO in the main AI‑TCP repository, add it as a Git
submodule:

```bash
cd /path/to/AI-TCP
git submodule add <kairo-repo-url> client/kairo
git submodule update --init --recursive
```

After cloning a fresh AI‑TCP repository, initialize and pull the KAIRO submodule
with `git submodule update --init --recursive`.
