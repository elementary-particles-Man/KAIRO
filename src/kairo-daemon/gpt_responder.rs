use serde_json::json;
use chrono::Utc;

pub fn handle_gpt_response(packet: &str) -> String {
    let response = json!({
        "from": "gpt://main",
        "to": "10.0.0.23/24",
        "timestamp": Utc::now().to_rfc3339(),
        "status": "received",
        "echo": packet,
        "signature": "GPT_SIGNATURE_PLACEHOLDER"
    });
    response.to_string()
}