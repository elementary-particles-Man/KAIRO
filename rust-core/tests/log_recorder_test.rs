use rust_core::log_recorder::LogRecorder;

#[test]
fn hmac_rotation_verification() {
    let initial_key = vec![1u8; 32];
    let mut recorder = LogRecorder::new(initial_key.clone());
    let data = b"test-entry";

    let sig1 = recorder.sign(data);
    assert!(recorder.verify(data, &sig1));

    recorder.rotate_key_if_needed();
    assert!(!recorder.verify(data, &sig1), "Old signature should fail after rotation");

    let sig2 = recorder.sign(data);
    assert!(recorder.verify(data, &sig2));
}
