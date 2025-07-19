// src/kairo-lib/lib.rs

// 外部クレート
pub use serde::{Deserialize, Serialize};

// 内部モジュール
pub mod config;
pub mod packet;
pub mod governance;

// 必要な型を公開
pub use config::{AgentConfig, save_config};
pub use packet::AiTcpPacket;
pub use governance::OverridePackage;

// kairo_core の各種ユーティリティ
// ※ kairo_core をこのプロジェクトで使うなら、Cargo.toml の [dependencies] に kairo_core を追加してください。
//     path = "../../kairo-core" のようにローカル指定も可
pub use kairo_core::mesh_auditor;
pub use kairo_core::mesh_trust_calculator;
pub use kairo_core::packet_parser;
pub use kairo_core::baseline_profile_manager;
pub use kairo_core::signature;
pub use kairo_core::keygen;
pub use kairo_core::packet_validator;
pub use kairo_core::ai_tcp_packet_generated;
pub use kairo_core::ephemeral_session_generated;
pub use kairo_core::log_recorder;
pub use kairo_core::coordination;
