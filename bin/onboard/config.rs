//! bin/onboard/config.rs
//! Handles loading and saving of the agent's identity.

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentConfig {
    pub p_address: String,
    pub public_key: String,
    pub secret_key: String,
}

const CONFIG_FILE: &str = "agent_config.json";

pub fn save_config(config: &AgentConfig) -> Result<(), std::io::Error> {
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(CONFIG_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, config)?;
    println!("-> Agent configuration saved to {}", CONFIG_FILE);
    Ok(())
}

pub fn load_config() -> Option<AgentConfig> {
    if let Ok(file) = File::open(CONFIG_FILE) {
        let reader = BufReader::new(file);
        if let Ok(config) = serde_json::from_reader(reader) {
            println!("-> Agent configuration loaded from {}", CONFIG_FILE);
            return Some(config);
        }
    }
    None
}
