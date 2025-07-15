use kairo_rust_core::packet_parser::PacketParser;
use flatbuffers::FlatBufferBuilder;
use bytes::Bytes;
use kairo_rust_core::ai_tcp_packet_generated::aitcp as fb;
use kairo_rust_core::ephemeral_session_generated::aitcp as fb_ephemeral;

#[test]
fn test_packet_parsing_success() {
    // 1. FlatBufferBuilderを初期化
    let mut builder = FlatBufferBuilder::new();

    let session_id_str = "test-session-id";
    let session_id = builder.create_string(session_id_str);
    let public_key = builder.create_vector(&[1u8; 32]);

    let packet_offset = fb_ephemeral::EphemeralSession::create(&mut builder, &fb_ephemeral::EphemeralSessionArgs {
        session_id: Some(session_id),
        public_key: Some(public_key),
        expiration_unix: 0, // ダミーの値
    });
    builder.finish(packet_offset, None);
    let buf = builder.finished_data();

    // 3. パーサーのインスタンスを作成して、parseメソッドを呼び出す
    let mut parser = PacketParser::new(vec![]);
    let result = parser.parse(&Bytes::from(buf.to_vec()));

    // 4. 正しくパースできることを確認
    assert!(result.is_ok());
    let parsed_packet = result.unwrap();

    // 存在するフィールド 'version' を検証
    assert_eq!(parsed_packet.header.version, 1);
}
