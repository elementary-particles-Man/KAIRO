use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;
use kairo_lib::packet::AiTcpPacket;
use log::info;
use reqwest;

pub async fn run_listener() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("KAIRO-P listening on 8080...");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Accepted from {:?}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 4096];
            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => {
                    let received_data = &buf[..n];
                    let received_str = match std::str::from_utf8(received_data) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Failed to convert received data to UTF-8: {:?}", e);
                            return;
                        }
                    };

                    match serde_json::from_str::<AiTcpPacket>(received_str) {
                        Ok(packet) => {
                            // パケットをJSONにシリアライズ
                            let _packet_json = serde_json::to_string(&packet).expect("Failed to serialize packet");

                            // 内部的なHTTP POSTリクエストを送信
                            let client = reqwest::Client::new();
                            let res = client.post("http://127.0.0.1:3030/send")
                                .json(&packet) // AiTcpPacketを直接JSONとして送信
                                .send()
                                .await;

                            match res {
                                Ok(response) => {
                                    if response.status().is_success() {
                                        let response_body = response.text().await.unwrap_or_else(|_| "{}".to_string());
                                        info!("Successfully forwarded packet to daemon. Response: {}", response_body);
                                        let _ = socket.write_all(response_body.as_bytes()).await; // デーモンからの応答をクライアントに返す
                                    } else {
                                        eprintln!("Failed to forward packet to daemon. Status: {:?}", response.status());
                                        let _ = socket.write_all(b"ERROR: Daemon forwarding failed").await;
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Failed to send HTTP request to daemon: {:?}", e);
                                    let _ = socket.write_all(b"ERROR: Internal forwarding failed").await;
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to deserialize packet: {:?} from {:?}", e, addr);
                            let _ = socket.write_all(b"ERROR: Invalid packet format").await;
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read from socket: {:?}", e);
                }
            }
        });
    }
}