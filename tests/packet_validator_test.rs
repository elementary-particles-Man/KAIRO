use kairo_rust_core::ai_tcp_packet_generated::aitcp as fb;
use kairo_rust_core::packet_validator::validate_packet;
use kairo_rust_core::signature::sign_ed25519;
use kairo_rust_core::keygen::ephemeral_key;
use ed25519_dalek::{SigningKey, VerifyingKey};
use flatbuffers::FlatBufferBuilder;

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
fn test_validate_packet_success() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let buf = build_packet(42, &key, b"hello");
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    let res = validate_packet(&packet, &VerifyingKey::from(&key), 42);
    assert!(res.is_ok());
}

#[test]
fn test_validate_packet_invalid_sequence() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let buf = build_packet(1, &key, b"world");
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    let res = validate_packet(&packet, &VerifyingKey::from(&key), 2);
    assert!(res.is_err());
}

#[test]
fn test_validate_packet_invalid_signature() {
    let key = SigningKey::from_bytes(&ephemeral_key());
    let mut buf = build_packet(1, &key, b"data");
    // tamper signature: zero out signature field (offset from 0). We'll rebuild
    let mut builder = FlatBufferBuilder::new();
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_vec = builder.create_vector(&1u64.to_le_bytes());
    let payload_vec = builder.create_vector(b"data");
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
    buf = builder.finished_data().to_vec();
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    let res = validate_packet(&packet, &VerifyingKey::from(&key), 1);
    assert!(res.is_err());
}

#[test]
fn test_validate_packet_vector_length_mismatch() {
    let key = SigningKey::from_bytes(&ephemeral_key());

    // Build a valid packet for control
    let buf_valid = build_packet(5, &key, b"test");
    let packet_valid = fb::root_as_aitcp_packet(&buf_valid).unwrap();
    assert!(validate_packet(&packet_valid, &VerifyingKey::from(&key), 5).is_ok());

    // Build packet with incorrect sequence length (7 bytes instead of 8)
    let mut builder = FlatBufferBuilder::new();
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let short_seq = [0u8; 7];
    let seq_vec = builder.create_vector(&short_seq);
    let payload_vec = builder.create_vector(b"test");
    let sig = sign_ed25519(&key, b"test");
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
    let buf_invalid = builder.finished_data().to_vec();
    let packet_invalid = fb::root_as_aitcp_packet(&buf_invalid).unwrap();
    assert!(validate_packet(&packet_invalid, &VerifyingKey::from(&key), 0).is_err());
}
