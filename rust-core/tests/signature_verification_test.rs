use rust_core::signature::{sign, verify};

#[test]
fn verifies_valid_signature() {
    let key = b"supersecret";
    let message = b"hello";
    let sig = sign(message, key);
    assert!(verify(message, key, &sig));
    let wrong_sig = sign(b"bad", key);
    assert!(!verify(message, key, &wrong_sig));
}
