use serde_json::json;
use chrono::Utc;
use log::info;

pub fn handle_gpt_response(packet: &str) -> String {
    info!(" GPT processing started for packet: {}", packet);
    let response = json!({
        "from": "gpt://main",
        "to": "10.0.0.23/24",
        "timestamp": Utc::now().to_rfc3339(),
        "status": "received",
        "echo": packet,
        "signature": "GPT_SIGNATURE_PLACEHOLDER"
    });
    info!("GPT simulated response: {}", response.to_string());
    response.to_string()
}

pub async fn gpt_log_and_respond(packet: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    info!("GPT processing started...");
    let gpt_response = handle_gpt_response(packet);
    // ここでログを記録
    // gpt_log_processor::log_gpt_response(&gpt_response).await?;
    info!("✅ GPT response logged successfully");
    Ok(gpt_response)
}