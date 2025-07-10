# RFC: AI-TCP Packet Layout v1

This document proposes a simple header/payload/footer structure for binary packets used by AI‑TCP. After editing `schema/ai_tcp_packet.fbs` run `scripts/update_flatbuffers.py` to regenerate the Rust bindings.

## Purpose

The three arrays allow small unencrypted metadata (`header`), a tiny control message (`payload`), and optional trailing information (`footer`). They are not encrypted by the main `encrypted_payload` field but may help with protocol negotiation or debugging.

## Encoding Hints

- Each field is a `ubyte` vector. Typical deployments keep the header under 16 bytes, payload under 32 bytes, and footer under 8 bytes.
- The contents may be plain bytes or, if desired, an encoded sub‑structure such as little‑endian integers or JSON.

## Relationship to LSC (Axiom)

The Logical Structure Consistency (LSC) axiom states that related data pieces should be grouped together and transmitted atomically. By explicitly separating header, payload, and footer sections, packet parsers can maintain strong logical consistency: each portion is accessed as an independent slice and validated before passing to the next stage.
