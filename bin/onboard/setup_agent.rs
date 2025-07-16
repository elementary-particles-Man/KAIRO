//! bin/onboard/setup_agent.rs
//! CUI for first-time onboarding to the KAIRO Mesh.

use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

fn main() {
    println!("--- KAIRO Mesh Initial Setup ---");

    println!("\nStep 1: Generating Static ID (ed25519 Key Pair)...");
    let mut csprng = OsRng;
    let keypair: SigningKey = SigningKey::generate(&mut csprng);
    let public_key: VerifyingKey = (&keypair).into();

    let private_key_hex = hex::encode(keypair.to_bytes());
    let public_key_hex = hex::encode(public_key.as_bytes());
    println!("-> Key Pair generated successfully.");

    println!("\nStep 2: Registering with a Seed Node...");
    register_with_seed_node(&public_key_hex).ok();

    println!("\n--- Onboarding Complete ---");
    println!("Your Mesh Address (Public Key): {}", public_key_hex);
    println!("Your Agent Token (Secret Key): {}", private_key_hex);
    println!("\nIMPORTANT: Keep your Agent Token secure. It will not be shown again.");
    println!("You can now use this token to launch your AI-TCP instance.");
}

// シードノードへの登録を行う関数の雛形
fn register_with_seed_node(public_key: &str) -> Result<(), reqwest::Error> {
    println!("-> Attempting to register public key with seed node...");
    // TODO: The actual seed node URL will be loaded from a config file.
    let seed_node_url = "http://localhost:8080/register";

    let mut a = std::collections::HashMap::new();
    a.insert("agent_id", public_key);

    // reqwest非同期ランタイムのセットアップ
    let client = reqwest::blocking::Client::new();
    let res = client.post(seed_node_url).json(&a).send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("-> Successfully registered with seed node.");
                Ok(())
            } else {
                println!("-> Failed to register. Status: {}", response.status());
                // In a real scenario, we would return a proper error.
                Ok(())
            }
        },
        Err(e) => {
            println!("-> Error connecting to seed node: {}", e);
            Err(e)
        }
    }
}
