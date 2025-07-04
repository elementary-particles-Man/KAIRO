// D:\dev\KAIRO\rust-core\src\log_recorder.rs
use hmac::Hmac;
use sha2::Sha256;
use chrono::{DateTime, Utc};

type HmacSha256 = Hmac<Sha256>;

pub struct LogRecorder {
    key: Vec<u8>,
    key_creation_time: DateTime<Utc>,
}

impl LogRecorder {
    pub fn new(initial_key: Vec<u8>) -> Self {
        Self {
            key: initial_key,
            key_creation_time: Utc::now(),
        }
    }
    // ダミーのキーローテーション関数
    pub fn rotate_key_if_needed(&mut self) {
        self.key = vec![0; 32]; // 新しいキーに更新
        self.key_creation_time = Utc::now();
    }
}