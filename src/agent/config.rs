//! src/agent/config.rs
//! Handles loading and saving of agent identities.

use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub p_address: String,
    pub public_key: String,
    pub secret_key: String,
    pub signature: String, // Signature of p_address + public_key
}

const CONFIG_DIR: &str = "agent_configs";

// Creates a signature for the config file to ensure its integrity.
pub fn create_signature(p_address: &str, public_key: &str, secret_key: &SigningKey) -> String {
    let message = format!("{}:{}", p_address, public_key);
    let signature = secret_key.sign(message.as_bytes());
    hex::encode(signature.to_bytes())
}

// Verifies the signature within the config file.
pub fn verify_signature(config: &AgentConfig) -> bool {
    let public_key_bytes = match hex::decode(&config.public_key) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => return false,
    };
    let signature_bytes = match hex::decode(&config.signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let message = format!("{}:{}", config.p_address, config.public_key);
    public_key.verify(message.as_bytes(), &signature).is_ok()
}

pub fn save_config(config: &AgentConfig) -> Result<(), std::io::Error> {
    fs::create_dir_all(CONFIG_DIR)?;
    let config_path = Path::new(CONFIG_DIR).join(format!("{}.json", config.p_address));
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(config_path)?;
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
                if let Ok(config): Result<AgentConfig, _> = serde_json::from_reader(reader) {
                    if verify_signature(&config) {
                        println!("-> Agent configuration integrity VERIFIED.");
                        return Some(config);
                    } else {
                        println!("CRITICAL: Agent configuration has been TAMPERED WITH. Loading aborted.");
                        return None;
                    }
                }
            }
        }
    }
    None
}
