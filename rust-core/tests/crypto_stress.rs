use ed25519_dalek::{SigningKey, VerifyingKey, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use rand::RngCore; // ← 必須
use bytes::Bytes;
use kairo_rust_core::keygen::ephemeral_key;
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
                let mut secret_bytes = [0u8; SECRET_KEY_LENGTH];
                OsRng.fill_bytes(&mut secret_bytes);
                let signing_key = SigningKey::from_bytes(&secret_bytes);
                let verifying_key = VerifyingKey::from(&signing_key);

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
                        expiration_unix: 0,
                    },
                );
                builder.finish(ephemeral_session_offset, None);
                let buf = builder.finished_data();

                // --- Signing ---
                let signature = sign_ed25519(&signing_key, buf);

                // --- Verification ---
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
    println!("Crypto stress test completed successfully.");
}
