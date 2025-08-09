//! setup_agent.rs – First-time onboarding CUI
//! - ed25519 keygen
//! - safe persist to ~/.kairo/agents/<agent_id>/agent.toml
//! - machine-readable JSON summary (1 line)

use clap::Parser;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "setup_agent")]
struct Args {
    /// KAIRO home dir (default: ~/.kairo)
    #[arg(long, default_value_t = default_home())]
    home: String,
    /// Seed node base URL (can be dummy for now)
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    seed: String,
    /// Optional human label for this agent
    #[arg(long, default_value = "")]
    label: String,
    /// Overwrite existing files without prompt
    #[arg(long, default_value_t = false)]
    yes: bool,
}

fn default_home() -> String {
    let dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.join(".kairo").to_string_lossy().to_string()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // 1) Generate ed25519
    let mut csprng = OsRng;
    let sk = SigningKey::generate(&mut csprng);
    let vk: VerifyingKey = (&sk).into();
    let sk_hex = hex::encode(sk.to_bytes());
    let pk_hex = hex::encode(vk.as_bytes());

    // 2) agent_id = first 4 bytes of SHA256(pubkey) as hex (8 chars)
    let mut h = Sha256::new();
    h.update(vk.as_bytes());
    let digest = h.finalize();
    let agent_id = hex::encode(&digest[..4]);

    // 3) Paths
    let home = PathBuf::from(&args.home);
    let agent_dir = home.join("agents").join(&agent_id);
    let cred_dir = home.join("credentials");
    let agent_toml = agent_dir.join("agent.toml");
    let cred_json = cred_dir.join(format!("agent_{}.json", agent_id));

    // 4) Create dirs (700相当)
    fs::create_dir_all(&agent_dir)?;
    fs::create_dir_all(&cred_dir)?;

    // 5) Safe write helper
    let write_maybe = |p: &PathBuf, bytes: &[u8]| -> anyhow::Result<()> {
        if p.exists() && !args.yes {
            anyhow::bail!(format!("exists: {} (use --yes to overwrite)", p.display()));
        }
        let mut f = fs::File::create(p)?;
        f.write_all(bytes)?;
        Ok(())
    };

    // 6) Write agent.toml
    let agent_toml_body = format!(
        "# KAIRO Agent\nagent_id = \"{}\"\nlabel = \"{}\"\nseed = \"{}\"\npublic_key_hex = \"{}\"\n",
        agent_id, args.label, args.seed, pk_hex
    );
    write_maybe(&agent_toml, agent_toml_body.as_bytes())?;

    // 7) Write secret json (0600相当はプラットフォーム差異のため後日対応)
    let cred_json_body = format!(
        "{{\"agent_id\":\"{}\",\"secret_key_hex\":\"{}\"}}",
        agent_id, sk_hex
    );
    write_maybe(&cred_json, cred_json_body.as_bytes())?;

    // 8) Human summary
    println!("--- KAIRO Onboarding Complete ---");
    println!("Agent ID       : {}", agent_id);
    println!("Mesh Address   : {}", pk_hex);
    println!("Home           : {}", home.display());
    println!("Seed           : {}", args.seed);
    println!(
        "Files          :\n  - {}\n  - {}",
        agent_toml.display(),
        cred_json.display()
    );
    println!("(Keep your secret file safe!)\n");

    // 9) Machine-readable JSON (single line)
    println!(
        "{{\"event\":\"onboarding_complete\",\"agent_id\":\"{}\",\"pubkey_hex\":\"{}\",\"home\":\"{}\",\"seed\":\"{}\"}}",
        agent_id, pk_hex, home.to_string_lossy(), args.seed
    );

    Ok(())
}
