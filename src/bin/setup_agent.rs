//! setup_agent: 初回オンボーディングCUI
use clap::Parser;
use chrono::Utc;
use dirs::home_dir;
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::Serialize;
use std::fs;
use std::io::{Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "setup_agent")]
#[command(about = "Generate ed25519 keypair and write ~/.kairo/agent.json")]
struct Opts {
    /// 既存ファイルを上書き
    #[arg(long)]
    force: bool,
}

#[derive(Serialize)]
struct AgentFile {
    public_key: String,
    private_key: String,
    mesh_address: String,
    created_at: String,
    version: u8,
}

fn kairo_dir() -> PathBuf {
    let mut home = home_dir().expect("HOME not found");
    home.push(".kairo");
    home
}

fn agent_path() -> PathBuf {
    let mut p = kairo_dir();
    p.push("agent.json");
    p
}

fn derive_mesh_address(pk: &VerifyingKey) -> String {
    // 便宜的に公開鍵(32B)の先頭8BをPアドレス由来値として十六進化
    let bytes = pk.as_bytes();
    let head = &bytes[..8.min(bytes.len())];
    format!("ka:{}", hex::encode(head))
}

#[cfg(unix)]
fn secure_permissions(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o600);
    fs::set_permissions(path, perms)
}

#[cfg(windows)]
fn secure_permissions(path: &Path) -> std::io::Result<()> {
    // Windowsはデフォルトでユーザー専用ACLになっていることが多いが、
    // 明示的に現在ユーザーのみに読み書きを付与
    use windows_acl::acl::{AceType, ACL};
    let p = path.to_string_lossy().to_string();
    let mut acl = ACL::from_file_path(&p)?;
    acl.clear()?;
    acl.allow_user_current(AceType::GenericRead | AceType::GenericWrite)?;
    acl.apply(&p)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("--- KAIRO Mesh Initial Setup ---");

    // 生成先
    let dir = kairo_dir();
    let path = agent_path();
    if !dir.exists() { fs::create_dir_all(&dir)?; }
    if path.exists() && !opts.force {
        println!("既存の ~/.kairo/agent.json が見つかりました。--force で上書きできます。");
        println!("Path: {}", path.display());
        return Ok(());
    }

    println!("Step 1: Generating ed25519 keypair...");
    let mut rng = OsRng;
    let sk = SigningKey::generate(&mut rng);
    let vk: VerifyingKey = (&sk).into();

    let public_key_hex = hex::encode(vk.as_bytes());
    let private_key_hex = hex::encode(sk.to_bytes());
    let mesh_address = derive_mesh_address(&vk);

    let payload = AgentFile {
        public_key: public_key_hex.clone(),
        private_key: private_key_hex,
        mesh_address: mesh_address.clone(),
        created_at: Utc::now().to_rfc3339(),
        version: 1,
    };

    let json = serde_json::to_vec_pretty(&payload)?;
    let mut f = fs::OpenOptions::new().create(true).write(true).truncate(true).open(&path)?;
    f.write_all(&json)?;
    f.flush()?;
    drop(f);
    secure_permissions(&path)?;

    println!("\n--- Onboarding Complete ---");
    println!("Saved: {}", path.display());
    println!("public_key: {}", public_key_hex);
    println!("mesh_address: {}", mesh_address);
    println!("(秘密鍵はファイルのみ。画面には表示しません)");
    println!("\n再実行時はデフォルトで上書きしません。上書きする場合は --force を付与してください。");
    Ok(())
}
