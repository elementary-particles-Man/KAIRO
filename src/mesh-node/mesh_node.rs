// メッシュノード（中継転送専用）
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::{net::UdpSocket, sync::mpsc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeshPacket {
    pub from: String,
    pub to: String,
    pub payload: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // UDPバインド（例：0.0.0.0:5050）
    let socket = UdpSocket::bind("0.0.0.0:5050").await?;
    let mut buf = [0; 2048];

    // 転送先アドレス管理用（動的登録）
    let route_table: Arc<RwLock<HashMap<String, SocketAddr>>> =
        Arc::new(RwLock::new(HashMap::new()));

    println!("🌐 Mesh Node is running on 0.0.0.0:5050...");
    loop {
        let (len, sender_addr) = socket.recv_from(&mut buf).await?;
        let packet: MeshPacket = match serde_json::from_slice(&buf[..len]) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("⚠️ Invalid packet received.");
                continue;
            }
        };

        // 発信元を登録
        route_table
            .write()
            .await
            .insert(packet.from.clone(), sender_addr);

        // 宛先を探索して転送
        let table = route_table.read().await;
        if let Some(target_addr) = table.get(&packet.to) {
            let bytes = serde_json::to_vec(&packet)?;
            socket.send_to(&bytes, target_addr).await?;
            println!("📤 Forwarded from {} to {}", packet.from, packet.to);
        } else {
            eprintln!("❌ No route to {}", packet.to);
        }
    }
}
