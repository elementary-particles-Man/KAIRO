# AI-TCP Mesh Node Discovery RFC

This RFC specifies how nodes discover peers within a given scope.
Includes Gossip propagation range, first-hop seeding, and fallback flows.

## Key Points
- get_gossip_range logic per scope (Personal, Family, etc.)
- Resilience to Seed Node loss
- Sybil resistance considerations
