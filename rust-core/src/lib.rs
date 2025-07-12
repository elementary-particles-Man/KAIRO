// ===========================
// 📄 rust-core/src/lib.rs
// ===========================

// ---------- 外部クレート ----------
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use rand::{thread_rng, RngCore};
use sha2::Sha256;

// ---------- 内部モジュール ----------
pub mod keygen;               // Ephemeral Key Generation
pub mod signature;            // Common signature helpers
pub mod force_disconnect;     // Force disconnect logic
pub mod fw_filter;            // Firewall filter logic
pub mod packet_parser;        // FlatBuffers parsing + sequence validation
pub mod packet_signer;        // Ephemeral Key signing for packets

pub mod compression;          // LZ4/Zstd compression utilities
pub mod session;              // Ephemeral DH session management
pub mod connection_manager;   // Manage ephemeral sessions per connection
pub mod rate_control;         // Adaptive sending rate controller
pub mod log_recorder;         // VoV log recorder with HMAC & key rotation
pub mod ai_tcp_packet_generated; // FlatBuffers generated code
pub mod error;                // Custom error types
pub mod coordination;         // Coordination Node Skeleton (Optional)
pub mod packet_validator;     // AITcpPacket validation logic

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

// ---------- LogRecorder 構造体 ----------
pub struct LogRecorder {
    key: [u8; 32],
    key_start: DateTime<Utc>,
    // 必要なら追加フィールド
}

// LogRecorder 実装
impl LogRecorder {
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        thread_rng().fill_bytes(&mut key);
        Self {
            key,
            key_start: Utc::now(),
        }
    }

    pub fn sign_log(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key).expect("HMAC init failed");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    pub fn rotate_key(&mut self) {
        self.key_start = Utc::now();
        thread_rng().fill_bytes(&mut self.key);
    }
}
