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
    println!("-> Registration request sent (simulated).");

    println!("\n--- Onboarding Complete ---");
    println!("Your Mesh Address (Public Key): {}", public_key_hex);
    println!("Your Agent Token (Secret Key): {}", private_key_hex);
    println!("\nIMPORTANT: Keep your Agent Token secure. It will not be shown again.");
    println!("You can now use this token to launch your AI-TCP instance.");
}
