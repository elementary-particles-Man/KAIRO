// D:\dev\KAIRO\rust-core\src\log_recorder.rs
use hmac::{Hmac, Mac};
use sha2::Sha256;
use chrono::{DateTime, Utc};
use rand::{rngs::OsRng, RngCore};

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
    /// Sign arbitrary data with the current HMAC key
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("HMAC init");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    /// Verify the HMAC for the given data
    pub fn verify(&self, data: &[u8], tag: &[u8]) -> bool {
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("HMAC init");
        mac.update(data);
        mac.verify_slice(tag).is_ok()
    }

    /// Rotate the signing key
    pub fn rotate_key_if_needed(&mut self) {
        self.key_creation_time = Utc::now();
        let mut new_key = vec![0u8; 32];
        OsRng.fill_bytes(&mut new_key);
        self.key = new_key;
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }}