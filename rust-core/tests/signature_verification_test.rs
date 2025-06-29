use rust_core::signature::Sha256Signature;

#[test]
fn sign_and_verify() {
    let msg = b"hello";
    let sig = Sha256Signature::sign(msg);
    assert!(Sha256Signature::verify(msg, &sig));
}
