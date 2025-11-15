use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};

pub struct Sig {
    pub nonce: [u8; 16],
    pub sig: [u8; 32],
}

pub fn ise_sign(payload: &[u8]) -> Sig {
    let mut nonce = [0; 16];
    OsRng.fill_bytes(&mut nonce);

    let mut hasher = Sha256::new();
    hasher.update(&nonce);
    hasher.update(payload);

    let mut sig = [0u8; 32];
    sig.copy_from_slice(&hasher.finalize());

    Sig { nonce, sig }
}
