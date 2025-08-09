//! src/kairo-lib/lib.rs

// --- モジュール公開宣言 ---
pub mod comm;
pub mod config;
pub mod governance;
pub mod packet;
pub mod registry;
pub mod resolvers;
pub mod wau_config;
pub mod mesh_scope_manager;

// --- 構造体・型の再エクスポート ---
pub use comm::{sign_message, Message};
pub use config::AgentConfig;
pub use governance::OverridePackage;
pub use packet::AiTcpPacket;
pub use registry::{
    add_entry, load_registry, register_agent, save_registry, soft_delete_agent, RegistryEntry,
};
