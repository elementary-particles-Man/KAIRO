//! src/agent/config.rs
//! Handles loading and saving of agent identities.

use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use ed25519_dalek::{SigningKey, Signature, Signer};
use hex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub p_address: String,
    pub public_key: String,
    pub secret_key: String,
}

const CONFIG_DIR: &str = "agent_configs";

pub fn save_config(config: &AgentConfig) -> Result<(), std::io::Error> {
    fs::create_dir_all(CONFIG_DIR)?;
    let config_path = Path::new(CONFIG_DIR).join(format!("{}.json", config.p_address));
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(config_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, config)?;
    println!("-> Agent configuration saved.");
    Ok(())
}

/// Loads the first available agent config.
/// In a real application multiple identities would be handled.
pub fn load_first_config() -> Option<AgentConfig> {
    if let Ok(entries) = fs::read_dir(CONFIG_DIR) {
        for entry in entries.flatten() {
            if let Ok(file) = File::open(entry.path()) {
                let reader = BufReader::new(file);
                if let Ok(config) = serde_json::from_reader(reader) {
                    println!("-> Loaded first available agent configuration.");
                    return Some(config);
                }
            }
        }
    }
    None
}

/// Creates a hex-encoded signature from the given secret key and payload.
/// Returns `Some(signature)` if successful, or `None` on error.
pub fn create_signature(secret_key_hex: &str, payload: &[u8]) -> Option<String> {
    let secret_bytes = hex::decode(secret_key_hex).ok()?;
    let signing_key = SigningKey::from_bytes(&secret_bytes.try_into().ok()?).ok()?;
    let signature: Signature = signing_key.sign(payload);
    Some(hex::encode(signature.to_bytes()))
}
