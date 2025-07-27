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
    pub p_address: String,
    pub public_key: String,
    pub secret_key: String,
    pub signature: String,
    #[serde(default)]
    pub last_sequence: u64,
}

impl AgentConfig {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let secret_key_bytes: [u8; 32] = { let mut bytes = [0u8; 32]; csprng.fill_bytes(&mut bytes); bytes };
        let signing_key = SigningKey::from_bytes(&secret_key_bytes);
        let verifying_key = VerifyingKey::from(&signing_key);

        AgentConfig {
            p_address: String::new(), // 仮の値
            public_key: hex::encode(verifying_key.to_bytes()),
            secret_key: hex::encode(secret_key_bytes),
            signature: String::new(),
            last_sequence: 0,
        }
    }
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
    Ok(config)
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
