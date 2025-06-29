use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn sign(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

pub fn verify(data: &[u8], key: &[u8], signature: &[u8]) -> bool {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.verify_slice(signature).is_ok()
}
