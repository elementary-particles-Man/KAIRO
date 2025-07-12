// ===========================
// 📄 rust-core/src/packet_validator.rs
// ===========================

// Validate AITcpPacket fields, signatures, and consistency.
use crate::ai_tcp_packet_generated::aitcp as fb;
use crate::signature::verify_ed25519;
use ed25519_dalek::{Signature as Ed25519Signature, VerifyingKey};

/// Validate an `AITcpPacket` by checking its sequence number and signature.
///
/// The sequence number is expected to be stored in little endian format in
/// `encrypted_sequence_id`. The signature is assumed to be over the
/// `encrypted_payload` bytes.
pub fn validate_packet(
    packet: &fb::AITcpPacket,
    verifying_key: &VerifyingKey,
    expected_sequence: u64,
) -> Result<(), String> {
    // Verify sequence number length
    let seq_vec = packet.encrypted_sequence_id();
    if seq_vec.len() != 8 {
        return Err("Invalid sequence ID length".into());
    }

    // Extract sequence number
    let mut seq_bytes = [0u8; 8];
    for (dst, src) in seq_bytes.iter_mut().zip(seq_vec.iter()) {
        *dst = src;
    }
    let seq = u64::from_le_bytes(seq_bytes);
    if seq != expected_sequence {
        return Err(format!("Sequence ID mismatch: expected {}, got {}", expected_sequence, seq));
    }

    // Prepare signature
    let sig_vec = packet.signature();
    if sig_vec.len() != 64 {
        return Err("Invalid signature length".into());
    }
    let mut sig_bytes = [0u8; 64];
    for (dst, src) in sig_bytes.iter_mut().zip(sig_vec.iter()) {
        *dst = src;
    }
    let signature = Ed25519Signature::from_bytes(&sig_bytes);

    // Verify signature
    let message: Vec<u8> = packet.encrypted_payload().iter().copied().collect();

    // Delegate to the shared helper for Ed25519 verification. Using the helper
    // keeps the logic consistent across crates and allows future changes (such
    // as domain separation) to be applied in one place.
    verify_ed25519(verifying_key, &message, &signature)
        .map_err(|_| "Signature verification failed".into())
}
