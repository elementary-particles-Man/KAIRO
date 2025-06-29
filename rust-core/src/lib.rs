use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;

/// Submodules for KAIRO core logic
pub mod signature;       // Common signature helpers
pub mod packet_parser;   // FlatBuffers parsing + sequence validation
pub mod packet_signer;   // Ephemeral Key signing for packets

/// LogRecorder handles VoV log HMAC signing with daily key rotation.
pub struct LogRecorder {
    key: [u8; 32],
    key_start: DateTime<Utc>,
}

impl LogRecorder {
    /// Creates a new LogRecorder with a fresh ephemeral HMAC key.
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self {
            key,
            key_start: Utc::now(),
        }
    }

    /// Rotates the HMAC key every 24 hours to limit compromise window.
    fn rotate_key_if_needed(&mut self) {
        if Utc::now() - self.key_start > Duration::hours(24) {
            OsRng.fill_bytes(&mut self.key);
            self.key_start = Utc::now();
        }
    }

    /// Signs VoV log data with the current ephemeral key.
    pub fn sign(&mut self, data: &[u8]) -> Vec<u8> {
        self.rotate_key_if_needed();
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .expect("HMAC can take key of any size");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }

    /// Expose current ephemeral key for testing or chaining.
    pub fn key(&self) -> &[u8; 32] {
        &self.key
    }

    /// For unit tests: manually set key start time.
    #[cfg(test)]
    pub fn set_key_start(&mut self, time: DateTime<Utc>) {
        self.key_start = time;
    }

    /// For unit tests: get key start time.
    #[cfg(test)]
    pub fn key_start(&self) -> DateTime<Utc> {
        self.key_start
    }
}

/// Transmit encoded packet to the wire.
/// Currently just a stub to satisfy the API server.
pub fn transmit_packet(_packet: &[u8]) {
    // In real implementation this would send bytes over the network
}
