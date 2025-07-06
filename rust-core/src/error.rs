// D:\dev\KAIRO\rust-core\src\error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KairoError {
    #[error("FlatBuffers: Failed to parse packet")]
    PacketParseFailed,
}