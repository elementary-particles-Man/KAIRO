use ed25519_dalek::{SigningKey, VerifyingKey, Keypair};
use rust_core::keygen::ephemeral_key;
use rust_core::signature::{sign_ed25519, verify_ed25519};

#[test]
fn ephemeral_key_signature_consistency() {
    let sk_bytes = ephemeral_key();
    let signing = SigningKey::from_bytes(&sk_bytes);
    let verifying = VerifyingKey::from(&signing);
    let keypair = Keypair{ secret: signing, public: verifying };
    let msg = b"integration-test";
    let sig = sign_ed25519(&keypair, msg);
    assert!(verify_ed25519(&keypair.public, msg, &sig));
}
