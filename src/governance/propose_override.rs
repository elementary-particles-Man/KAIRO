//! src/governance/propose_override.rs

use kairo_lib::governance::{OverridePackage, ReissueRequestPayload, SignaturePackage};
use clap::Parser;

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

fn main() {
    let args = Args::parse();

    let payload = ReissueRequestPayload {
        old_agent_id: args.old_agent_id,
        new_agent_id: args.new_agent_id,
        reason: args.reason,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let mut signatures = vec![
        SignaturePackage {
            signatory_id: "seednode-01".to_string(),
            signatory_role: "SeedNode".to_string(),
            signature: "(simulated_seed_node_sig)".to_string(),
        },
        SignaturePackage {
            signatory_id: "peera-alpha".to_string(),
            signatory_role: "PeerAI".to_string(),
            signature: "(simulated_peer_ai_sig)".to_string(),
        },
    ];

    if !args.invalid_quorum {
        signatures.push(SignaturePackage {
            signatory_id: "auditor-human-01".to_string(),
            signatory_role: "HumanAuditor".to_string(),
            signature: "(simulated_human_sig)".to_string(),
        });
    }

    let package = OverridePackage {
        payload,
        signatures,
    };

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://127.0.0.1:8000/emergency_reissue")
        .json(&package)
        .send();

    match res {
        Ok(response) => {
            println!("-> Sent OverridePackage. Server response: {}", response.status());
            println!("{}", response.text().unwrap_or_default());
        }
        Err(e) => eprintln!("-> Failed to send OverridePackage: {}", e),
    }
}
