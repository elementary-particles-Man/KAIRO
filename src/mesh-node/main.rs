//! mesh-node/src/main.rs
// (Existing use statements...)
use kairo_lib::packet::AiTcpPacket;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use chrono::{Utc};

// (Existing structs like AgentInfo, RegisterRequest, etc.)

// In-memory message queue, now stores full packets
static MESSAGE_QUEUE: once_cell::sync::Lazy<Arc<Mutex<std::collections::HashMap<String, Vec<AiTcpPacket>>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(std::collections::HashMap::new())));

// (Existing functions like read_registry, write_registry, handle_registration, etc.)

// --- AI-TCP Communication Handlers ---
async fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!("Received packet to: {}, from: {}", packet.destination_p_address, packet.source_p_address);
    let mut queue = MESSAGE_QUEUE.lock().await;
    let inbox = queue.entry(packet.destination_p_address.clone()).or_insert_with(Vec::new);
    inbox.push(packet);
    Ok(warp::reply::json(&"packet_queued"))
}

async fn handle_receive(p_address: String) -> Result<impl Reply, Rejection> {
    let mut queue = MESSAGE_QUEUE.lock().await;
    if let Some(inbox) = queue.get_mut(&p_address) {
        let messages = inbox.clone();
        inbox.clear();
        println!("Delivered {} packets to {}", messages.len(), p_address);
        Ok(warp::reply::json(&messages))
    } else {
        Ok(warp::reply::json(&Vec::<AiTcpPacket>::new()))
    }
}

#[tokio::main]
async fn main() {
    // (Existing setup...)

    let send = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and_then(handle_send);

    let receive = warp::get()
        .and(warp::path("receive"))
        .and(warp::path::param())
        .and_then(handle_receive);

    let routes = register.or(revoke).or(reissue).or(emergency_reissue).or(send).or(receive);

    warp::serve(routes).run(([127, 0, 0, 1], 8082)).await;
}

// ここに実際のWarpエンドポイントのテストコードを追加する（内容省略）
