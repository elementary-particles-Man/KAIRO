use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use hex::{encode, decode};


#[derive(Debug, Deserialize)]
pub struct DaemonConfig {
    pub listen_address: String,
    pub listen_port: u16,
}

pub fn load_daemon_config(path: &str) -> Result<DaemonConfig, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: DaemonConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentConfig {
    pub p_address: String,
    pub public_key: String,
    pub secret_key: String,
    pub signature: Option<String>,
    #[serde(default)]
    pub last_sequence: u64,
}

impl AgentConfig {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let secret_key_bytes: [u8; 32] = { let mut bytes = [0u8; 32]; csprng.fill_bytes(&mut bytes); bytes };
        let signing_key = SigningKey::from_bytes(&secret_key_bytes);
        let verifying_key = VerifyingKey::from(&signing_key);

        AgentConfig {
            p_address: String::new(), // 仮の値
            public_key: encode(verifying_key.to_bytes()),
            secret_key: encode(secret_key_bytes),
            signature: None,
            last_sequence: 0,
        }
    }

    // 署名対象のデータを生成
    fn get_signable_data(&self) -> Vec<u8> {
        format!("{}-{}-{}", self.p_address, self.public_key, self.secret_key).as_bytes().to_vec()
    }

    // 設定に署名
    pub fn sign(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let secret_key_bytes = decode(&self.secret_key)?;
        let signing_key = SigningKey::from_bytes(secret_key_bytes.as_slice().try_into()?);
        let signature = signing_key.sign(&self.get_signable_data());
        self.signature = Some(encode(signature.to_bytes()));
        Ok(())
    }

    // 署名を検証
    pub fn verify(&self) -> Result<(), String> {
        if let Some(sig_hex) = &self.signature {
            let public_key_bytes = decode(&self.public_key).map_err(|_| "Invalid public key hex")?;
            let verifying_key = VerifyingKey::from_bytes(public_key_bytes.as_slice().try_into().map_err(|_| "Invalid public key length")?)
                .map_err(|_| "Invalid public key")?;
            let signature_bytes = decode(sig_hex).map_err(|_| "Invalid signature hex")?;
            let signature_bytes_array: [u8; 64] = signature_bytes.as_slice().try_into().map_err(|_| "Invalid signature length")?;
            let signature = Signature::from_bytes(&signature_bytes_array);

            verifying_key.verify(&self.get_signable_data(), &signature)
                .map_err(|_| "CRITICAL: Agent configuration has been TAMPERED WITH.".to_string())?;
            Ok(())
        } else {
            Err("Agent configuration has no signature.".to_string())
        }
    }
}

pub fn save_agent_config(mut config: AgentConfig, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    config.sign()?; // 保存前に署名
    let json = serde_json::to_string_pretty(&config)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_agent_config(path: &str) -> Result<AgentConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let config: AgentConfig = serde_json::from_str(&json)?;
    config.verify()?; // 読み込み後に署名を検証
    Ok(config)
}

pub fn load_first_config() -> AgentConfig {
    match load_agent_config("agent_config.json") {
        Ok(config) => config,
        Err(_) => {
            let config = AgentConfig::generate();
            let _ = save_agent_config(config.clone(), "agent_config.json"); // cloneして渡す
            config
        }
    }
}

pub fn load_all_configs() -> Result<Vec<AgentConfig>, Box<dyn std::error::Error>> {
    let mut configs = Vec::new();
    for entry in fs::read_dir("agent_configs")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let config_str = fs::read_to_string(&path)?;
            let config: AgentConfig = serde_json::from_str(&config_str)?;
            // ここは私のverify()を使うように変更
            if config.verify().is_ok() {
                configs.push(config);
            } else {
                println!("WARNING: Skipping tampered config {:?}", path);
            }
        }
    }
    Ok(configs)
}

/// Validate the structure of an `AgentConfig`.
/// This performs basic sanity checks on P address and key lengths.
pub fn validate_agent_config(cfg: &AgentConfig) -> Result<(), String> {
    if !cfg.p_address.contains('/') {
        return Err("p_address must contain subnet like '10.0.0.x/24'".to_string());
    }
    if cfg.public_key.len() != 64 || !cfg.public_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("public_key must be 64 hex characters".to_string());
    }
    if cfg.secret_key.len() != 64 || !cfg.secret_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("secret_key must be 64 hex characters".to_string());
    }
    Ok(())
}
