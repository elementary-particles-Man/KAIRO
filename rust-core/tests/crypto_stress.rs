use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use flatbuffers::FlatBufferBuilder;
use rust_core::ai_tcp_packet_generated::aitcp as fb;
use rust_core::log_recorder::LogRecorder;
use rust_core::packet_parser::PacketParser;
use rust_core::signature::{sign_ed25519, verify_ed25519};

#[test]
fn crypto_stress() {
    // 繰り返し回数（必要に応じて増減可）
    for _ in 0..100 {
        // --- 1️⃣ HMAC鍵ローテーション確認 ---
        let mut recorder = LogRecorder::new(vec![1; 32]);
        recorder.rotate_key_if_needed();
        assert_eq!(recorder.key().len(), 32);

        // --- 2️⃣ FlatBuffersパケット生成＆パース（ゼロコピー構造確認） ---
        let mut builder = FlatBufferBuilder::new();

        let ephemeral_key_vec = builder.create_vector(&[1u8; 32]);
        let nonce_vec = builder.create_vector(&[0u8; 12]);
        let seq_id: u64 = 1;
        let seq_id_vec = builder.create_vector(&seq_id.to_le_bytes());
        let payload_vec = builder.create_vector(&[0u8; 0]);
        let signature_vec = builder.create_vector(&[0u8; 64]);

        let packet_offset = fb::AITcpPacket::create(
            &mut builder,
            &fb::AITcpPacketArgs {
                version: 1,
                ephemeral_key: Some(ephemeral_key_vec),
                nonce: Some(nonce_vec),
                encrypted_sequence_id: Some(seq_id_vec),
                encrypted_payload: Some(payload_vec),
                signature: Some(signature_vec),
            },
        );

        builder.finish(packet_offset, None);
        let buf = builder.finished_data();

        let mut parser = PacketParser::new(Vec::new());
        let _packet = parser.parse(buf).expect("Packet parsing failed");

        // --- 3️⃣ Ed25519 署名生成＆検証 ---
        let mut csprng = rand::thread_rng();
        let keypair = SigningKey::generate(&mut csprng);
        let message: &[u8] = b"test-message-for-signature";

        let sig = sign_ed25519(&keypair, message);
        assert!(verify_ed25519(&keypair.verifying_key(), message, &sig));
    }
}
