use clap::Parser;
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use kairo_lib::AgentConfig;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// P address to fetch messages for
    #[arg(long, value_name = "P_ADDRESS")]
    for_address: String,

    #[arg(long)]
    from: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    _version: u32,
    source_public_key: String,
    _destination_p_address: String,
    _sequence: u64,
    _timestamp: i64,
    _payload_type: String,
    payload: String,
    signature: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let agent_config_path = PathBuf::from(format!("agent_configs/{}.json", args.from.replace("/", "_")));
    let config_data = match fs::read_to_string(agent_config_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read agent_config.json: {}", e);
            return;
        }
    };
    let _config: AgentConfig = match serde_json::from_str(&config_data) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse agent_config.json: {}", e);
            return;
        }
    };

    // Fetch messages from daemon
    let url = format!("http://127.0.0.1:3030/receive?for={}", args.for_address);
    let client = reqwest::Client::new();
    let response = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to contact daemon: {}", e);
            return;
        }
    };

    if !response.status().is_success() {
        eprintln!("Daemon returned HTTP {}", response.status());
        return;
    }

    let messages: Vec<Message> = match response.json().await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to parse response JSON: {}", e);
            return;
        }
    };

    if messages.is_empty() {
        println!("No messages found.");
        return;
    }

    for msg in messages {
        let public_key_bytes = match hex::decode(&msg.source_public_key) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Invalid source public key hex in message: {}", e);
                continue;
            }
        };

        let verifying_key = match public_key_bytes.as_slice().try_into() {
            Ok(bytes) => match VerifyingKey::from_bytes(bytes) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("Invalid source public key in message: {}", e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Source public key bytes in message not 32 bytes long: {}", e);
                continue;
            }
        };

        let signature_bytes = match hex::decode(&msg.signature) {
            Ok(b) => b,
            Err(_) => {
                println!("From {}: invalid signature encoding", msg.source_public_key);
                continue;
            }
        };

        let signature = match Signature::try_from(signature_bytes.as_slice()) {
            Ok(sig) => sig,
            Err(_) => {
                println!("From {}: invalid signature format", msg.source_public_key);
                continue;
            }
        };

        if verifying_key.verify(msg.payload.as_bytes(), &signature).is_ok() {
            println!("From {}: {} (signature OK)", msg.source_public_key, msg.payload);
        } else {
            println!("From {}: {} (signature INVALID)", msg.source_public_key, msg.payload);
        }
    }
}

