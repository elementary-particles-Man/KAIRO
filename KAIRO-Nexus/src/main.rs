//! kairo-nexus/src/main.rs
//! Pアドレス宛のメッセージを、maps.json に基づいて適切なUI（ターミナル等）に中継するルータ。

mod ui_control;

use anyhow::{anyhow, Result};
use clap::Parser;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc,
};
use ui_control::{UiKey, UiKind};

/// ============ 引数 =============
#[derive(Parser, Debug)]
#[command(name = "kairo-nexus")]
struct Args {
    #[arg(long, default_value = "./maps.json")]
    map: PathBuf,
}

/// ============ モデル =============
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PAddr(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Envelope {
    to_p: String,
    from_p: String,
    body: String,
}

/// ============ マッピングファイル =============
#[derive(Debug, Clone, Deserialize)]
struct MapsFile {
    bindings: Vec<Binding>,
}

#[derive(Debug, Clone, Deserialize)]
struct Binding {
    p: String,
    ui: UiBinding,
}

#[derive(Debug, Clone, Deserialize)]
struct UiBinding {
    kind: String,
    port: u16,
}

/// ============ レジストリ：P→UI紐付け =============
#[derive(Clone, Default)]
struct Registry {
    inner: Arc<RwLock<HashMap<PAddr, UiKey>>>,
}

impl Registry {
    fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let parsed: MapsFile = serde_json::from_str(&content)?;
        let mut map = HashMap::new();
        for b in parsed.bindings {
            let kind = match b.ui.kind.as_str() {
                "chrome" => UiKind::Chrome,
                "winapp" => UiKind::WinApp,
                "terminal" => UiKind::Terminal,
                _ => return Err(anyhow!("unknown ui.kind: {}", b.ui.kind)),
            };
            map.insert(PAddr(b.p), UiKey { kind, port: b.ui.port });
        }
        Ok(Self {
            inner: Arc::new(RwLock::new(map)),
        })
    }

    fn get(&self, p: &PAddr) -> Option<UiKey> {
        self.inner.read().get(p).cloned()
    }
}

/// ============ JSON抽出（最小） =============
fn extract_json_or_fallback(s: &str) -> String {
    if let Some(idx) = s.find('{') {
        let bytes = s.as_bytes();
        let mut depth = 0i32;
        for (i, &b) in bytes[idx..].iter().enumerate() {
            match b {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        let end = idx + i + 1;
                        let candidate = &s[idx..end];
                        if serde_json::from_str::<Value>(candidate).is_ok() {
                            return candidate.to_string();
                        }
                    }
                }
                _ => {}
            }
        }
    }
    s.to_string()
}

/// ============ ルータ本体 =============
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let registry = match Registry::load_from_file(&args.map) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to load map file: {e}");
            return Err(e);
        }
    };

    let (tx, mut rx) = mpsc::unbounded_channel::<Envelope>();

    // 標準入力から 1 行 1 メッセージ
    tokio::spawn(async move {
        let mut reader = BufReader::new(tokio::io::stdin()).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            if line.trim().is_empty() {
                continue;
            }
            match serde_json::from_str::<Envelope>(&line) {
                Ok(env) => {
                    let _ = tx.send(env);
                }
                Err(e) => eprintln!("Invalid envelope JSON: {e}"),
            }
        }
    });

    // ルーティングループ
    while let Some(env) = rx.recv().await {
        if let Err(e) = handle(env, registry.clone()).await {
            eprintln!("route error: {e}");
        }
    }

    Ok(())
}

async fn handle(env: Envelope, registry: Registry) -> Result<()> {
    let to = PAddr(env.to_p.clone());
    let from = PAddr(env.from_p.clone());

    let to_ui_key = registry
        .get(&to)
        .ok_or_else(|| anyhow!("no UI bound for {}", to.0))?;
    let from_ui_key = registry
        .get(&from)
        .ok_or_else(|| anyhow!("no UI bound for {}", from.0))?;

    let to_client = make_ui_client(to_ui_key);
    let from_client = make_ui_client(from_ui_key);

    // Send the command to the destination and get the output
    let out = to_client.send_command(&env.body).await?;
    let reply = extract_json_or_fallback(&out);

    // Send the reply back to the sender
    from_client.send_command(&reply).await?;
    Ok(())
}

fn make_ui_client(key: UiKey) -> Box<dyn ui_control::Ui> {
    match key.kind {
        UiKind::Terminal => Box::new(ui_control::TerminalUi::new(key.port)),
        _ => Box::new(ui_control::DummyUi::new(key)),
    }
}
