// 署名付きパケット送信クライアント
use kairo_lib::packet::AiTcpPacket;
use ed25519_dalek::{SigningKey, Signature, Signer};
use std::fs::File;
use std::io::Read;
use chrono::Utc;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::Path;
use hex;

#[derive(Serialize, Deserialize)]
struct AgentConfig {
    pub p_address: String,
    pub secret_key: String,
    pub public_key: String,
}

fn main() {
    // agent_config.jsonの読み込み
    let path = Path::new("agent_configs/agent_config_1.json");
    let mut file = File::open(path).expect("Failed to open config");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: AgentConfig = serde_json::from_str(&contents).unwrap();

    let payload = "signed hello";
    let payload_bytes = payload.as_bytes();

    let secret_bytes = hex::decode(&config.secret_key).unwrap();
    let signing_key = SigningKey::from_bytes(&secret_bytes.try_into().unwrap());
    let signature: Signature = signing_key.sign(payload_bytes);
    let signature_hex = hex::encode(signature.to_bytes());

    let packet = AiTcpPacket {
        version: 1,
        source_p_address: config.p_address.clone(),
        destination_p_address: "10.0.0.2".to_string(),
        sequence: 1,
        timestamp: Utc::now().timestamp(),
        payload_type: "text".to_string(),
        payload: payload.to_string(),
        signature: signature_hex,
    };

    let client = Client::new();
    let res = client.post("http://127.0.0.1:3030/send")
        .json(&packet)
        .send()
        .expect("Request failed");

    println!("Send result: {}", res.status());
}
