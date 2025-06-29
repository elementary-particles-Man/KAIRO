use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Clone)]
pub struct Sha256Signature([u8; 32]);

impl Sha256Signature {
    /// Sign the provided message and return the SHA-256 digest as a signature.
    pub fn sign(message: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let result = hasher.finalize();
        let mut sig = [0u8; 32];
        sig.copy_from_slice(&result);
        Sha256Signature(sig)
    }

    /// Verify that the provided signature matches the message.
    pub fn verify(message: &[u8], signature: &Sha256Signature) -> bool {
        Sha256Signature::sign(message) == *signature
    }
}

impl PartialEq for Sha256Signature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for Sha256Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sha256Signature({:x?})", &self.0)
    }
}
