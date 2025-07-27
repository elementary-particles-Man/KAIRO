// forged_sender.rs - Send signed packet with spoofed P address for testing

use kairo_lib::packet::sign_packet;
use kairo_lib::AgentConfig as LibAgentConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use reqwest::blocking::Client;
use hex;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Destination P address
    #[arg(long)]
    to: String,

    /// Message body
    #[arg(long)]
    message: String,

    /// Spoofed source P address
    #[arg(long)]
    from: String,
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
    version: u32,
    source_p_address: String,
    destination_p_address: String,
    sequence: u64,
    timestamp: i64,
    payload_type: String,
    payload: String,
    signature: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config_path = PathBuf::from("agent_config.json");
    let config_data = fs::read_to_string(config_path)?;
    let config: AgentConfig = serde_json::from_str(&config_data)?;

    println!("Loaded AgentConfig: {:?}", config);

    let signature_hex = sign_packet(&LibAgentConfig {
        p_address: config.p_address.clone(),
        public_key: config.public_key.clone(),
        secret_key: config.secret_key.clone(),
        signature: config.signature.clone(),
    }, 1, chrono::Utc::now().timestamp(), &args.message)?;

    let packet = AiTcpPacket {
        version: 1,
        source_p_address: args.from,
        destination_p_address: args.to,
        sequence: 1, // 仮の値
        timestamp: chrono::Utc::now().timestamp(),
        payload_type: "text/plain".to_string(),
        payload: args.message,
        signature: signature_hex,
    };

    println!("{:#?}", serde_json::to_string(&packet)?);

    let client = Client::new();
    let res = client
        .post("http://127.0.0.1:3030/send")
        .json(&packet)
        .send()?;

    if res.status().is_success() {
        println!("✅ Packet sent successfully.");
    } else {
        println!("❌ Failed to send packet: {}", res.status());
    }

    Ok(())
}

