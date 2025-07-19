// signed_sender.rs（署名付きパケット送信＋改ざんテスト用）

use kairo_lib::config::load_agent_config;
use ed25519_dalek::{SigningKey, Signature, Signer};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use clap::Parser;
use reqwest::blocking::Client;
use hex;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    to: String,

    #[arg(long)]
    message: String,

    #[arg(long, default_value_t = false)]
    fake: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct AgentConfig {
    p_address: String,
    public_key: String,
    secret_key: String,
    signature: String,
}

#[derive(Serialize, Debug)]
struct AiTcpPacket {
    source_p_address: String,
    destination_p_address: String,
    payload: String,
    signature: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config_path = PathBuf::from("agent_config.json");
    let config_data = fs::read_to_string(config_path)?;
    let config: AgentConfig = serde_json::from_str(&config_data)?;

    let signing_key_bytes = hex::decode(&config.secret_key)?;
    let signing_key = SigningKey::from_bytes(&signing_key_bytes.try_into()?);

    let actual_payload = if args.fake {
        format!("{}-tampered", args.message) // 故意に改ざん
    } else {
        args.message.clone()
    };

    let signature: Signature = signing_key.sign(actual_payload.as_bytes());
    let signature_hex = hex::encode(signature.to_bytes());

    let packet = AiTcpPacket {
        source_p_address: config.p_address,
        destination_p_address: args.to,
        payload: args.message, // 表示上は正規メッセージ
        signature: signature_hex,
    };

    let client = Client::new();
    let res = client.post("http://127.0.0.1:3030/send")
        .json(&packet)
        .send()?;

    if res.status().is_success() {
        println!("✅ Packet sent successfully.");
    } else {
        println!("❌ Failed to send packet: {}", res.status());
    }

    Ok(())
}