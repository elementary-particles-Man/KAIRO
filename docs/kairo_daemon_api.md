# Kairo Daemon HTTP API

This document describes the public endpoints exposed by `kairo_daemon`.

## `POST /assign_p_address`
Assign a new P address to a registering agent.

**Request Body**
```json
{ "public_key": "<hex-encoded-key>" }
```

**Response**
```json
{ "p_address": "10.0.0.5/24" }
```

Error codes:
- `400` invalid request
- `409` address allocation failed

## `POST /send`
Submit a signed AI-TCP packet to be routed.

**Sample Payload**
```json
{
  "source": "agent1",
  "destination": "agent2",
  "version": 1,
  "source_p_address": "10.0.0.1/24",
  "destination_p_address": "10.0.0.2/24",
  "source_public_key": "...",
  "sequence": 1,
  "timestamp_utc": 0,
  "payload_type": "text/plain",
  "payload": "hello",
  "signature": "..."
}
```

Responses:
- `200 OK` – `"packet_queued"`
- `400 BAD_REQUEST` – invalid packet

## `GET /receive/{p_address}`
Retrieve queued packets for the given destination.

Responses:
- `200 OK` – list of packets (may be empty)

## Common Error Codes
- `404` endpoint not found
- `500` internal server error
