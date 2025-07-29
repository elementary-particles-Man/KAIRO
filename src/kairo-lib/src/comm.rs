use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, Signer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub payload: String,
    pub signature: String,
}

pub fn sign_message(message: &Message, private_key: &str) -> Result<Message, String> {
    let signing_key_bytes = hex::decode(private_key).map_err(|e| format!("Invalid private key format: {}", e))?;
    let key_bytes: [u8; 32] = signing_key_bytes.try_into().map_err(|_| "Invalid private key length".to_string())?;
    let signing_key = SigningKey::from_bytes(&key_bytes);

    let message_bytes = serde_json::to_string(&message).map_err(|e| format!("Failed to serialize message: {}", e))?;
    let signature = signing_key.sign(message_bytes.as_bytes());

    let mut signed_message = message.clone();
    signed_message.signature = hex::encode(signature.to_bytes());

    Ok(signed_message)
}
