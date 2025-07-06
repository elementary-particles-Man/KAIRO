// D:\dev\KAIRO\rust-core\src\signature.rs
use ed25519_dalek::{Keypair, Signer, Verifier, Signature};

pub fn sign(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

pub fn verify(public_key: &ed25519_dalek::PublicKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}