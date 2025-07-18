//! KAIRO/bin/onboard/setup_agent.rs

use ed25519_dalek::{SigningKey, SECRET_KEY_LENGTH};
use rand_core::OsRng;
use rand_core::RngCore;

pub mod config;
use config::{load_config, save_config, AgentConfig};

fn main() {
    if let Some(config) = load_config() {
        // If config was loaded, attempt registration with the existing public key
        register_with_seed_node(&config.public_key);
        println!("\n--- Welcome Back ---");
        println!("Restored identity from agent_config.json");
        println!("Your KAIRO-P Address: {}", config.p_address);
        println!("Your Public Key: {}", config.public_key);
        // In a real app, you would now proceed with this identity.
        return;
    }
    println!("--- KAIRO Mesh Initial Setup ---");

    let mut csprng = OsRng;

    let mut secret_bytes = [0u8; SECRET_KEY_LENGTH];
    csprng.fill_bytes(&mut secret_bytes);

    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();

    let private_key_hex = hex::encode(signing_key.to_bytes());
    let public_key_hex = hex::encode(verifying_key.to_bytes());

    println!("Secret Key: {:?}", private_key_hex);
    println!("Public Key: {:?}", public_key_hex);

    println!("\nStep 2: Registering with a Seed Node...");

    // This section is now only for new agents
    let p_address = request_p_address();
    let config = AgentConfig {
        p_address: p_address.clone(),
        public_key: public_key_hex,
        secret_key: private_key_hex,
    };
    // Always attempt to register the persistent ID with the seed node
    register_with_seed_node(&config.public_key);
    save_config(&config).expect("Failed to save agent configuration.");
    println!("\n--- Onboarding Complete ---");
    println!("Your assigned KAIRO-P Address: {}", p_address);
}

// シードノードへの登録を行う関数
fn register_with_seed_node(public_key: &str) -> Result<(), reqwest::Error> {
    println!("-> Attempting to register public key with seed node...");
    let seed_node_url = "http://localhost:8080/register";

    let mut payload = std::collections::HashMap::new();
    payload.insert("agent_id", public_key);

    let client = reqwest::blocking::Client::new();
    let res = client.post(seed_node_url).json(&payload).send()?;

    if res.status().is_success() {
        println!("-> Successfully registered with seed node.");
    } else {
        println!("-> Failed to register. Status: {}", res.status());
    }

    Ok(())
}

fn request_p_address() -> String {
    println!("\nRequesting KAIRO-P address from local daemon...");
    let client = reqwest::blocking::Client::new();
    match client.post("http://localhost:3030/request_address").send() {
        Ok(res) => {
            let addr = res.json::<String>().unwrap_or_else(|_| "error".to_string());
            println!("-> KAIRO-P Address assigned: {}", addr);
            addr
        },
        Err(e) => {
            println!("-> Failed to connect to KAIRO-P daemon: {}. Is it running?", e);
            "failed_to_connect".to_string()
        }
    }
}
