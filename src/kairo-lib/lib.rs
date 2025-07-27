//! src/kairo-lib/lib.rs

// --- モジュール公開宣言 ---
pub mod config;
pub mod governance;
pub mod packet;
pub mod resolvers;

// --- 構造体・型の再エクスポート ---
pub use governance::OverridePackage;
pub use packet::AiTcpPacket;
pub use config::AgentConfig;
