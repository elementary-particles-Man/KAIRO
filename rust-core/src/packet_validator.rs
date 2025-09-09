// ===========================
// 📄 rust-core/src/packet_validator.rs
// ===========================

// Validate AITcpPacket fields, signatures, and consistency.
use crate::ai_tcp_packet_generated::aitcp as fb;
use crate::signature::verify_ed25519;
use ed25519_dalek::{Signature as Ed25519Signature, VerifyingKey};

/// Verify an ed25519 signature and emit log output for each step.
fn verify_packet_signature(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Ed25519Signature,
) -> Result<(), String> {
    println!("🔵 Verifying signature... (msg {} bytes)", message.len());
    match verify_ed25519(verifying_key, message, signature) {
        Ok(_) => {
            println!("🟢 Signature valid");
            Ok(())
        }
        Err(e) => {
            println!("🔴 Signature verification failed: {:?}", e);
            Err("Signature verification failed".into())
        }
    }
}

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
    println!("🔵 Validating packet (expected seq {})", expected_sequence);

    // Verify sequence number length
    let seq_vec = packet.encrypted_sequence_id();
    println!("🔵 Sequence field length: {}", seq_vec.len());
    if seq_vec.len() != 8 {
        println!("🔴 Invalid sequence ID length: {}", seq_vec.len());
        return Err("Invalid sequence ID length".into());
    }

    // Extract sequence number
    let mut seq_bytes = [0u8; 8];
    for (dst, src) in seq_bytes.iter_mut().zip(seq_vec.iter()) {
        *dst = src;
    }
    let seq = u64::from_le_bytes(seq_bytes);
    println!("🔵 Sequence number extracted: {}", seq);
    if seq != expected_sequence {
        println!(
            "🔴 Sequence ID mismatch: expected {}, got {}",
            expected_sequence, seq
        );
        return Err(format!(
            "Sequence ID mismatch: expected {}, got {}",
            expected_sequence, seq
        ));
    }

    // Prepare signature
    let sig_vec = packet.signature();
    println!("🔵 Signature length: {}", sig_vec.len());
    if sig_vec.len() != 64 {
        println!("🔴 Invalid signature length: {}", sig_vec.len());
        return Err("Invalid signature length".into());
    }
    let mut sig_bytes = [0u8; 64];
    for (dst, src) in sig_bytes.iter_mut().zip(sig_vec.iter()) {
        *dst = src;
    }
    let signature = Ed25519Signature::from_bytes(&sig_bytes);

    // Verify payload
    let payload_vec = packet.encrypted_payload();
    println!("🔵 Payload length: {}", payload_vec.len());
    if payload_vec.is_empty() {
        println!("🔴 Empty payload not allowed");
        return Err("Empty payload".into());
    }
    let message: Vec<u8> = payload_vec.iter().collect();

    // Delegate to helper
    verify_packet_signature(verifying_key, &message, &signature)?;

    println!("🟢 Packet validation succeeded");
    Ok(())
}
