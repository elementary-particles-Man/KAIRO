use hmac::{Hmac, Mac};
use sha2::Sha256;
use chrono::{DateTime, Utc};
use rand::{rngs::OsRng, RngCore};
use std::fs;
use std::path::Path;
use flatbuffers::FlatBufferBuilder;
use crate::ai_tcp_packet_generated::aitcp as fb;

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

    /// Export the current state to FlatBuffers and write to `path`.
    /// This encodes the HMAC key and key start timestamp using the
    /// existing `AITcpPacket` schema for simplicity.
    pub fn export_flatbuffers(&self, path: &Path) {
        let mut builder = FlatBufferBuilder::new();

        let key_vec = builder.create_vector(&self.key);
        let nonce_vec = builder.create_vector(&[0u8; 12]);
        let ts = self.key_start.timestamp_millis();
        let ts_vec = builder.create_vector(&ts.to_le_bytes());
        let payload_vec = builder.create_vector(&[] as &[u8]);
        let sig_vec = builder.create_vector(&[] as &[u8]);

        let packet = fb::AITcpPacket::create(
            &mut builder,
            &fb::AITcpPacketArgs {
                version: 1,
                ephemeral_key: Some(key_vec),
                nonce: Some(nonce_vec),
                encrypted_sequence_id: Some(ts_vec),
                encrypted_payload: Some(payload_vec),
                signature: Some(sig_vec),
                header: None,
                payload: None,
                footer: None,
            },
        );

        builder.finish(packet, None);
        let data = builder.finished_data();
        fs::write(path, data).expect("failed to write FlatBuffer log");
    }

    /// Attempt to recover state from a potentially corrupted log file.
    /// If parsing fails, the key is rotated and a fresh FlatBuffer log is written.
    pub fn recover_on_corruption(&mut self, path: &Path) {
        let data = fs::read(path).unwrap_or_default();
        if fb::root_as_aitcp_packet(&data).is_err() {
            self.rotate_key();
            self.export_flatbuffers(path);
        }
    }
}
