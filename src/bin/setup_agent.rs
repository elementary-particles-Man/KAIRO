//! setup_agent.rs - First-time onboarding CUI
//! 1) ed25519 keypair gen 2) seed register 3) persist agent.toml
use std::{fs, path::PathBuf, io::{self, Write}};
use clap::Parser;
use rand::rngs::OsRng;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[command(name = "setup_agent")]
struct Opts {
    /// run without prompts
    #[arg(long)]
    non_interactive: bool,
    /// output file path for agent material
    #[arg(long)]
    out: Option<String>,
    /// seed endpoint base url (e.g., http://127.0.0.1:8080)
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    seed: String
}

#[derive(Serialize, Deserialize)]
struct AgentToml {
    mesh_address: String,
    public_key_hex: String,
    secret_key_hex: String,
    agent_id: Option<String>,
    issued_at: Option<String>
}

#[derive(Serialize)]
struct RegisterReq {
    public_key_hex: String,
    mesh_address: String
}

#[derive(Deserialize)]
struct RegisterResp {
    agent_id: String,
    issued_at: String
}

fn default_out() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    #[cfg(target_os = "windows")] let p = home.join(".kairo\\agent.toml");
    #[cfg(not(target_os = "windows"))] let p = home.join(".kairo/agent.toml");
    p
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("--- KAIRO Mesh Initial Setup ---");

    // 1) keypair
    let mut csprng = OsRng;
    let sk: SigningKey = SigningKey::generate(&mut csprng);
    let vk: VerifyingKey = (&sk).into();
    let sk_hex = hex::encode(sk.to_bytes());
    let pk_hex = hex::encode(vk.as_bytes());

    // mesh address = pk_hex 短縮やハッシュも可。まずは pk_hex 直採用。
    let mesh_address = pk_hex.clone();

    // 2) seed register
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/register", opts.seed.trim_end_matches('/'));
    let body = RegisterReq { public_key_hex: pk_hex.clone(), mesh_address: mesh_address.clone() };
    let resp: Option<RegisterResp> = match client.post(&url).json(&body).send() {
        Ok(r) if r.status().is_success() => {
            match r.json::<RegisterResp>() { Ok(j) => Some(j), Err(_) => None }
        },
        _ => None
    };

    // 3) persist agent.toml
    let mut out = opts.out.map(PathBuf::from).unwrap_or_else(default_out);
    if let Some(parent) = out.parent() { fs::create_dir_all(parent)?; }
    let agent = AgentToml {
        mesh_address: mesh_address.clone(),
        public_key_hex: pk_hex.clone(),
        secret_key_hex: sk_hex.clone(),
        agent_id: resp.as_ref().map(|x| x.agent_id.clone()),
        issued_at: resp.as_ref().map(|x| x.issued_at.clone())
    };
    let toml_str = toml::to_string_pretty(&agent)?;
    let mut f = fs::File::create(&out)?;
    #[cfg(unix)] { use std::os::unix::fs::PermissionsExt; fs::set_permissions(&out, fs::Permissions::from_mode(0o600))?; }
    f.write_all(toml_str.as_bytes())?;

    // 4) summary
    println!("\n--- Onboarding Complete ---");
    println!("Mesh Address : {}", mesh_address);
    println!("Saved to     : {}", out.display());
    if let Some(r) = resp { println!("Seed Issued  : {} @ {}", r.agent_id, r.issued_at); }
    else { println!("Seed Register: skipped or failed (offline OK)\"); }
    println!("Keep your agent.toml safe. You will use it to launch AI-TCP.");
    Ok(())
}
