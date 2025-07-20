use kairo_lib::{AgentConfig, save_agent_config, sign_config};
use p256::ecdsa::SigningKey;
use rand_core::OsRng;
use reqwest;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use kairo_lib::config as daemon_config;

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
    public_key: String,
    p_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let create_new = args.contains(&"--new".to_string());

    let agent_config_path = PathBuf::from("agent_config.json");
    let mut config: AgentConfig;

    if create_new || !agent_config_path.exists() {
        println!("--- KAIRO Mesh Initial Setup ---");

        // Generate key pair
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let secret_key_bytes = signing_key.to_bytes();
        let public_key_bytes = verifying_key.to_encoded_point(false).as_bytes().to_vec();

        let public_key_hex = hex::encode(&public_key_bytes);
        let secret_key_hex = hex::encode(&secret_key_bytes);

        println!("Secret Key: \"{}\"", secret_key_hex);
        println!("Public Key: \"{}\"", public_key_hex);

        println!("\nStep 2: Registering with a Seed Node...");

        // Try to request P address from local daemon
        let mut p_address = String::from("invalid_address");
        println!("
Requesting KAIRO-P address from local daemon...");

        // Change reqwest::get to reqwest::Client::new().post
        match reqwest::Client::new()
            .post(get_daemon_assign_url()) // Changed endpoint
            .json(&serde_json::json!({ "public_key": public_key_hex })) // Changed to public_key
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<AgentMapping>().await {
                        Ok(mapping) => {
                            println!("-> Received P Address: {}", mapping.p_address);
                            p_address = mapping.p_address;
                        }
                        Err(e) => {
                            println!("-> Failed to parse JSON response: {}", e);
                        }
                    }
                } else {
                    println!("-> Failed to assign P address. Status: {}", response.status());
                }
            }
            Err(e) => {
                println!("-> Failed to contact local daemon: {}", e);
            }
        }

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
        sign_config(&mut config);
        save_agent_config(&config)?;
        println!("-> Agent configuration saved.");

        println!("\n--- Onboarding Complete ---");
        println!("Your assigned KAIRO-P Address: {}", config.p_address);
    } else {
        println!("--- Welcome Back ---");
        let contents = fs::read_to_string(agent_config_path)?;
        config = serde_json::from_str(&contents)?;
        println!("Restored identity from agent_config.json");
        println!("Your KAIRO-P Address: {}", config.p_address);
        println!("Your Public Key: {}", config.public_key);
    }

    Ok(())
}
