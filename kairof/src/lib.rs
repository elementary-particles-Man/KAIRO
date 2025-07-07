use flatbuffers::FlatBufferBuilder;
use rand::RngCore;
use rust_core::ai_tcp_packet_generated::aitcp as fb;
use std::time::{SystemTime, UNIX_EPOCH};

/// Build a single AITcpPacket with the given sequence id.
pub fn build_packet(seq_id: u64) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let mut ephemeral = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut ephemeral);
    let ephemeral_vec = builder.create_vector(&ephemeral);

    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    let nonce_vec = builder.create_vector(&nonce);

    let seq_bytes = seq_id.to_le_bytes();
    let seq_vec = builder.create_vector(&seq_bytes);

    let payload_vec = builder.create_vector(&[0u8; 0]);
    let signature_vec = builder.create_vector(&[0u8; 64]);

    let pkt = fb::AITcpPacket::create(&mut builder, &fb::AITcpPacketArgs {
        version: 1,
        ephemeral_key: Some(ephemeral_vec),
        nonce: Some(nonce_vec),
        encrypted_sequence_id: Some(seq_vec),
        encrypted_payload: Some(payload_vec),
        signature: Some(signature_vec),
    });
    builder.finish(pkt, None);
    builder.finished_data().to_vec()
}

/// Build a simple PCAP capture containing three packets.
pub fn build_pcap() -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(&0xA1B2C3D4u32.to_le_bytes());
    out.extend(&2u16.to_le_bytes());
    out.extend(&4u16.to_le_bytes());
    out.extend(&0u32.to_le_bytes());
    out.extend(&0u32.to_le_bytes());
    out.extend(&65535u32.to_le_bytes());
    out.extend(&101u32.to_le_bytes());

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    for i in 1..=3u32 {
        let pkt = build_packet(i as u64);
        out.extend(&(ts + i - 1).to_le_bytes());
        out.extend(&0u32.to_le_bytes());
        out.extend(&(pkt.len() as u32).to_le_bytes());
        out.extend(&(pkt.len() as u32).to_le_bytes());
        out.extend(&pkt);
    }

    out
}
