use ed25519_dalek::{SigningKey, VerifyingKey};
use flatbuffers::FlatBufferBuilder;
use rand_core::OsRng;
use rust_core::ai_tcp_packet_generated::aitcp as fb;
use rust_core::packet_validator::validate_packet;
use rust_core::signature::sign_ed25519;

fn build_packet(seq: u64, key: &SigningKey, payload: &[u8]) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_vec = builder.create_vector(&seq.to_le_bytes());
    let payload_vec = builder.create_vector(payload);
    let sig = sign_ed25519(key, payload);
    let sig_vec = builder.create_vector(sig.as_ref());
    let packet_offset = fb::AITcpPacket::create(
        &mut builder,
        &fb::AITcpPacketArgs {
            version: 1,
            ephemeral_key: Some(ephemeral_key_vec),
            nonce: Some(nonce_vec),
            encrypted_sequence_id: Some(seq_vec),
            encrypted_payload: Some(payload_vec),
            signature: Some(sig_vec),
        },
    );
    builder.finish(packet_offset, None);
    builder.finished_data().to_vec()
}

#[test]
fn validate_success() {
    let key = SigningKey::generate(&mut OsRng);
    let payload = b"hello";
    let buf = build_packet(1, &key, payload);
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    assert!(validate_packet(&packet, &VerifyingKey::from(&key), 1));
}

#[test]
fn validate_wrong_sequence() {
    let key = SigningKey::generate(&mut OsRng);
    let payload = b"hello";
    let buf = build_packet(1, &key, payload);
    let packet = fb::root_as_aitcp_packet(&buf).unwrap();
    assert!(!validate_packet(&packet, &VerifyingKey::from(&key), 2));
}

#[test]
fn validate_bad_signature() {
    let key = SigningKey::generate(&mut OsRng);
    let payload = b"hello";
    let buf = build_packet(1, &key, payload);
    let mut packet = fb::root_as_aitcp_packet(&buf).unwrap();
    // Rebuild with tampered signature
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
        },
    );
    builder.finish(packet_offset, None);
    let tampered_buf = builder.finished_data();
    packet = fb::root_as_aitcp_packet(tampered_buf).unwrap();
    assert!(!validate_packet(&packet, &VerifyingKey::from(&key), 1));
}
