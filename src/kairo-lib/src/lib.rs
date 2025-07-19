//! src/kairo-lib/src/lib.rs

// Publicly export all essential modules for external crates like kairo-server.

pub mod config;
pub mod governance;
pub mod packet;

// Re-export key data structures for easier access.
pub use config::AgentConfig;
pub use governance::OverridePackage;
pub use packet::AiTcpPacket;
