use thiserror::Error;

#[derive(Error, Debug)]
pub enum KairoError {
    #[error("Packet parsing failed")]
    PacketParseFailed,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid packet format")]
    InvalidPacketFormat,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}
