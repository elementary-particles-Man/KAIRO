use tokio::net::UdpSocket;
use eyre::Result;
use serde::{Deserialize, Serialize};
use clap::Parser;
use log::{info, error, LevelFilter};
use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config, TerminalMode, ColorChoice};
use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
struct MeshPacket {
    from: String,
    to: String,
    payload: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    log_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(log_path) = args.log_file {
        CombinedLogger::init(
            vec![
                WriteLogger::new(
                    LevelFilter::Info,
                    Config::default(),
                    File::create(log_path)?,
                )
            ]
        )?;
    } else {
        TermLogger::init(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )?;
    }

    info!("Mesh node started.");

    let listen_addr = "0.0.0.0:8080"; // メッシュノードがリッスンするアドレス
    let socket = UdpSocket::bind(listen_addr).await?;
    info!("Listening on: {}", listen_addr);

    let mut buf = [0; 1024];

    loop {
        let (len, peer_addr) = socket.recv_from(&mut buf).await?;
        let received_data = &buf[..len];

        match serde_json::from_slice::<MeshPacket>(received_data) {
            Ok(packet) => {
                info!("Received MeshPacket: {:?} from {}", packet, peer_addr);
                // ここで転送ロジックを実装
                // 例: ログに出力する
                info!("Forwarded from {} to {}: {}", packet.from, packet.to, packet.payload);
            },
            Err(e) => {
                error!("Failed to parse MeshPacket: {} from {}", e, peer_addr);
            }
        }
    }
}