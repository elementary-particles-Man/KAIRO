//! kairo-lib/src/packet.rs
//! Defines the formal AI-TCP packet structure with time synchronization.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiTcpPacket {
    pub version: u32,
    pub source_p_address: String,
    pub destination_p_address: String,
    pub sequence: u64,      // Monotonically increasing counter per-source
    pub timestamp_utc: i64, // UNIX epoch seconds (UTC)
    pub payload_type: String, // e.g., "text/plain", "application/json"
    pub payload: String,
    pub source_public_key: String,
    pub signature: String, // Hex-encoded signature of (sequence + timestamp + payload)
}
