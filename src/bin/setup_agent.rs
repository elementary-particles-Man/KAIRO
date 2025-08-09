//! src/bin/setup_agent.rs
//! KAIRO Mesh: First-time onboarding CUI

use std::{fs, io, path::PathBuf, time::SystemTime};
use clap::Parser;
use dirs::home_dir;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey};

#[derive(Parser, Debug)]
#[command(name = "setup_agent", about = "KAIRO Mesh onboarding")]
struct Args {
    /// scope level: personal|family|group|community|world
    #[arg(long, default_value = "personal")]
    scope: String,
    /// overwrite existing keys
    #[arg(long, default_value_t = false)]
    force: bool,
}

#[derive(Serialize, Deserialize)]
struct AgentJson {
    public_key_hex: String,
    secret_key_hex: String,
    scope: String,
    created_at: String,
}

fn kairo_dir() -> io::Result<PathBuf> {
    let mut p = home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no home dir"))?;
    p.push(".kairo");
    if !p.exists() { fs::create_dir_all(&p)?; }
    Ok(p)
}

fn now_iso() -> String {
    let t = SystemTime::now();
    let dt: chrono::DateTime<chrono::Utc> = t.into();
    dt.to_rfc3339()
}

fn mesh_address_from_pk(pk: &VerifyingKey, scope: &str) -> String {
    // NOTE: simple placeholder — real allocator will encode scope bits
    let suffix = &hex::encode(pk.as_bytes())[0..4];
    match scope {
        "personal" => format!("f5f9:abcd::{} /120 (scope=personal)", suffix),
        "family" => format!("f5f9:abcd:{}:: /96 (scope=family)", suffix),
        "group" => format!("f5f9:ab:{}:: /64 (scope=group)", suffix),
        "community" => format!("f5f9:{}:: /48 (scope=community)", suffix),
        _ => format!("f5f9::{} /32 (scope=world)", suffix),
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    println!("--- KAIRO Mesh Initial Setup ---");

    let dir = kairo_dir()?;
    let agent_path = dir.join("agent.json");
    let addr_path = dir.join("mesh_address.txt");

    if agent_path.exists() && !args.force {
        println!("Existing agent found: {}", agent_path.display());
        println!("Tip: re-generate with --force");
        println!("Next: kairo ai-tcp start --token {}", agent_path.display());
        return Ok(());
    }

    // Step1: generate ed25519 keypair
    println!("\nStep 1: Generating Static ID (ed25519)...");
    let mut rng = OsRng;
    let sk = SigningKey::generate(&mut rng);
    let vk: VerifyingKey = (&sk).into();
    let sk_hex = hex::encode(sk.to_bytes());
    let vk_hex = hex::encode(vk.as_bytes());

    // Step2: simulate seed registration
    println!("Step 2: Registering with Seed Node (simulated)...");
    let seed_status = "queued"; // TODO: real HTTP call

    // Files: agent.json
    let agent = AgentJson { public_key_hex: vk_hex.clone(), secret_key_hex: sk_hex.clone(), scope: args.scope.clone(), created_at: now_iso() };
    let agent_json = serde_json::to_string_pretty(&agent).unwrap();
    fs::write(&agent_path, agent_json)?;

    // Files: mesh_address.txt
    let addr = mesh_address_from_pk(&vk, &args.scope);
    fs::write(&addr_path, addr.as_bytes())?;

    // Logs
    let log_dir = PathBuf::from("./logs");
    if !log_dir.exists() { let _ = fs::create_dir_all(&log_dir); }
    let log_path = log_dir.join(format!("onboarding_{}.log", chrono::Utc::now().format("%Y%m%dT%H%M%SZ")));
    let log_body = format!("status=ok\nscope={}\nseed_status={}\nagent={}\nmesh_addr={}\n", args.scope, seed_status, agent_path.display(), addr_path.display());
    fs::write(&log_path, log_body)?;

    println!("\n--- Onboarding Complete ---");
    println!("Mesh Address: {}", addr);
    println!("Files:\n  {}\n  {}", agent_path.display(), addr_path.display());
    println!("Next: kairo ai-tcp start --token {}", agent_path.display());
    Ok(())
}

