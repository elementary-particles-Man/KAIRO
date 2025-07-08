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

// --- Ed25519 Signature Test ---
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rust_core::signature::{sign_ed25519, verify_ed25519};

#[test]
fn ed25519_signature_verification() {
    let mut csprng = OsRng{};
    let keypair: SigningKey = SigningKey::generate(&mut csprng);
    let message: &[u8] = b"test";

    let signature = sign_ed25519(&keypair, message);
    assert!(verify_ed25519(&keypair.verifying_key(), message, &signature));
}
