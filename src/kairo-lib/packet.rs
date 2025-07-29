use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTcpPacket {
    pub source: String,
    pub destination: String,
    pub payload: String,
    pub source_public_key: String,
    pub destination_p_address: String,
    pub signature: String
}

