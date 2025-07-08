// D:\dev\KAIRO\rust-core\tests\packet_parser_test.rs
use rust_core::packet_parser::PacketParser;
use flatbuffers::FlatBufferBuilder;
use rust_core::ai_tcp_packet_generated::aitcp as fb;

#[test]
fn test_packet_parsing_success() {
    // 1. FlatBufferBuilderを初期化
    let mut builder = FlatBufferBuilder::new();

    // 2. スキーマに存在するフィールドを全て指定してAITcpPacketを作成
    let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
    let nonce_vec = builder.create_vector(&[0u8; 12]);
    let seq_id: u64 = 12345;
    let seq_id_vec = builder.create_vector(&seq_id.to_le_bytes());
    let payload_vec = builder.create_vector(&[0u8; 0]); // ダミーの空ペイロード
    let signature_vec = builder.create_vector(&[0u8; 64]); // ダミーの署名

    let packet_offset = fb::AITcpPacket::create(&mut builder, &fb::AITcpPacketArgs{
        version: 1,
        ephemeral_key: Some(ephemeral_key_vec),
        nonce: Some(nonce_vec),
        encrypted_sequence_id: Some(seq_id_vec),
        // コンパイラの指摘に従い、必須フィールドを追加する
        encrypted_payload: Some(payload_vec),
        signature: Some(signature_vec),
    });
    builder.finish(packet_offset, None);
    let buf = builder.finished_data();

    // 3. パーサーのインスタンスを作成して、parseメソッドを呼び出す
    let mut parser = PacketParser::new(vec![]);
    let result = parser.parse(buf);

    // 4. 正しくパースできることを確認
    assert!(result.is_ok());
    let packet = fb::root_as_aitcp_packet(buf).expect("Failed to parse packet");
}

#[test]
fn detects_sequence_mismatch() {
    let mut parser = PacketParser::new(vec![]);
    let mut packet1 = 1u64.to_be_bytes().to_vec();
    packet1.extend_from_slice(b"hello");
    let result = parser.parse(&packet1);
    assert!(result.is_ok());

    // Reusing same sequence id should trigger error
    let result_err = parser.parse(&packet1);
    assert!(result_err.is_err());
}