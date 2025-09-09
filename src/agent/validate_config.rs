use clap::Parser;
use kairo_lib::config::{load_agent_config, validate_agent_config};
use serde_json::Value;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "agent_config.json")]
    path: String,
}

fn main() {
    let args = Args::parse();
    let json_str = match std::fs::read_to_string(&args.path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let value: Value = match serde_json::from_str(&json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Invalid JSON: {}", e);
            std::process::exit(1);
        }
    };

    for key in ["agent_id", "p_address", "public_key", "signature"] {
        if !value.get(key).is_some() {
            eprintln!("❌ Missing required field '{}'", key);
            std::process::exit(1);
        }
    }

    match load_agent_config(&args.path) {
        Ok(cfg) => match validate_agent_config(&cfg) {
            Ok(()) => println!("✅ {} is valid", args.path),
            Err(e) => {
                eprintln!("❌ Validation failed: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to parse AgentConfig: {}", e);
            std::process::exit(1);
        }
    }
}
