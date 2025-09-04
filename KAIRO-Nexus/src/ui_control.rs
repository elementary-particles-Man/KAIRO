//! kairo-nexus/src/ui_control.rs

use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::net::SocketAddr;

/// ===== 共通 I/F =====
/// The main trait for interacting with a UI bridge.
#[async_trait]
pub trait Ui: Send + Sync {
    // The `paste` and `send` concepts are merged for TCP.
    // We send a command and immediately expect a response.
    async fn send_command(&self, command: &str) -> Result<String>;
}

/// UI 種別
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UiKind {
    Chrome,
    WinApp,
    Terminal,
}

/// UI を特定するキー (maps.json の kind/port に対応)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UiKey {
    pub kind: UiKind,
    pub port: u16,
}

/// ===== TerminalUi 実装 (TCP ブリッジ方式) =====
/// The bridge script is expected to be a TCP server listening on 127.0.0.1:{port}.
/// Nexus connects, sends one command, reads one response, and closes the connection.
pub struct TerminalUi {
    addr: SocketAddr,
}

impl TerminalUi {
    pub fn new(port: u16) -> Self {
        Self {
            addr: SocketAddr::new([127, 0, 0, 1].into(), port),
        }
    }
}

#[async_trait]
impl Ui for TerminalUi {
    async fn send_command(&self, command: &str) -> Result<String> {
        // Connect to the bridge server
        let mut stream = TcpStream::connect(self.addr)
            .await
            .with_context(|| format!("Failed to connect to TCP bridge at {}", self.addr))?;

        // Send the command (request)
        stream.write_all(command.as_bytes()).await?;
        stream.shutdown().await?; // Shutdown the write half to signal end of request

        // Read the response
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;
        
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}

/// ===== ダミー UI（Chrome/WinApp）=====
pub struct DummyUi {
    key: UiKey,
}
impl DummyUi {
    pub fn new(key: UiKey) -> Self { Self { key } }
}

#[async_trait]
impl Ui for DummyUi {
    async fn send_command(&self, command: &str) -> Result<String> {
        eprintln!("[DUMMY {:?}@{}] received command: {}", self.key.kind, self.key.port, command);
        let reply = format!(
            r#"{{"status":"ok","echo":"dummy reply for {:?}@{} to command: {}"}}"#,
            self.key.kind, self.key.port, command
        );
        Ok(reply)
    }
}