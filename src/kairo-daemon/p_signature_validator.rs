use sha2::{Digest, Sha256};

/// Minimal validator: recompute canonical SHA256 from nonce+meta and compare to provided sig32.
/// This is intentionally minimal: replace with TPM-backed verification for production.

pub fn validate(sig_nonce: &[u8;16], sig32: &[u8;32], canon: &[u8]) -> bool {
    let mut h = Sha256::new();
    h.update(sig_nonce);
    h.update(canon);
    let out = h.finalize();
    out.as_slice() == &sig32[..]
}
