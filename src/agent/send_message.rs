use clap::Parser;
use ed25519_dalek::{Signer, SigningKey};
use kairo_lib::AgentConfig;
use serde_json::json;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
struct Args {
    /// Destination P address
    #[arg(long)]
    to: String,

    /// Message body
    #[arg(long)]
    message: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Load agent configuration
    let mut contents = String::new();
    match File::open("agent_config.json") {
        Ok(mut file) => {
            if file.read_to_string(&mut contents).is_err() {
                eprintln!("Error: failed to read agent_config.json");
                return;
            }
        }
        Err(e) => {
            eprintln!("Error: failed to open agent_config.json: {}", e);
            return;
        }
    }

    let config: AgentConfig = match serde_json::from_str(&contents) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error: failed to parse agent_config.json: {}", e);
            return;
        }
    };

    // Sign the message
    let secret_bytes = match hex::decode(&config.secret_key) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error: invalid secret key hex: {}", e);
            return;
        }
    };

    let signing_key = match secret_bytes[..].try_into() {
        Ok(slice) => SigningKey::from_bytes(&slice),
        Err(_) => {
            eprintln!("Error: secret key size incorrect");
            return;
        }
    };

    let signature = signing_key.sign(args.message.as_bytes());
    let signature_hex = hex::encode(signature.to_bytes());

    // Build request body
    let body = json!({
        "to": args.to,
        "message": args.message,
        "from": config.p_address,
        "signature": signature_hex,
    });

    // Send HTTP POST
    let client = reqwest::Client::new();
    match client
        .post("http://127.0.0.1:3030/send")
        .json(&body)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("Message sent.");
            } else {
                println!("Error: HTTP {}", resp.status());
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
