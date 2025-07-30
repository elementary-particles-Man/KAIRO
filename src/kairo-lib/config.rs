use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read, Write};


#[derive(Debug, Deserialize)]
pub struct DaemonConfig {
    pub listen_address: String,
    pub listen_port: u16,
}

pub fn load_daemon_config(path: &str) -> Result<DaemonConfig, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: DaemonConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub p_address: String,
    pub public_key: String,
    pub secret_key: String,
    pub signature: String, // Signature of p_address + public_key
}

impl AgentConfig {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let secret_key_bytes: [u8; 32] = { let mut bytes = [0u8; 32]; csprng.fill_bytes(&mut bytes); bytes };
        let signing_key = SigningKey::from_bytes(&secret_key_bytes);
        let verifying_key = VerifyingKey::from(&signing_key);

        AgentConfig {
            p_address: String::new(), // placeholder
            public_key: hex::encode(verifying_key.to_bytes()),
            secret_key: hex::encode(secret_key_bytes),
            signature: String::new(),
        }
    }
}

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

pub fn save_agent_config(config: &AgentConfig, path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(config)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_agent_config(path: &str) -> Result<AgentConfig, std::io::Error> {
    let mut file = File::open(path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let config: AgentConfig = serde_json::from_str(&json)?;
    if verify_signature(&config) {
        println!("-> Agent configuration integrity VERIFIED.");
        Ok(config)
    } else {
        println!("CRITICAL: Agent configuration has been TAMPERED WITH. Loading aborted.");
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid signature"))
    }
}

pub fn load_first_config() -> AgentConfig {
    match load_agent_config("agent_config.json") {
        Ok(config) => config,
        Err(_) => {
            let config = AgentConfig::generate();
            let _ = save_agent_config(&config, "agent_config.json");
            config
        }
    }
}

pub fn load_all_configs() -> Result<Vec<AgentConfig>, Box<dyn std::error::Error>> {
    let mut configs = Vec::new();
    for entry in fs::read_dir("agent_configs")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let config_str = fs::read_to_string(&path)?;
            let config: AgentConfig = serde_json::from_str(&config_str)?;
            configs.push(config);
        }
    }
    Ok(configs)
}

/// Validate the structure of an `AgentConfig`.
/// This performs basic sanity checks on P address and key lengths.
pub fn validate_agent_config(cfg: &AgentConfig) -> Result<(), String> {
    if !cfg.p_address.contains('/') {
        return Err("p_address must contain subnet like '10.0.0.x/24'".to_string());
    }
    if cfg.public_key.len() != 64 || !cfg.public_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("public_key must be 64 hex characters".to_string());
    }
    if cfg.secret_key.len() != 64 || !cfg.secret_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("secret_key must be 64 hex characters".to_string());
    }
    Ok(())
}
