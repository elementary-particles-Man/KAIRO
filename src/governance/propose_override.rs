//! src/governance/propose_override.rs

use kairo_lib::governance::{OverridePackage, ReissueRequestPayload, SignaturePackage};
use clap::Parser;
use ed25519_dalek::{SigningKey, Signer};
use hex;
use rand::rngs::OsRng;

#[derive(Parser, Debug)]
#[command(about = "Constructs and sends a governance OverridePackage to the seed node.")]
struct Args {
    #[arg(long)]
    old_agent_id: String,

    #[arg(long)]
    new_agent_id: String,

    #[arg(long, default_value = "Key lost, emergency recovery required.")]
    reason: String,

    #[arg(long)]
    invalid_quorum: bool,
}

use reqwest::Client;
use tokio::main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let payload = ReissueRequestPayload {
        old_agent_id: args.old_agent_id,
        new_agent_id: args.new_agent_id,
        reason: args.reason,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let payload_json = serde_json::to_string(&payload).expect("Failed to serialize payload");

    // Simulated secret keys for demonstration purposes
    let seednode_secret_key_bytes: [u8; 32] = [1; 32]; // Example secret key
    let peera_secret_key_bytes: [u8; 32] = [2; 32]; // Example secret key
    let auditor_secret_key_bytes: [u8; 32] = [3; 32]; // Example secret key

    let seednode_signing_key = SigningKey::from_bytes(&seednode_secret_key_bytes);
    let peera_signing_key = SigningKey::from_bytes(&peera_secret_key_bytes);
    let auditor_signing_key = SigningKey::from_bytes(&auditor_secret_key_bytes);

    let seednode_signature = seednode_signing_key.sign(payload_json.as_bytes());
    let peera_signature = peera_signing_key.sign(payload_json.as_bytes());
    let auditor_signature = auditor_signing_key.sign(payload_json.as_bytes());

    let mut signatures = vec![
        SignaturePackage {
            signatory_id: "seednode-01".to_string(),
            signatory_role: "SeedNode".to_string(),
            signature: hex::encode(seednode_signature.to_bytes()),
        },
        SignaturePackage {
            signatory_id: "peera-alpha".to_string(),
            signatory_role: "PeerAI".to_string(),
            signature: hex::encode(peera_signature.to_bytes()),
        },
    ];

    if !args.invalid_quorum {
        signatures.push(SignaturePackage {
            signatory_id: "auditor-human-01".to_string(),
            signatory_role: "HumanAuditor".to_string(),
            signature: hex::encode(auditor_signature.to_bytes()),
        });
    } else {
        // Provide an invalid signature if --invalid-quorum is set
        signatures.push(SignaturePackage {
            signatory_id: "auditor-human-01".to_string(),
            signatory_role: "HumanAuditor".to_string(),
            signature: "invalid_signature".to_string(), // Intentionally invalid
        });
    }

    let package = OverridePackage {
        payload,
        signatures,
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8000/emergency_reissue")
        .json(&package)
        .send()
        .await;

    match res {
        Ok(response) => {
            println!("-> Sent OverridePackage. Server response: {}", response.status());
            println!("{}", response.text().await.unwrap_or_default());
        }
        Err(e) => eprintln!("-> Failed to send OverridePackage: {}", e),
    }
    Ok(())
}
