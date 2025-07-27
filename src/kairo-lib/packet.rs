//! kairo-lib/src/packet.rs
//! Defines the formal AI-TCP packet structure with time synchronization.

use serde::{Deserialize, Serialize};
use ed25519_dalek::{Signer, SigningKey};
use hex;
use crate::AgentConfig;

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

/// Sign (sequence + timestamp_utc + payload) using the agent's secret key
pub fn sign_packet(
    config: &AgentConfig,
    sequence: u64,
    timestamp_utc: i64,
    payload: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let secret_bytes = hex::decode(&config.secret_key)?;
    let key_bytes: [u8; 32] = secret_bytes
        .try_into()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key length"))?;
    let signing_key = SigningKey::from_bytes(&key_bytes);
    let mut message = Vec::new();
    message.extend_from_slice(&sequence.to_le_bytes());
    message.extend_from_slice(&timestamp_utc.to_le_bytes());
    message.extend_from_slice(payload.as_bytes());
    let signature = signing_key.sign(&message);
    Ok(hex::encode(signature.to_bytes()))
}
