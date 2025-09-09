// src/kairo-lib/packet.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTcpPacket {
    pub source: String,
    pub destination: String,
    pub version: u8,
    pub source_p_address: String,
    pub destination_p_address: String,
    pub source_public_key: String,
    pub sequence: u64,
    pub timestamp_utc: u64,
    pub payload_type: String,
    pub payload: String,
    pub signature: String,
}
