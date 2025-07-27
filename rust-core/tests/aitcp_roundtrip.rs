use flatbuffers::FlatBufferBuilder;
use kairo_core::ai_tcp_packet_generated::aitcp as fb;

#[test]
fn aitcp_packet_binary_roundtrip() {
    let header = [0xAAu8; 8];
    let payload = [0xBBu8; 16];
    let footer = [0xCCu8; 4];

    let mut builder = FlatBufferBuilder::new();
    let header_vec = builder.create_vector(&header);
    let payload_vec = builder.create_vector(&payload);
    let signature_vec = builder.create_vector(&footer);
    let footer_vec = builder.create_vector(&footer);
    let ephemeral_vec = builder.create_vector(&[0u8; 32]);
    let source_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_vec = builder.create_vector(&1u64.to_le_bytes());
    let enc_payload_vec = builder.create_vector(&payload);

    let pkt = fb::AITcpPacket::create(
        &mut builder,
        &fb::AITcpPacketArgs {
            version: 1,
            ephemeral_key: Some(ephemeral_vec),
            source_public_key: Some(source_vec),
            nonce: Some(nonce_vec),
            encrypted_sequence_id: Some(seq_vec),
            encrypted_payload: Some(enc_payload_vec),
            signature: Some(signature_vec),
            header: Some(header_vec),
            payload: Some(payload_vec),
            footer: Some(footer_vec),
        },
    );
    builder.finish(pkt, None);
    let buf = builder.finished_data();

    let parsed = fb::root_as_aitcp_packet(buf).unwrap();
    assert_eq!(parsed.version(), 1);
    assert_eq!(parsed.header().unwrap().len(), header.len());
    assert_eq!(parsed.payload().unwrap().len(), payload.len());
    assert_eq!(parsed.footer().unwrap().len(), footer.len());
    assert_eq!(parsed.header().unwrap().iter().collect::<Vec<_>>(), header);
    assert_eq!(parsed.payload().unwrap().iter().collect::<Vec<_>>(), payload);
    assert_eq!(parsed.footer().unwrap().iter().collect::<Vec<_>>(), footer);
}
