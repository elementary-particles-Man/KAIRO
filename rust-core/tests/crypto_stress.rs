use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rust_core::ai_tcp_packet_generated::aitcp as fb;
use rust_core::log_recorder::LogRecorder;
use rust_core::packet_parser::PacketParser;
use rust_core::signature::{sign_ed25519, verify_ed25519};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_crypto_stress_multi_threaded() {
    let num_threads = 10;
    let iterations_per_thread = 100;
    let log_recorder = Arc::new(Mutex::new(LogRecorder::new()));

    let mut handles = vec![];

    for _ in 0..num_threads {
        let log_recorder_clone = Arc::clone(&log_recorder);
        let handle = thread::spawn(move || {
            let mut csprng = OsRng;
            for i in 0..iterations_per_thread {
                // --- Key Generation (修正点) ---
                // SigningKey::generate で鍵ペアを生成する
                let signing_key = SigningKey::generate(&mut csprng);
                let verifying_key: VerifyingKey = VerifyingKey::from(&signing_key);

                // --- Packet Building ---
                let mut builder = flatbuffers::FlatBufferBuilder::new();
                let payload = builder.create_vector(&[i as u8; 10]);
                let packet = fb::AITcpPacket::create(
                    &mut builder,
                    &fb::AITcpPacketArgs {
                        payload: Some(payload),
                        ..Default::default()
                    },
                );
                builder.finish(packet, None);
                let buf = builder.finished_data();

                // --- Signing ---
                // 生成したkeypairをそのまま署名に使用します。
                let signature = sign_ed25519(&signing_key, buf);

                // --- Verification ---
                // keypairから公開鍵(.public)を取り出して検証に使用します。
                let verification_result = verify_ed25519(&verifying_key, buf, &signature);
                assert!(verification_result.is_ok(), "Signature verification failed");

                // --- Parsing ---
                let mut parser = PacketParser::new(vec![]);
                let parsed_packet = parser.parse(buf);
                assert!(parsed_packet.is_ok());

                // --- Logging ---
                let mut recorder = log_recorder_clone.lock().unwrap();
                // TODO: implement LogRecorder::log
                // recorder.log(&format!(
                //     "Thread {:?}, Iteration {}: OK",
                //     thread::current().id(),
                //     i
                // ));
                // Simulate some work
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let _final_logs = log_recorder.lock().unwrap();
    // TODO: implement LogRecorder::get_logs
    // let final_logs = log_recorder.lock().unwrap().get_logs();
    // assert_eq!(final_logs.len(), num_threads * iterations_per_thread);
    println!("Crypto stress test completed successfully.");
}