use hmac::{Hmac, Mac};
use sha2::Sha256;
use rand::{rngs::OsRng, RngCore};

pub struct PacketSigner {
    key: [u8; 32],
}

impl PacketSigner {
    pub fn new() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self { key }
    }

    pub fn sign(&self, data: &[u8]) -> [u8; 32] {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key).expect("HMAC can take key of any size");
        mac.update(data);
        let result = mac.finalize().into_bytes();
        let mut sig = [0u8; 32];
        sig.copy_from_slice(&result[..32]);
        sig
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key).expect("HMAC can take key of any size");
        mac.update(data);
        mac.verify_slice(signature).is_ok()
    }

    pub fn encrypt(&self, data: &[u8], ephemeral_key: &[u8]) -> Vec<u8> {
        data.iter().zip(ephemeral_key.iter().cycle()).map(|(b,k)| b ^ k).collect()
    }

    pub fn decrypt(&self, data: &[u8], ephemeral_key: &[u8]) -> Vec<u8> {
        self.encrypt(data, ephemeral_key)
    }
}
