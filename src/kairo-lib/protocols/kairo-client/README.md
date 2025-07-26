# Kairo Client Example

This directory provides a simple HTTP interface implementing a few demo commands.

## Server Commands
- `generate_ephemeral_key`: create a random key used for HMAC signing.
- `verify_signature`: verify a message HMAC using the current key.
- `apply_deny_rules`: block a client by IP address.
- `log_vov`: log a message to stdout.
- `force_disconnect`: shut down the server.

## Running the Demo

Execute `example_usage.py` to start the server, interact with it, and force a shutdown:

```bash
python3 src/protocols/kairo-client/example_usage.py
```

The script demonstrates dummy traffic and shows `force_disconnect` closing the server.
