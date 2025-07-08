// D:\dev\KAIRO\rust-core\src\log_recorder.rs
use hmac::{Hmac, Mac};
use sha2::Sha256;
use chrono::{DateTime, Utc};
use rand::{rngs::OsRng, RngCore};

type HmacSha256 = Hmac<Sha256>;

pub struct LogRecorder {
    key: [u8; 32],
    key_start: DateTime<Utc>,
}

impl LogRecorder {
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
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
        OsRng.fill_bytes(&mut self.key);
    }}