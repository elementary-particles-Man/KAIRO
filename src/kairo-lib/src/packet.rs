//! kairo-lib/src/packet.rs
//! Defines the formal AI-TCP packet structure.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiTcpPacket {
    pub version: u32,
    pub source_p_address: String,
    pub destination_p_address: String,
    pub sequence: u64,
    pub timestamp: i64,
    pub payload_type: String,
    pub payload: String,
    pub signature: String, // Hex-encoded signature of the payload
}
