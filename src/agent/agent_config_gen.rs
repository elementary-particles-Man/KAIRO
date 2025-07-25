// 自動 agent_config.json 生成ツール
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Serialize};
use std::{fs, env};
use hex;

#[derive(Serialize)]
struct AgentConfig {
    public_key: String,
    secret_key: String,
    signature: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <agent_name>", args[0]);
        return;
    }
    let name = &args[1];
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    let config = AgentConfig {
        public_key: hex::encode(verifying_key.to_bytes()),
        secret_key: hex::encode(signing_key.to_bytes()),
        signature: "".to_string()
    };

    let path = format!("{}_config.json", name);
    let json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&path, json).unwrap();
    println!("✅ {} written", path);
}
