//! src/kairo_daemon/config.rs

use serde::Deserialize;
use std::fs;

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
