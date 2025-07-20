//! mesh-node/src/main.rs

mod seed_node;

use kairo_lib::packet::AiTcpPacket;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};
use crate::seed_node::{read_registry, AgentInfo};
use std::sync::Arc;

// (Existing structs like AgentInfo, RegisterRequest, etc.)

// --- Signature Verification Logic ---
// Validate packet authenticity by checking the sender's ed25519 signature.
use ed25519_dalek::{VerifyingKey, Signature, Verifier};

fn verify_packet_signature(packet: &AiTcpPacket, registry: &[AgentInfo]) -> bool {
    let source_agent = match registry.iter().find(|a| a.public_key == packet.source_public_key) {
        Some(agent) => agent,
        None => {
            println!("Signature Fail: Source agent {} not found in registry.", packet.source_public_key);
            return false;
        }
    };

    let public_key_bytes = match hex::decode(&source_agent.public_key) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let public_key = match VerifyingKey::try_from(public_key_bytes.as_slice()) {
        Ok(key) => key,
        Err(_) => return false,
    };

    let signature_bytes = match hex::decode(&packet.signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    // The signature is created over the payload only for this implementation
    public_key.verify(packet.payload.as_bytes(), &signature).is_ok()
}

// In-memory message queue, now stores full packets
static MESSAGE_QUEUE: once_cell::sync::Lazy<Arc<Mutex<std::collections::HashMap<String, Vec<AiTcpPacket>>>>> = once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(std::collections::HashMap::new())));

// (Existing functions like read_registry, write_registry, handle_registration, etc.)

// --- AI-TCP Communication Handlers ---
async fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!("Received packet to: {}, from: {}", packet.destination_p_address, packet.source_public_key);
    let mut queue = MESSAGE_QUEUE.lock().await;
    let registry = read_registry().expect("DB read error during send");
    if verify_packet_signature(&packet, &registry) {
        println!("Signature VERIFIED for packet from {}", packet.source_public_key);
        let inbox = queue.entry(packet.destination_p_address.clone()).or_insert_with(Vec::new);
        inbox.push(packet);
    } else {
        println!("Signature FAILED for packet from {}", packet.source_public_key);
        // Do not queue the packet if signature is invalid
    }
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

    let seed_routes = seed_node::routes();
    let routes = seed_routes.or(send).or(receive);

    warp::serve(routes).run(([127, 0, 0, 1], 8082)).await;
}

// ここに実際のWarpエンドポイントのテストコードを追加する（内容省略）
