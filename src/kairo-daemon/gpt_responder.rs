use anyhow::anyhow;
use kairo_lib::packet::Packet;
use log::{error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
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

const GPT_MODEL: &str = "gpt-4o-2024-05-13";
const DEFAULT_TIMEOUT_SECS: u64 = 60;

pub async fn gpt_log_and_respond(packet: &Packet) -> Result<(String, SocketAddr), anyhow::Error> {
    info!(
        "  [GPT_Subsystem] Processing packet from {}",
        packet.source_p_address
    );

    let api_key = env::var("OPENAI_API_KEY").map_err(|e| {
        error!("OPENAI_API_KEY not set: {}", e);
        anyhow!("OPENAI_API_KEY not set: {}", e)
    })?;

    let client = Client::builder()
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
        .build()?;

    let request_payload = GptRequest {
        model: GPT_MODEL.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: packet.payload.clone(),
        }],
        temperature: 0.5,
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_payload)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send to OpenAI: {}", e);
            anyhow!("Failed to send to OpenAI: {}", e)
        })?;

    let remote_addr = response.remote_addr().unwrap_or_else(|| {
        warn!("Could not get remote_addr from response, falling back to 0.0.0.0:0");
        "0.0.0.0:0".parse().unwrap()
    });
    info!("  [GPT_Subsystem] Actual remote addr: {}", remote_addr);

    if !response.status().is_success() {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "<no body>".to_string());
        error!("OpenAI API error ({}): {}", status, text);
        return Err(anyhow!("OpenAI API error ({}): {}", status, text));
    }

    let gpt_response = response.json::<GptResponse>().await.map_err(|e| {
        error!("Failed to parse OpenAI response: {}", e);
        anyhow!("Failed to parse OpenAI response: {}", e)
    })?;

    let resp_text = gpt_response
        .choices
        .get(0)
        .map_or_else(
            || {
                error!("No choices returned from OpenAI");
                "[Error: No choices returned]".to_string()
            },
            |choice| choice.message.content.clone(),
        );

    Ok((resp_text, remote_addr))
}
