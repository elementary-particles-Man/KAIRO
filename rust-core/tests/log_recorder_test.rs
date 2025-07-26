use kairo_core::log_recorder::LogRecorder;
use kairo_core::ai_tcp_packet_generated::aitcp as fb;
use std::fs;

#[test]
fn hmac_rotation_verification() {
    let mut recorder = LogRecorder::new();
    let data = b"test-entry";

    let sig1 = recorder.sign(data);
    assert!(recorder.verify(data, &sig1));

    recorder.rotate_key();
    assert!(!recorder.verify(data, &sig1), "Old signature should fail after rotation");

    let sig2 = recorder.sign(data);
    assert!(recorder.verify(data, &sig2));
}

#[test]
fn export_flatbuffers_creates_file() {
    let recorder = LogRecorder::new();
    let mut path = std::env::temp_dir();
    path.push("log.fb");
    recorder.export_flatbuffers(&path);
    let data = fs::read(&path).expect("file should exist");
    let pkt = fb::root_as_aitcp_packet(&data).expect("valid flatbuffer");
    assert_eq!(pkt.version(), 1);
    fs::remove_file(&path).ok();
}

#[test]
fn recover_on_corruption_rewrites_file() {
    let mut recorder = LogRecorder::new();
    let mut path = std::env::temp_dir();
    path.push("corrupt.fb");
    recorder.export_flatbuffers(&path);
    fs::write(&path, b"bad").unwrap();
    let old_key = recorder.key().to_vec();
    recorder.recover_on_corruption(&path);
    let data = fs::read(&path).unwrap();
    fb::root_as_aitcp_packet(&data).expect("valid after recovery");
    assert_ne!(recorder.key(), old_key.as_slice());
    fs::remove_file(&path).ok();
}
