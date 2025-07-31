//! src/bot/main.rs

use simple_logger; // simple_logger は残す
use log::*;
use warp::{Filter, http::StatusCode, fs as warp_fs};
use kairo_lib::config::{self};
use ed25519_dalek::{SigningKey, VerifyingKey};
use hex::{decode};
use kairo_core::ai_tcp_packet_generated::aitcp::{AITcpPacket, AITcpPacketArgs};
use flatbuffers::FlatBufferBuilder;
use reqwest::Client;
use std::fs;
use std::path::Path;
use simplelog::{Config, WriteLogger, LevelFilter}; // simplelog を追加

mod api;
use api::status::status_route;

#[tokio::main]
async fn main() {
    // ログディレクトリが存在しない場合は作成
    let log_dir = "logs";
    if !Path::new(log_dir).exists() {
        fs::create_dir_all(log_dir).expect("Failed to create log directory");
    }

    let log_file_path = format!("{}/kairobot.log", log_dir);

    // ログファイルが既に存在する場合は削除
    if Path::new(&log_file_path).exists() {
        fs::remove_file(&log_file_path).expect("Failed to remove existing log file");
    }

    // simplelog を使用してログをファイルに出力
    WriteLogger::init(
        LevelFilter::Debug, // ログレベル
        Config::default(),
        fs::File::create(&log_file_path).expect("Failed to create log file"),
    ).expect("Failed to initialize logger");

    info!("KAIROBOT: Logger initialized successfully. Starting warp server...");

    let cli_config = match config::load_agent_config("./users/CLI/cli_identity.json") {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load CLI config: {}", e);
            // ファイルが存在しない場合は新しく生成して保存する
            if let Some(os_err) = e.downcast_ref::<std::io::Error>() {
                if os_err.kind() == std::io::ErrorKind::NotFound {
                    info!("CLI config file not found. Generating a new one.");
                    let new_config = config::AgentConfig::generate();
                    if let Err(save_err) = config::save_agent_config(new_config.clone(), "./users/CLI/cli_identity.json") {
                        error!("Failed to save new CLI config: {}", save_err);
                        panic!("Failed to save new CLI config: {}", save_err);
                    }
                    info!("New CLI config generated and saved.");
                    new_config
                } else {
                    panic!("Failed to load CLI config: {}", e);
                }
            } else {
                panic!("Failed to load CLI config: {}", e);
            }
        }
    };

    let secret_key_bytes: [u8; 32] = decode(&cli_config.secret_key).unwrap().try_into().unwrap();
    let _signing_key = SigningKey::from_bytes(&secret_key_bytes);
    let public_key_bytes = decode(&cli_config.public_key).unwrap();
    let _verifying_key = VerifyingKey::from_bytes(&public_key_bytes.as_slice().try_into().unwrap()).unwrap();

    let client = Client::new();

    let forward_packet = warp::path("forward")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |_body: serde_json::Value| {
            let client = client.clone();
            let _signing_key = _signing_key.clone();
            let _verifying_key = _verifying_key.clone();
            async move {
                // ここでAiTcpPacketを構築
                let mut builder = FlatBufferBuilder::new();
                let ephemeral_key_vec = builder.create_vector(&[0u8; 32]); // 仮の値
                let nonce_vec = builder.create_vector(&[0u8; 12]); // 仮の値
                let encrypted_sequence_id_vec = builder.create_vector(&[0u8; 8]); // 仮の値
                let encrypted_payload_vec = builder.create_vector(&[0u8; 16]); // 仮の値
                let signature_vec = builder.create_vector(&[0u8; 64]); // 仮の値

                let packet_args = AITcpPacketArgs {
                    version: 1,
                    ephemeral_key: Some(ephemeral_key_vec),
                    nonce: Some(nonce_vec),
                    encrypted_sequence_id: Some(encrypted_sequence_id_vec),
                    encrypted_payload: Some(encrypted_payload_vec),
                    signature: Some(signature_vec),
                    header: None,
                    payload: None,
                    footer: None,
                };
                let packet = AITcpPacket::create(&mut builder, &packet_args);
                builder.finish(packet, None);
                let finished_buf = builder.finished_data();

                // デーモンへの転送ロジック
                let daemon_url = "http://127.0.0.1:8080/packet"; // デーモンのURL
                let response = client.post(daemon_url)
                    .body(finished_buf.to_vec())
                    .send()
                    .await;

                (match response {
                    Ok(response) => Ok(warp::reply::with_status(warp::reply::json(&response.status().as_u16()), response.status())),
                    Err(_) => Ok(warp::reply::with_status(warp::reply::json(&"Failed to forward packet to daemon"), StatusCode::INTERNAL_SERVER_ERROR)),
                }) as Result<_, warp::Rejection>
            }
        });

    // UI ルートを追加
    let ui_routes = warp_fs::dir("vov/kairobot_ui");
    let index_route = warp::path::end().and(warp_fs::file("vov/kairobot_ui/index.html"));

    let routes = index_route.or(status_route()).or(forward_packet).or(ui_routes);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 4040))
        .await;
}