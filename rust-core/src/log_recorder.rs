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
    pub fn rotate_key(&mut self) {
        self.key_start = Utc::now();
        OsRng.fill_bytes(&mut self.key);
    }

    /// Access the current key
    pub fn key(&self) -> &[u8] {
        &self.key
    }
}
