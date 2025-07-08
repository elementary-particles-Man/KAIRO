// ===========================
// 📄 rust-core/src/lib.rs
// ===========================

// ---------- 外部クレート ----------


// ---------- 内部モジュール ----------
pub mod keygen;               // Ephemeral Key Generation
pub mod signature;            // Common signature helpers
pub mod force_disconnect;     // Force disconnect logic
pub mod fw_filter;            // Firewall filter logic
pub mod packet_parser;        // FlatBuffers parsing + sequence validation
pub mod packet_signer;        // Ephemeral Key signing for packets
pub mod compression;          // LZ4/Zstd compression utilities
pub mod session;              // Ephemeral DH session management
pub mod rate_control;         // Adaptive sending rate controller
pub mod log_recorder;         // VoV log recorder with HMAC & key rotation
pub mod ai_tcp_packet_generated; // FlatBuffers generated code
pub mod error;                // Custom error types

// ---------- Coordination Node Skeleton (Optional) ----------
// pub mod coordination;       // Uncomment when using coordination node

// ---------- Go連携用エクスポート関数 ----------
#[no_mangle]
pub extern "C" fn force_disconnect() {
    println!("Force disconnect triggered from Go!");
    // 必要に応じて VoV ログ処理などを呼び出す
}

#[no_mangle]
pub extern "C" fn example_function() {
    println!("Hello from rust-core cdylib! This proves that the DLL exports are working correctly.");
}

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn log_parse_error() {
    let err = error::KairoError::PacketParseFailed;
    eprintln!("Kairo error: {err}");
}

// Re-export LogRecorder for convenience
pub use crate::log_recorder::LogRecorder;
