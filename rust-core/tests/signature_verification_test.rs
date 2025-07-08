// ===========================
// 📄 rust-core/tests/signature_verification_test.rs
// ===========================

// --- SHA256 Signature Test ---
use crate::signature::Sha256Signature;

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
    sig.signature[0] = sig.signature[0].wrapping_add(1); // Tamper with the signature
    assert!(!Sha256Signature::verify(msg, &sig));
}

// --- Ed25519 Signature Test ---
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use crate::signature::{sign_ed25519, verify_ed25519};
use ed25519_dalek::Signer;

#[test]
fn ed25519_signature_verification() {
    let mut csprng = OsRng{};
    let signing_key = SigningKey::generate(&mut csprng);
    let message: &[u8] = b"test";

    let signature = signing_key.sign(message);
    assert!(verify_ed25519(&signing_key.verifying_key(), message, &signature).is_ok());
}

#[test]
fn ed25519_verify_with_wrong_message() {
    let mut csprng = OsRng{};
    let signing_key = SigningKey::generate(&mut csprng);
    let message: &[u8] = b"test";
    let wrong_message: &[u8] = b"wrong";

    let signature = signing_key.sign(message);
    assert!(verify_ed25519(&signing_key.verifying_key(), wrong_message, &signature).is_err());
}

#[test]
fn ed25519_verify_with_wrong_signature() {
    let mut csprng = OsRng{};
    let signing_key = SigningKey::generate(&mut csprng);
    let message: &[u8] = b"test";

    let mut signature = signing_key.sign(message);
    signature.0[0] = signature.0[0].wrapping_add(1); // Tamper with the signature
    assert!(verify_ed25519(&signing_key.verifying_key(), message, &signature).is_err());
}
