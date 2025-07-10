// ===========================
// 📄 rust-core/tests/signature_verification_test.rs
// ===========================

// --- SHA256 Signature Test ---
use rust_core::signature::Sha256Signature;

#[test]
fn sha256_sign_and_verify() {
    let msg = b"hello";
    let sig = Sha256Signature::sign(msg);
    assert!(Sha256Signature::verify(msg, &sig));
}

#[test]
fn sha256_verify_with_wrong_message() {
    let msg = b"hello";
    let wrong_msg = b"world";
    let sig = Sha256Signature::sign(msg);
    assert!(!Sha256Signature::verify(wrong_msg, &sig));
}

#[test]
fn sha256_verify_with_wrong_signature() {
    let msg = b"hello";
    let mut sig = Sha256Signature::sign(msg);
    sig.0[0] ^= 0xFF; // Tamper with the signature
    assert!(!Sha256Signature::verify(msg, &sig));
}

// --- Ed25519 Signature Test ---
use ed25519_dalek::{SigningKey, VerifyingKey, Signature};
use rust_core::keygen::ephemeral_key;
use rust_core::signature::{sign_ed25519, verify_ed25519};

#[test]
fn ed25519_signature_verification() {
    let signing_key = SigningKey::from_bytes(&ephemeral_key());
    let message: &[u8] = b"test";

    let signature = sign_ed25519(&signing_key, message);
    assert!(verify_ed25519(&signing_key.verifying_key(), message, &signature).is_ok());
}

#[test]
fn ed25519_verify_with_wrong_message() {
    let signing_key = SigningKey::from_bytes(&ephemeral_key());
    let message: &[u8] = b"test";
    let wrong_message: &[u8] = b"wrong";

    let signature = sign_ed25519(&signing_key, message);
    assert!(verify_ed25519(&signing_key.verifying_key(), wrong_message, &signature).is_err());
}

#[test]
fn ed25519_verify_with_wrong_signature() {
    let signing_key = SigningKey::from_bytes(&ephemeral_key());
    let message: &[u8] = b"test";

    let signature = sign_ed25519(&signing_key, message);
    let mut bytes = signature.to_bytes();
    bytes[0] ^= 0xFF; // Tamper with the signature
    let tampered = Signature::from_bytes(&bytes);
    assert!(verify_ed25519(&signing_key.verifying_key(), message, &tampered).is_err());
}
