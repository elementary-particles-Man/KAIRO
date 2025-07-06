// D:\dev\KAIRO\rust-core\tests\signature_verification_test.rs
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use rust_core::signature::{sign, verify};

#[test]
fn test_signature_verification_compiles() {
    // このテストは、ひとまずコンパイルが通ることを確認するダミーテストです。
    // ロジックの正しさは、今後のステップで実装します。
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let message: &[u8] = b"test";
    let signature = sign(&keypair, message);
    assert!(verify(&keypair.public, message, &signature));
}