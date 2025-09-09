// ===========================
// 📄 rust-core/tests/packet_validator_test.rs
// ===========================

use ed25519_dalek::{SigningKey, VerifyingKey};
use flatbuffers::FlatBufferBuilder;
use kairo_core::ai_tcp_packet_generated::aitcp as fb;
use kairo_core::keygen::ephemeral_key;
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

/// Build a packet with a raw signature value.
fn build_packet_with_sig(seq: u64, payload: &[u8], signature: &[u8]) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_vec = builder.create_vector(&seq.to_le_bytes());
    let payload_vec = builder.create_vector(payload);
    let sig_vec = builder.create_vector(signature);
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
    let _buf = build_packet(1, &key, payload);
    let packet = fb::root_as_aitcp_packet(&_buf).unwrap();
    assert!(validate_packet(&packet, &VerifyingKey::from(&key), 1).is_ok());
}

#[test]
fn validate_wrong_sequence() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let payload = b"hello";
    let _buf = build_packet(1, &key, payload);
    let packet = fb::root_as_aitcp_packet(&_buf).unwrap();
    let err = validate_packet(&packet, &VerifyingKey::from(&key), 2).unwrap_err();
    assert!(err.contains("Sequence ID mismatch"));
}

#[test]
fn validate_bad_signature() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let payload = b"hello";
    let _buf = build_packet(1, &key, payload);

    // Tamper signature using zeroed bytes
    let tampered_buf = build_packet_with_sig(1, payload, &[0u8; 64]);
    let packet = fb::root_as_aitcp_packet(&tampered_buf).unwrap();
    let err = validate_packet(&packet, &VerifyingKey::from(&key), 1).unwrap_err();
    assert!(err.contains("Signature verification failed"));
}

#[test]
fn validate_missing_signature() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let payload = b"hello";
    let buf = build_packet_with_sig(1, payload, &[]);
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    let err = validate_packet(&packet, &VerifyingKey::from(&key), 1).unwrap_err();
    assert_eq!(err, "Invalid signature length");
}
