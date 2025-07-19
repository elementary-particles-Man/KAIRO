//! src/kairo-lib/lib.rs

// --- モジュール公開宣言 ---
pub mod config;
pub mod governance;
pub mod packet;

// --- 構造体・型の再エクスポート ---
pub use governance::OverridePackage;
pub use packet::AiTcpPacket;

// --- AgentConfig の定義とユーティリティ関数 ---
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self};

/// Agentの鍵・アドレス情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub p_address: String,
    pub public_key: String,
    pub secret_key: String,
    pub signature: String,
}

/// AgentConfig を JSON ファイルとして保存
pub fn save_agent_config(config: &AgentConfig) -> io::Result<()> {
    let file = File::create("agent_config.json")?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}

/// AgentConfig に署名を追加（現在はダミー）
pub fn sign_config(config: &mut AgentConfig) {
    // 仮実装：将来的には秘密鍵とConfigのハッシュに基づく署名へ
    config.signature = format!("signed({})", config.public_key);
}
