use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use flatbuffers::FlatBufferBuilder;
use crate::ai_tcp_packet_generated::aitcp as fb;
use crate::log_recorder::LogRecorder;
use crate::packet_parser::PacketParser;
use rust_core::signature::{sign, verify};

#[test]
fn crypto_stress() {
    // 繰り返し回数（必要に応じて増減可）
    for _ in 0..100 {
        // --- 1️⃣ HMAC鍵ローテーション確認 ---
        let mut recorder = LogRecorder::new();
        recorder.rotate_key();

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
        let packet = parser.parse(buf).expect("Packet parsing failed");
        assert_eq!(packet.version(), 1); // version() 実装済前提

        // --- 3️⃣ Ed25519 署名生成＆検証 ---
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let message: &[u8] = b"test-message-for-signature";

        let sig = sign(&keypair, message);
        assert!(verify(&keypair.public, message, &sig));
    }
}
