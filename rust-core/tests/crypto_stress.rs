use ed25519_dalek::{Keypair, Signer};
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
                // Keypairとして一体で生成します。
                let keypair: Keypair = Keypair::generate(&mut csprng);

                // --- Packet Building ---
                let mut builder = flatbuffers::FlatBufferBuilder::new();
                let payload = builder.create_vector(&[i as u8; 10]);
                let packet = fb::AITcpPacket::create(
                    &mut builder,
                    &fb::AITcpPacketArgs {
                        session_id: i as u32,
                        payload: Some(payload),
                        ..Default::default()
                    },
                );
                builder.finish(packet, None);
                let buf = builder.finished_data();

                // --- Signing ---
                // 生成したkeypairをそのまま署名に使用します。
                let signature = sign_ed25519(&keypair, buf);

                // --- Verification ---
                // keypairから公開鍵(.public)を取り出して検証に使用します。
                let verification_result = verify_ed25519(&keypair.public, buf, &signature);
                assert!(verification_result.is_ok(), "Signature verification failed");

                // --- Parsing ---
                let parsed_packet = PacketParser::parse(buf);
                assert!(parsed_packet.is_some());
                assert_eq!(parsed_packet.unwrap().session_id(), i as u32);

                // --- Logging ---
                let mut recorder = log_recorder_clone.lock().unwrap();
                recorder.log(&format!(
                    "Thread {:?}, Iteration {}: OK",
                    thread::current().id(),
                    i
                ));
                // Simulate some work
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_logs = log_recorder.lock().unwrap().get_logs();
    assert_eq!(final_logs.len(), num_threads * iterations_per_thread);
    println!("Crypto stress test completed successfully.");
}