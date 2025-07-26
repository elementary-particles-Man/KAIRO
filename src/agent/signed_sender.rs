// signed_sender.rs（署名付きパケット送信＋改ざんテスト用）

use ed25519_dalek::{SigningKey, Signature, Signer};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use reqwest::Client;
use hex;
use kairo_lib::AgentConfig;
use chrono::Utc;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    from: String,
    #[arg(long)]
    to: String,

    #[arg(long)]
    message: String,

    #[arg(long, default_value_t = false)]
    fake: bool,
}

#[derive(Serialize, Debug)]
struct AiTcpPacket {
    source_public_key: String,
    destination_p_address: String,
    payload: String,
    signature: String,
}

use kairo_lib::config as daemon_config;

fn get_daemon_url() -> String {
    let config = daemon_config::load_daemon_config("daemon_config.json")
        .unwrap_or_else(|_| {
            println!("WARN: daemon_config.json not found or invalid. Falling back to default bootstrap address.");
            daemon_config::DaemonConfig {
                listen_address: "127.0.0.1".to_string(),
                listen_port: 3030,
            }
        });

    format!("http://{}:{}/send", config.listen_address, config.listen_port)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let agent_config_path = PathBuf::from(format!("agent_configs/{}.json", args.from.replace("/", "_")));
    let config_data = fs::read_to_string(agent_config_path)?;
    let config: AgentConfig = serde_json::from_str(&config_data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let signing_key_bytes = hex::decode(&config.secret_key).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let key_bytes: [u8; 32] = signing_key_bytes.try_into()
        .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key length")) as Box<dyn std::error::Error>)?;
    let signing_key = SigningKey::from_bytes(&key_bytes);

    let actual_payload = if args.fake {
        format!("{}-tampered", args.message) // 故意に改ざん
    } else {
        args.message.clone()
    };

    let signature: Signature = signing_key.sign(actual_payload.as_bytes());
    let signature_hex = hex::encode(signature.to_bytes());

    let packet = AiTcpPacket {
        source_public_key: config.public_key.clone(),
        destination_p_address: args.to,
        payload: args.message, // 表示上は正規メッセージ
        signature: signature_hex,
    };

    println!("{:#?}", serde_json::to_string(&packet).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?);

    let client = Client::new();
    let res = client.post(get_daemon_url())
        .json(&packet)
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    if res.status().is_success() {
        println!("✅ Packet sent successfully.");
        Ok(())
    } else {
        println!("❌ Failed to send packet: {}", res.status());
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to send packet: {}", res.status()))) as Box<dyn std::error::Error>)
    }
}
