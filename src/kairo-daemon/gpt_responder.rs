use anyhow::{anyhow, Error};
use kairo_lib::packet::Packet;
use log::{error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct GptRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GptResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
}

const DEFAULT_TIMEOUT_SECS: u64 = 10;
const ZERO_COST_ENDPOINT: &str = "https://example.com";

pub async fn gpt_log_and_respond(packet: &Packet) -> Result<(String, SocketAddr), Error> {
    info!(
        "  [GPT_Subsystem] Processing packet seq={} (Zero-Cost Mode)",
        packet.sequence
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
        .build()?;

    let response = client
        .get(ZERO_COST_ENDPOINT)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to connect for remote_addr test: {}", e);
            anyhow!("Failed to connect for remote_addr test: {}", e)
        })?;

    let remote_addr = response.remote_addr().unwrap_or_else(|| {
        warn!("Could not get remote_addr from response, falling back to 0.0.0.0:0");
        "0.0.0.0:0".parse().unwrap()
    });
    info!("  [GPT_Subsystem] Actual remote addr: {}", remote_addr);

    if !response.status().is_success() {
        let status = response.status();
        warn!(
            "Test connection to example.com failed ({}). Still got remote_addr.",
            status
        );
    }

    Ok(("OK (Zero-Cost Response)".to_string(), remote_addr))
}
