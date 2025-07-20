//! src/governance/sign_override.rs

use clap::Parser;
use ed25519_dalek::{SigningKey, Signer};
use hex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use kairo_lib::governance::{OverridePackage, ReissueRequestPayload, SignaturePackage};

#[derive(Parser, Debug)]
#[command(about = "Signs an OverridePackage payload using a specified agent's secret key.")]
struct Args {
    #[arg(long)]
    user_dir: PathBuf,
    #[arg(long)]
    override_file_path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AgentConfig {
    p_address: String,
    public_key: String,
    secret_key: String,
    signature: String,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load agent_config.json
    let agent_config_path = args.user_dir.join("agent_config.json");
    let config_data = fs::read_to_string(&agent_config_path)
        .map_err(|e| format!("Failed to read agent_config.json from {}: {}", agent_config_path.display(), e))?;
    let config: AgentConfig = serde_json::from_str(&config_data)
        .map_err(|e| format!("Failed to parse agent_config.json: {}", e))?;

    // Load OverridePackage from file
    let override_file_content = fs::read_to_string(&args.override_file_path)
        .map_err(|e| format!("Failed to read override.json from {}: {}", args.override_file_path.display(), e))?;
    let mut override_package: OverridePackage = serde_json::from_str(&override_file_content)
        .map_err(|e| format!("Failed to parse override.json: {}", e))?;

    // Prepare signing key
    let signing_key_bytes = hex::decode(&config.secret_key)
        .map_err(|e| format!("Failed to decode secret key: {}", e))?;
    let key_bytes: [u8; 32] = signing_key_bytes.try_into()
        .map_err(|_| "Invalid secret key length")?;
    let signing_key = SigningKey::from_bytes(&key_bytes);

    // Sign the payload
    let payload_json = serde_json::to_string(&override_package.payload).expect("Failed to serialize payload");
    let signature = signing_key.sign(payload_json.as_bytes());
    let signature_hex = hex::encode(signature.to_bytes());

    // Create SignaturePackage
    let own_signature = SignaturePackage {
        signatory_id: config.public_key, // Using public_key as signatory_id
        signatory_role: "Agent".to_string(), // Role can be generalized or passed as arg
        signature: signature_hex,
    };

    // Add the new signature to the OverridePackage
    override_package.signatures.push(own_signature);

    // Save the updated OverridePackage back to the file
    let updated_json = serde_json::to_string_pretty(&override_package)?;
    fs::write(&args.override_file_path, updated_json)?;
    println!("✔️ Updated override.json with new signature.");

    Ok(())
}
