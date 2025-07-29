// signed_sender.rs（署名付きパケット送信＋改ざんテスト用）

use ed25519_dalek::{SigningKey, Signature, Signer};
use std::path::PathBuf;
use clap::Parser;
use reqwest::Client;
use hex;
use kairo_lib::AgentConfig;
use chrono::Utc;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "agent_test")]
    from: String,

    #[arg(long)]
    to: String,

    #[arg(long)]
    message: String,

    #[arg(long, default_value = "./agent_configs")]
    config_dir: String,

    #[arg(long, default_value_t = false)]
    fake: bool,
}

use kairo_lib::packet::AiTcpPacket;

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
    let mut config: AgentConfig = kairo_lib::config::load_agent_config(&agent_config_path.to_string_lossy())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let current_sequence = config.last_sequence + 1;
    config.last_sequence = current_sequence;

    kairo_lib::config::save_agent_config(&config, &agent_config_path.to_string_lossy())
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let signing_key_bytes = hex::decode(&config.secret_key).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let key_bytes: [u8; 32] = signing_key_bytes.try_into()
        .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key length")) as Box<dyn std::error::Error>)?;
    let signing_key = SigningKey::from_bytes(&key_bytes);

    let actual_payload = if args.fake {
        format!("{}-tampered", args.message) // 故意に改ざん
    } else {
        args.message.clone()
    };

    let current_timestamp = Utc::now().timestamp() + 3600;

    let message_to_sign = [
        &current_sequence.to_le_bytes()[..],
        &current_timestamp.to_le_bytes()[..],
        actual_payload.as_bytes(),
    ].concat();

    let signature: Signature = signing_key.sign(&message_to_sign);
    let signature_hex = hex::encode(signature.to_bytes());

    let packet = AiTcpPacket {
        source: config.public_key.clone(),
        destination: args.to.clone(),
        version: 1,
        source_p_address: config.p_address.clone(),
        destination_p_address: args.to.clone(),
        source_public_key: config.public_key.clone(),
        sequence: current_sequence,
        timestamp_utc: current_timestamp as u64,
        payload_type: "message".to_string(),
        payload: args.message.clone(),
        signature: hex::encode(signature.to_bytes()),
    };


    println!("{:#?}", serde_json::to_string(&packet).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?);

    let client = Client::new();

    // Wait for daemon to be ready
    println!("Waiting for KAIRO-P Daemon to be ready...");
    loop {
        match client.get("http://127.0.0.1:3030/").send().await {
            Ok(_) => {
                println!("KAIRO-P Daemon is ready.");
                break;
            },
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }

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
