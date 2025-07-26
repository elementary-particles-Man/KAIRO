use rand::thread_rng;
use rand::RngCore;

/// Generate a 256-bit ephemeral key.
pub fn ephemeral_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    thread_rng().fill_bytes(&mut key);
    key
}
