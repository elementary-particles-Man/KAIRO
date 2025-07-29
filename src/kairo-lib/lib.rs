//! src/kairo-lib/lib.rs

// --- モジュール公開宣言 ---
pub mod config;
pub mod governance;
pub mod packet;
pub mod resolvers;
pub mod comm;
pub mod registry;

// --- 構造体・型の再エクスポート ---
pub use governance::OverridePackage;
pub use packet::AiTcpPacket;
pub use config::AgentConfig;
pub use comm::{Message, sign_message};
pub use registry::{RegistryEntry, load_registry, save_registry, add_entry};
