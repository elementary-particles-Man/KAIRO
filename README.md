KAIRO
AI-TCP Step2

## Using this repository with AI-TCP

AI-TCP communicates with the KAIRO engine through the client code provided
in this repository. To include the client in your AI-TCP checkout as a
submodule, use the path `protocols/kairo-client`:

```bash
git submodule add https://github.com/elementary-particles-Man/KAIRO protocols/kairo-client
git submodule update --init --recursive
```

When you want to fetch updates from this repository, run:

```bash
git submodule update --remote protocols/kairo-client
```

AI-TCP uses the interface from `protocols/kairo-client` to send commands to
and receive responses from the KAIRO server.

## Log Collection

Logs are recorded in JSON Lines format using `src/log_recorder.py`. A helper
script `src/collect_logs.py` collects logs from multiple nodes and appends them
into `vov/log.jsonl`. The signing key rotates every 24 hours automatically.

To generate logs from two example nodes:

```bash
python -m src.collect_logs 192.168.1.10 192.168.1.11 --count 5
```

See `vov/README.md` for the log schema.
