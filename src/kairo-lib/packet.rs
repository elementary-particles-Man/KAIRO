use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTcpPacket {
    pub source: String,
    pub destination: String,
    pub payload: String,
}

// 既存の他のPacket構造体があればこの下に追記
