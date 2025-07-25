// OverridePackage logic
use std::collections::HashMap;
use ed25519_dalek::{Signature, VerifyingKey, Verifier};

pub struct OverridePackage {
    pub new_value: String,
    pub signatures: HashMap<String, String>, // p_address -> hex signature
}

pub fn verify_override_package(
    pkg: &OverridePackage,
    quorum_keys: &HashMap<String, String>, // p_address -> pubkey
    payload: &str,
    quorum_threshold: usize,
) -> bool {
    let mut valid = 0;
    for (p_addr, sig_hex) in &pkg.signatures {
        if let Some(pub_hex) = quorum_keys.get(p_addr) {
            let Ok(pub_bytes) = hex::decode(pub_hex) else { continue };
            let Ok(sig_bytes) = hex::decode(sig_hex) else { continue };
            let Ok(pk) = VerifyingKey::from_bytes(&pub_bytes) else { continue };
            let Ok(sig) = Signature::from_bytes(&sig_bytes) else { continue };
            if pk.verify(payload.as_bytes(), &sig).is_ok() {
                valid += 1;
            }
        }
    }
    valid >= quorum_threshold
}
