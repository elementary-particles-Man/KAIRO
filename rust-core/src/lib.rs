// rust-core/src/lib.rs

pub mod ai_tcp_packet_generated;
pub mod coordination;
pub mod keygen;
pub mod log_recorder;
pub mod packet_parser;
pub mod signature;
pub mod ephemeral_session_generated;
pub mod error;
pub mod connection_manager;
pub mod force_disconnect;
pub mod fw_filter;
pub mod mesh;
pub mod packet_signer;
pub mod packet_validator;
pub mod rate_control;
pub mod session;
pub mod compression;

pub use crate::ai_tcp_packet_generated;
pub use crate::coordination;
pub use crate::keygen;
pub use crate::log_recorder;
pub use crate::packet_parser;
pub use crate::signature;
pub use crate::ephemeral_session_generated;
pub use crate::error;
pub use crate::connection_manager;
pub use crate::force_disconnect;
pub use crate::fw_filter;
pub use crate::mesh;
pub use crate::packet_signer;
pub use crate::packet_validator;
pub use crate::rate_control;
pub use crate::session;
pub use crate::baseline_profile_manager;
pub use crate::mesh_auditor;

// ======== 本体モジュール ========
pub mod mesh_trust_calculator;
pub fn example_function() {
    println!("Hello from kairo_rust_core!");
}
pub mod baseline_profile_manager;
pub mod mesh_auditor;

// ======== ユニットテスト ========

#[cfg(test)]
#[path = "../tests/aitcp_roundtrip.rs"]
pub mod aitcp_roundtrip;

#[cfg(test)]
#[path = "../tests/coordination_test.rs"]
pub mod coordination_test;

#[cfg(test)]
#[path = "../tests/crypto_stress.rs"]
pub mod crypto_stress;

#[cfg(test)]
#[path = "../tests/ephemeral_signature_test.rs"]
pub mod ephemeral_signature_test;

#[cfg(test)]
#[path = "../tests/key_rotation_test.rs"]
pub mod key_rotation_test;

#[cfg(test)]
#[path = "../tests/log_recorder_test.rs"]
pub mod log_recorder_test;

#[cfg(test)]
#[path = "../tests/mesh_auditor_test.rs"]
pub mod mesh_auditor_test;

#[cfg(test)]
#[path = "../tests/packet_parser_test.rs"]
pub mod packet_parser_test;

#[cfg(test)]
#[path = "../tests/packet_validator_test.rs"]
pub mod packet_validator_test;

#[cfg(test)]
#[path = "../tests/signature_verification_test.rs"]
pub mod signature_verification_test;