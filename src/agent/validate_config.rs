use clap::Parser;
use kairo_lib::config::{load_agent_config, validate_agent_config};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "agent_config.json")]
    path: String,
}

fn main() {
    let args = Args::parse();
    match load_agent_config(&args.path) {
        Ok(cfg) => match validate_agent_config(&cfg) {
            Ok(()) => println!("✅ {} is valid", args.path),
            Err(e) => {
                eprintln!("❌ Validation failed: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
}
