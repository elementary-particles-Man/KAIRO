use clap::Parser;
use ed25519_dalek::SigningKey;
use kairo_lib::config as daemon_config;
use kairo_lib::config::{save_agent_config, load_agent_config};
use kairo_lib::registry::{register_agent, RegistryEntry};
use kairo_lib::AgentConfig;
// This import is unused and has been removed.
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct CliArgs {
    #[arg(long)]
    new: bool,
    #[arg(long)]
    force: bool,
    #[arg(
        long,
        help = "エージェント名 (英数字およびアンダースコアのみ。二重引用符は不要です)"
    )]
    name: String,
}

// This function is unused and has been removed.
/* fn get_daemon_assign_url() -> String {
    let config = daemon_config::load_daemon_config(".kairo/config/daemon_config.json")
    .unwrap_or_else(|_| {
        println!("WARN: daemon_config.json not found or invalid. Falling back to default bootstrap address.");
        daemon_config::DaemonConfig {
            listen_address: "127.0.0.1".to_string(),
            listen_port: 8080
        }
    });
    format!(
        "http://{}:{}/assign_p_address",
        config.listen_address, config.listen_port
    )
} */

#[derive(Deserialize)]
struct AgentMapping {
    // p_address is unused in this struct
    // p_address: String,
}

// エージェント名をサニタイズする関数
fn sanitize_agent_name(name: &str) -> Result<(), String> {
    if name.contains('"') || name.contains('\'') || name.contains('/') {
        return Err(format!("エージェント名に無効な文字が含まれています: '{}'。二重引用符、バックスラッシュ、スラッシュは使用できません。おそらく二重引用符が混入しています。", name));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = CliArgs::parse();

    // エージェント名のサニタイズ
    sanitize_agent_name(&cli_args.name)?;

    let agent_config_dir = PathBuf::from("D:/Dev/KAIRO/users")
        .join(&cli_args.name)
        .join("agent_configs");
    fs::create_dir_all(&agent_config_dir)?;

    let agent_config_file_name = format!("{}.json", cli_args.name);
    let agent_config_path = agent_config_dir.join(&agent_config_file_name);

    let mut config: AgentConfig;

    if cli_args.new {
        if agent_config_path.exists() && !cli_args.force {
            eprintln!(
                "Config {:?} already exists. Use --force to overwrite",
                agent_config_path
            );
            return Ok(());
        }
        if agent_config_path.exists() {
            println!(
                "Overwriting existing config at {:?} due to --force",
                agent_config_path
            );
        }
        println!("--- KAIRO Mesh Initial Setup ---");

        // Generate key pair
        use rand::rngs::OsRng;
        use rand::RngCore;
        let mut csprng = OsRng;
        let mut sk_bytes = [0u8; 32];
        csprng.fill_bytes(&mut sk_bytes);
        let signing_key = SigningKey::from_bytes(&sk_bytes);
        let public_key_bytes = signing_key.verifying_key().to_bytes().to_vec();
        let secret_key_bytes = signing_key.to_bytes().to_vec();

        let public_key_hex = hex::encode(&public_key_bytes);
        let secret_key_hex = hex::encode(&secret_key_bytes);

        println!("Secret Key: {}", secret_key_hex);
        println!("Public Key: {}", public_key_hex);

        println!(
            "\nStep 2: Registering with a Seed Node..."
        );

        // Try to request P address from local daemon
        println!(
            "\nSkipping KAIRO-P address assignment from local daemon (not implemented)."
        );
        let p_address = format!("10.0.0.{}/24", rand::random::<u8>()); // ダミーのPアドレスを生成

        println!("-> Assigned Dummy P Address: {}", p_address);

        println!("-> Skipping registration with seed node (not implemented).");

        config = AgentConfig {
            p_address: p_address.clone(),
            public_key: public_key_hex.clone(),
            secret_key: secret_key_hex,
            signature: None,
            last_sequence: 0,
        };

        // Save the new agent config to the specified path
        save_agent_config(config.clone(), agent_config_path.to_str().unwrap())?;
        // Update global registry
        if let Err(e) = register_agent(
            "agent_registry.json",
            RegistryEntry {
                name: cli_args.name.clone(),
                p_address: p_address.clone(),
                deleted: false,
                last_contact: None,
            },
        ) {
            eprintln!("Failed to register agent: {}", e);
        }

        println!("--- Onboarding Complete ---");
    } else if !agent_config_path.exists() {
        eprintln!(
            "Config {:?} not found. Use --new to create it",
            agent_config_path
        );
        return Ok(());
    } else {
        println!("--- Welcome Back ---");
        match load_agent_config(agent_config_path.to_str().unwrap()) {
            Ok(loaded_config) => {
                config = loaded_config;
                println!("Restored identity from agent_config.json");
                println!("Your Public Key: {}", config.public_key);
            },
            Err(e) => {
                eprintln!("Error loading agent config: {}", e);
                return Ok(());
            }
        }
    }

    Ok(())
}