// ===========================
// 📄 rust-core/tests/packet_validator_test.rs
// ===========================

use ed25519_dalek::{SigningKey, VerifyingKey};
use kairo_core::keygen::ephemeral_key;
use flatbuffers::FlatBufferBuilder;
use kairo_core::ai_tcp_packet_generated::aitcp as fb;
use kairo_core::packet_validator::validate_packet;
use kairo_core::signature::sign_ed25519;

/// Build a valid AITcpPacket FlatBuffer for testing.
fn build_packet(seq: u64, key: &SigningKey, payload: &[u8]) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_vec = builder.create_vector(&seq.to_le_bytes());
    let payload_vec = builder.create_vector(payload);
    let sig = sign_ed25519(key, payload);
    let sig_vec = builder.create_vector(sig.to_bytes().as_ref());
    let packet_offset = fb::AITcpPacket::create(
        &mut builder,
        &fb::AITcpPacketArgs {
            version: 1,
            ephemeral_key: Some(ephemeral_key_vec),
            nonce: Some(nonce_vec),
            encrypted_sequence_id: Some(seq_vec),
            encrypted_payload: Some(payload_vec),
            signature: Some(sig_vec),
            header: None,
            payload: None,
            footer: None,
        },
    );
    builder.finish(packet_offset, None);
    builder.finished_data().to_vec()
}

#[test]
fn validate_success() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let payload = b"hello";
    let buf = build_packet(1, &key, payload);
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    assert!(validate_packet(&packet, &VerifyingKey::from(&key), 1).is_ok());
}

#[test]
fn validate_wrong_sequence() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let payload = b"hello";
    let buf = build_packet(1, &key, payload);
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    assert!(validate_packet(&packet, &VerifyingKey::from(&key), 2).is_err());
}

#[test]
fn validate_bad_signature() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let payload = b"hello";
    let buf = build_packet(1, &key, payload);

    // Tamper signature: build new buffer with zeroed signature
    let mut builder = FlatBufferBuilder::new();
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_vec = builder.create_vector(&1u64.to_le_bytes());
    let payload_vec = builder.create_vector(payload);
    let sig_vec = builder.create_vector(&[0u8; 64]);
    let packet_offset = fb::AITcpPacket::create(
        &mut builder,
        &fb::AITcpPacketArgs {
            version: 1,
            ephemeral_key: Some(ephemeral_key_vec),
            nonce: Some(nonce_vec),
            encrypted_sequence_id: Some(seq_vec),
            encrypted_payload: Some(payload_vec),
            signature: Some(sig_vec),
            header: None,
            payload: None,
            footer: None,
        },
    );
    builder.finish(packet_offset, None);
    let tampered_buf = builder.finished_data();
    let packet = fb::root_as_aitcp_packet(tampered_buf).unwrap();

    assert!(validate_packet(&packet, &VerifyingKey::from(&key), 1).is_err());
}
