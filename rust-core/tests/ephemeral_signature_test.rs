use ed25519_dalek::{SigningKey, VerifyingKey};
use crate::keygen::ephemeral_key;
use crate::signature::{sign_ed25519, verify_ed25519};

#[test]
fn ephemeral_key_signature_consistency() {
    let sk_bytes = ephemeral_key();
    let signing = SigningKey::from_bytes(&sk_bytes);
    let verifying = VerifyingKey::from(&signing);
    let msg = b"integration-test";
    let sig = sign_ed25519(&signing, msg);
    assert!(verify_ed25519(&verifying, msg, &sig).is_ok());
}
