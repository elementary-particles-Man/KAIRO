use ed25519_dalek::{SigningKey, VerifyingKey};
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentConfig {
    pub signing_key_bytes: [u8; 32],
    pub verifying_key_bytes: [u8; 32],
}

impl AgentConfig {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let mut secret_bytes = [0u8; 32];
        csprng.fill_bytes(&mut secret_bytes);

        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let verifying_key = VerifyingKey::from(&signing_key);

        AgentConfig {
            signing_key_bytes: signing_key.to_bytes(),
            verifying_key_bytes: verifying_key.to_bytes(),
        }
    }
}

pub fn save_config(config: &AgentConfig) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(config)?;
    let mut file = File::create("agent_config.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_config(path: &str) -> Result<AgentConfig, std::io::Error> {
    let mut file = File::open(path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let config: AgentConfig = serde_json::from_str(&json)?;
    Ok(config)
}

pub fn load_first_config() -> AgentConfig {
    match load_config("agent_config.json") {
        Ok(config) => config,
        Err(_) => {
            let config = AgentConfig::generate();
            let _ = save_config(&config);
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
