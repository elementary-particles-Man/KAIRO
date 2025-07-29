use clap::Parser;
use kairo_lib::comm::{sign_message, Message};
use kairo_lib::config::load_agent_config;
use reqwest::blocking::Client;
use std::fs;
use std::process;

/// CLI arguments for the signed sender
#[derive(Parser, Debug)]
#[command(name = "signed_sender")]
#[command(about = "Send a signed message to the KAIRO network", long_about = None)]
struct Args {
    /// The destination P address
    #[arg(short, long)]
    to: String,

    /// The message payload to send
    #[arg(short, long)]
    payload: String,

    /// Path to the agent's configuration file
    #[arg(short, long, default_value = "agent_configs/config_agent_1.json")]
    config: String,

    /// Optional sender ID (overrides config file value)
    #[arg(short, long)]
    from: String,

    /// Allow mismatch between --from and config.agent_id (use with caution)
    #[arg(long)]
    allow_mismatch: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    // Load agent config from file
    let config = load_agent_config(&args.config).unwrap_or_else(|err| {
        eprintln!("❌ Failed to load agent config: {}", err);
        process::exit(1);
    });

    // Determine actual sender ID
    let from_id = args.from.clone();

    // Check consistency if --from was given
    if args.from != config.public_key && !args.allow_mismatch {
        eprintln!(
            "❌ --from argument ({}) does not match public_key in config ({})",
            args.from, config.public_key
        );
        eprintln!("ℹ️  Use --allow-mismatch to override this check.");
        process::exit(1);
    }

    // Construct message
    let message = Message {
        from: from_id.clone(),
        to: args.to.clone(),
        payload: args.payload.clone(),
        signature: String::new(), // Placeholder
    };

    // Sign the message
    let signed_message = sign_message(&message, &config.secret_key).unwrap_or_else(|err| {
        eprintln!("❌ Failed to sign message: {}", err);
        process::exit(1);
    });

    // Send it
    let client = Client::new();
    let res = client
        .post("http://127.0.0.1:8080/send")
        .json(&signed_message)
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("✅ Message sent successfully to {}", args.to);
            } else {
                eprintln!("❌ Server responded with status: {}", response.status());
            }
        }
        Err(err) => {
            eprintln!("❌ Failed to send message: {}", err);
            process::exit(1);
        }
    }

    // verify delivery
    match client.get("http://127.0.0.1:8080/").send() {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("✅ Daemon acknowledged receipt");
            } else {
                eprintln!("⚠️  Verification failed: {}", resp.status());
            }
        }
        Err(e) => eprintln!("⚠️  Failed to verify delivery: {}", e),
    }
}
