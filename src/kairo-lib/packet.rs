//! kairo-lib/src/packet.rs
//! Defines the formal AI-TCP packet structure.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiTcpPacket {
    pub version: u8,
    pub sequence: u32,
    pub timestamp: u64,
    pub from: String,
    pub to: String,
    pub payload: String,
}
