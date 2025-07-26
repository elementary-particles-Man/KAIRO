use kairo_lib::{save_agent_config};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use reqwest;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use kairo_lib::config as daemon_config;
use kairo_lib::AgentConfig;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct CliArgs {
    #[arg(long)]
    new: bool,
    #[arg(long)]
    name: String,
}

fn get_daemon_assign_url() -> String {
    let config = daemon_config::load_daemon_config(".kairo/config/daemon_config.json")
    .unwrap_or_else(|_| {
        println!("WARN: daemon_config.json not found or invalid. Falling back to default bootstrap address.");
        daemon_config::DaemonConfig {
            listen_address: "127.0.0.1".to_string(),
            listen_port: 3030
        }
    });
    format!("http://{}:{}/assign_p_address", config.listen_address, config.listen_port)
}


#[derive(Deserialize)]
struct AgentMapping {
    p_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = CliArgs::parse();

    let agent_config_dir = PathBuf::from("agent_configs");
    fs::create_dir_all(&agent_config_dir)?;

    let agent_config_file_name = format!("{}.json", cli_args.name);
    let agent_config_path = agent_config_dir.join(&agent_config_file_name);

    let mut config: AgentConfig;

    if cli_args.new || !agent_config_path.exists() {
        println!("--- KAIRO Mesh Initial Setup ---");

        // Generate key pair
let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let public_key_bytes = signing_key.verifying_key().to_bytes().to_vec();
        let secret_key_bytes = signing_key.to_bytes().to_vec();

        let public_key_hex = hex::encode(&public_key_bytes);
        let secret_key_hex = hex::encode(&secret_key_bytes);

        println!("Secret Key: \"{}\"", secret_key_hex);
        println!("Public Key: \"{}\"", public_key_hex);

        println!("\nStep 2: Registering with a Seed Node...");

        // Try to request P address from local daemon
        println!("
Requesting KAIRO-P address from local daemon...");

        // Change reqwest::get to reqwest::Client::new().post
        let p_address_response = reqwest::Client::new()
            .post(get_daemon_assign_url()) // Changed endpoint
            .json(&serde_json::json!({ "public_key": public_key_hex })) // Changed to public_key
            .send()
            .await?; // エラーハンドリングを簡略化

        let p_address_mapping: AgentMapping = p_address_response.json().await?;
        let p_address = p_address_mapping.p_address; // Pアドレスを取得

        println!("-> Received P Address: {}", p_address);

        println!("-> Attempting to register public key with seed node...");
        let _ = reqwest::Client::new()
            .post("http://127.0.0.1:8000/register")
            .json(&serde_json::json!({
                "agent_id": public_key_hex,
                "p_address": p_address,
            }))
            .send()
            .await?;

        println!("-> Successfully registered with seed node.");

        config = AgentConfig {
            p_address: p_address.clone(),
            public_key: public_key_hex.clone(),
            secret_key: secret_key_hex,
            signature: String::new(), // will be set below
        };

        // Save the new agent config to the specified path
        save_agent_config(&config, agent_config_path.to_str().unwrap())?;

        println!("--- Onboarding Complete ---");
    } else {
        println!("--- Welcome Back ---");
        let contents = fs::read_to_string(agent_config_path)?;
        config = serde_json::from_str(&contents)?;
        println!("Restored identity from agent_config.json");
        println!("Your Public Key: {}", config.public_key);
    }

    Ok(())
}
