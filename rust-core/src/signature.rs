// ===========================
// 📄 rust-core/src/signature.rs
// ===========================

// --- SHA-256 シグネチャ実装 ---
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Clone)]
pub struct Sha256Signature([u8; 32]);

impl Sha256Signature {
    /// SHA-256 でメッセージを署名（ダイジェスト作成）
    pub fn sign(message: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let result = hasher.finalize();
        let mut sig = [0u8; 32];
        sig.copy_from_slice(&result);
        Sha256Signature(sig)
    }

    /// SHA-256 シグネチャが一致するか検証
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

// --- Ed25519 シグネチャ実装 ---
use ed25519_dalek::{Keypair, PublicKey, Signer, Verifier, Signature as Ed25519Signature};

/// Ed25519 署名
pub fn sign(keypair: &Keypair, message: &[u8]) -> Ed25519Signature {
    keypair.sign(message)
}

/// Ed25519 検証
pub fn verify(public_key: &PublicKey, message: &[u8], signature: &Ed25519Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}
