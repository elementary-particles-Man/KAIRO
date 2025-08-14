//! KAIRO-Nexus MVP (LSC-TCP v0.1) -- mcp.rs
//! - send: 指定先へ LSC-TCP JSON を送信（署名は空文字で後日）
//! - receive: デーモンから受信した LSC-TCP JSON を表示
//! 既定設定：~/.kairo/agent.json（単一運用） or agent_configs/{name}.json（複数運用）

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use uuid::Uuid;

/// LSC-TCP v0.1 メッセージ
#[derive(Debug, Serialize, Deserialize)]
struct LscTcpMessage {
    version: String,          // "0.1.0"
    from_p_address: String,   // 送信元 P アドレス（または公開鍵導出値）
    to_p_address: String,     // 宛先
    task_id: String,          // UUID v4 もしくは timestamp
    payload: Payload,
    signature: String,        // MVPでは空文字
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    #[serde(rename = "type")]
    kind: String,             // 例: "code_generation_request"
    content: String,          // 指示本文
}

/// agent.json / {name}.json の最小スキーマ想定
#[derive(Debug, Deserialize)]
struct AgentFile {
    public_key: Option<String>,
    mesh_address: Option<String>,
    // …ほかフィールドは無視（private_key 等）
}

#[derive(Parser, Debug)]
#[command(name = "mcp", about = "KAIRO-Nexus (MCP) MVP CLI")]
struct Cli {
    /// 名前付きエージェントを使う場合 (agent_configs/<name>.json を参照)
    #[arg(long)]
    name: Option<String>,

    /// 単一運用の保存先を切り替える（既定は ~/.kairo/agent.json）
    #[arg(long)]
    agent_path: Option<PathBuf>,

    /// デーモンのベースURL
    #[arg(long, default_value = "http://127.0.0.1:4040")]
    daemon_url: String,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// 送信：LSC-TCP v0.1 でメッセージを送る
    Send {
        /// 宛先 P アドレス
        #[arg(long)]
        to: String,

        /// 指示内容
        #[arg(long)]
        message: String,

        /// payload.type（既定: "status_report"）
        #[arg(long, default_value = "status_report")]
        payload_type: String,

        /// JSONを送らず表示だけする（疎通前の確認用）
        #[arg(long, default_value_t = false)]
        dry_run: bool,
    },

    /// 受信：デーモンからメッセージを取得して表示
    Receive {
        /// 追従表示（long-poll風に繰り返し取得）
        #[arg(long, default_value_t = false)]
        follow: bool,

        /// 取得間隔（秒）
        #[arg(long, default_value_t = 3)]
        interval_sec: u64,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 送信元 P アドレスの決定
    let from_p = resolve_from_p_address(cli.name.as_deref(), cli.agent_path.as_ref())
        .context("failed to resolve from_p_address")?;

    match cli.cmd {
        Cmd::Send { to, message, payload_type, dry_run } => {
            let msg = LscTcpMessage {
                version: "0.1.0".to_string(),
                from_p_address: from_p.clone(),
                to_p_address: to,
                task_id: Uuid::new_v4().to_string(),
                payload: Payload { kind: payload_type, content: message },
                signature: String::new(), // 署名は後日実装
            };

            if dry_run {
                println!("{}", serde_json::to_string_pretty(&msg)?);
                return Ok(());
            }

            // 送信用：デーモンの仮API /mcp/send へ POST
            let url = format!("{}/mcp/send", cli.daemon_url.trim_end_matches('/'));
            let resp = reqwest::blocking::Client::new()
                .post(url)
                .json(&msg)
                .send()
                .context("failed to POST /mcp/send")?;

            if !resp.status().is_success() {
                anyhow::bail!("send failed: HTTP {}", resp.status());
            }
            println!("sent: task_id={}", msg.task_id);
        }

        Cmd::Receive { follow, interval_sec } => {
            // 受信用：デーモンの仮API /mcp/inbox?for=<from_p> から GET
            let client = reqwest::blocking::Client::new();
            let url_base = format!("{}/mcp/inbox?for={}", cli.daemon_url.trim_end_matches('/'), urlencoding::encode(&from_p));

            if !follow {
                let text = client.get(&url_base).send()
                    .context("failed to GET /mcp/inbox")?
                    .text()
                    .context("failed to read response text")?;
                println!("{}", text);
            } else {
                loop {
                    let text = client.get(&url_base).send()
                        .context("failed to GET /mcp/inbox (follow)")?
                        .text()
                        .unwrap_or_else(|_| String::from("")); // ノイズは無視
                    if !text.trim().is_empty() {
                        println!("{}", text);
                    }
                    std::thread::sleep(std::time::Duration::from_secs(interval_sec));
                }
            }
        }
    }

    Ok(())
}

/// 送信元の P アドレスを解決
/// 優先順：
///   1) agent_configs/<name>.json の mesh_address
///   2) agent_configs/<name>.json の public_key から導出（先頭短縮）
///   3) ~/.kairo/agent.json（または --agent-path）
///   4) それでも無ければエラー
fn resolve_from_p_address(name: Option<&str>, agent_path: Option<&PathBuf>) -> Result<String> {
    // 1) 名前付き
    if let Some(n) = name {
        let p = PathBuf::from("agent_configs").join(format!("{n}.json"));
        if p.exists() {
            if let Some(addr) = read_mesh_or_pk(&p)? {
                return Ok(addr);
            }
        }
    }
    // 2) 明示パス
    if let Some(p) = agent_path {
        if p.exists() {
            if let Some(addr) = read_mesh_or_pk(p)? {
                return Ok(addr);
            }
        }
    }
    // 3) 単一運用の既定 (~/.kairo/agent.json or $KAIRO_HOME/agent.json)
    let mut home = if let Ok(h) = std::env::var("KAIRO_HOME") {
        PathBuf::from(h)
    } else {
        dirs::home_dir().context("home dir not found")?
    };
    home.push("agent.json");
    if home.exists() {
        if let Some(addr) = read_mesh_or_pk(&home)? {
            return Ok(addr);
        }
    }
    anyhow::bail!("agent config not found (name/path/home). Run `setup_agent` first.");
}

/// JSONから mesh_address を優先取得、無ければ public_key 先頭から短縮導出
fn read_mesh_or_pk(p: &PathBuf) -> Result<Option<String>> {
    let s = fs::read_to_string(p).with_context(|| format!("read {}", p.display()))?;
    let jf: AgentFile = serde_json::from_str(&s).with_context(|| format!("parse {}", p.display()))?;
    if let Some(addr) = jf.mesh_address {
        if !addr.trim().is_empty() {
            return Ok(Some(addr));
        }
    }
    if let Some(pk) = jf.public_key {
        if pk.len() >= 16 {
            let short = format!("{}:{}:{}:{}", &pk[0..4], &pk[4..8], &pk[8..12], &pk[12..16]);
            return Ok(Some(short));
        }
    }
    Ok(None)
}
