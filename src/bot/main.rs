//! src/bot/main.rs
// The main entry point for the KAIROBOT.

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::bot::core::{TaskQueue, main_loop};
use crate::bot::api::{receiver, status};
use warp::Filter;

use kairo_lib::config::{self, AgentConfig};
use kairo_lib::packet::AiTcpPacket;
use ed25519_dalek::{Signer, SigningKey};
use chrono::Utc;

#[derive(Debug, serde::Deserialize)]
struct UiSendRequest {
    to_p_address: String,
    payload: String,
}

async fn handle_ui_send(req: UiSendRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let cli_config = match config::load_config_from_dir("./users/CLI") {
        Some(c) => c,
        None => return Ok(warp::reply::with_status(warp::reply::json(&"CLI agent config not found"), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    };

    let secret_key_bytes = hex::decode(&cli_config.secret_key).unwrap();
    let signing_key = SigningKey::from_bytes(&secret_key_bytes);

    let message_to_sign = req.payload.as_bytes();
    let signature = signing_key.sign(message_to_sign);

    let packet = AiTcpPacket {
        version: 1,
        source_p_address: cli_config.p_address,
        destination_p_address: req.to_p_address,
        sequence: 0,
        timestamp_utc: Utc::now().timestamp(),
        payload_type: "text/plain".to_string(),
        payload: req.payload,
        signature: hex::encode(signature.to_bytes()),
    };

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3030/send").json(&packet).send().await;

    match res {
        Ok(response) => Ok(warp::reply::json(&response.status().as_u16())),
        Err(_) => Ok(warp::reply::json(&"Failed to forward packet to daemon"))
    }
}

#[tokio::main]
async fn main() {
    println!("KAIROBOT: Starting bootstrap process...");

    let task_queue = Arc::new(Mutex::new(TaskQueue::load()));

    // Start the API server in a separate task
    let api_task_queue = Arc::clone(&task_queue);
    let api_server = tokio::spawn(async move {
        let add_task_route = receiver::create_task_route(Arc::clone(&api_task_queue));
        let status_route = status::create_status_route(Arc::clone(&api_task_queue));
        let health_check_route = warp::path::end().map(|| warp::reply::json(&"KAIROBOT is alive"));
        let ui_route = warp::path("ui").and(warp::fs::dir("./vov/kairobot_ui/"));
        let send_signed_route = warp::post().and(warp::path("send_signed")).and(warp::body::json()).and_then(handle_ui_send);
        let routes = add_task_route.or(status_route).or(ui_route).or(health_check_route).or(send_signed_route);
        println!("KAIROBOT API: Listening on http://127.0.0.1:4040");
        warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
    });

    // Start the core processing loop
    let core_loop = tokio::spawn(async move {
        main_loop(task_queue).await;
    });

    // Keep the bot alive
    let _ = tokio::try_join!(api_server, core_loop);
}
