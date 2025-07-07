// rust-core/src/error.rs

#[derive(Debug)]
pub enum KairoError {
    PacketParseError(String),
    CoordinationError(String),
    Other(String),
}

impl std::fmt::Display for KairoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for KairoError {}
