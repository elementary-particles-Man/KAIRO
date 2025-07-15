use ed25519_dalek::{Keypair, VerifyingKey};
use bytes::Bytes;
use kairo_rust_core::keygen::ephemeral_key;
use rand::rngs::OsRng;
use kairo_rust_core::ephemeral_session_generated::aitcp as fb;
use kairo_rust_core::log_recorder::LogRecorder;
use kairo_rust_core::packet_parser::PacketParser;
use kairo_rust_core::signature::{sign_ed25519, verify_ed25519};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_crypto_stress_multi_threaded() {
    let num_threads = 10;
    let iterations_per_thread = 100;
    let log_recorder: Arc<Mutex<LogRecorder>> = Arc::new(Mutex::new(LogRecorder::new()));

    let mut handles = vec![];

    for _ in 0..num_threads {
        let log_recorder_clone: Arc<Mutex<LogRecorder>> = Arc::clone(&log_recorder);
        let handle = thread::spawn(move || {
            for i in 0..iterations_per_thread {
                // --- Key Generation ---
                let keypair = Keypair::generate(&mut OsRng);
                let signing_key = keypair.secret;
                let verifying_key = keypair.public;
                let verifying_key = VerifyingKey::from(&signing_key);
                // signing_key and verifying_key generated above

                // --- Packet Building ---
                let mut builder = flatbuffers::FlatBufferBuilder::new();
                let session_id_str = format!("session-{}", i);
                let session_id = builder.create_string(&session_id_str);
                let public_key = builder.create_vector(&ephemeral_key());
                let ephemeral_session_offset = fb::EphemeralSession::create(
                    &mut builder,
                    &fb::EphemeralSessionArgs {
                        session_id: Some(session_id),
                        public_key: Some(public_key),
                        expiration_unix: 0, // ダミーの値
                    },
                );
                builder.finish(ephemeral_session_offset, None);
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
                let parsed_packet = parser.parse(&Bytes::from(buf.to_vec()));
                assert!(parsed_packet.is_ok());

                // --- Logging ---
                let _recorder = log_recorder_clone
                    .lock()
                    .expect("failed to lock log recorder");
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
        handle
            .join()
            .expect("crypto stress thread panicked");
    }

    let _final_logs = log_recorder
        .lock()
        .expect("failed to lock final log recorder");
    // TODO: implement LogRecorder::get_logs
    // let final_logs = log_recorder.lock().unwrap().get_logs();
    // assert_eq!(final_logs.len(), num_threads * iterations_per_thread);
    println!("Crypto stress test completed successfully.");
}
