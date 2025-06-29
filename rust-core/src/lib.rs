use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;

pub mod signature;
pub mod packet_parser;
pub mod packet_signer;

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

    fn rotate_key_if_needed(&mut self) {
        if Utc::now() - self.key_start > Duration::hours(24) {
            OsRng.fill_bytes(&mut self.key);
            self.key_start = Utc::now();
        }
    }

    pub fn sign(&mut self, data: &[u8]) -> Vec<u8> {
        self.rotate_key_if_needed();
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .expect("HMAC can take key of any size");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    pub fn key(&self) -> &[u8; 32] {
        &self.key
    }

    #[cfg(test)]
    pub fn set_key_start(&mut self, time: DateTime<Utc>) {
        self.key_start = time;
    }

    #[cfg(test)]
    pub fn key_start(&self) -> DateTime<Utc> {
        self.key_start
    }
}
