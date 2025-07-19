//! src/kairo_daemon/src/main.rs
//! The primary agent interface, handling P-address assignment and secure packet reception.

use warp::{Filter, Rejection, Reply, http::StatusCode};
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use kairo_lib::packet::AiTcpPacket;
use kairo_lib::config::AgentConfig;
use ed25519_dalek::{VerifyingKey, Signature, Verifier};

// --- Shared State ---
struct AddressPool { next_address: u8 }
static MESSAGE_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vec<AiTcpPacket>>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// --- Signature Verification Logic ---
fn verify_packet_signature(packet: &AiTcpPacket, registry: &[AgentConfig]) -> bool {
    // (This logic is copied from the old mesh-node and is assumed to be correct)
    let source_agent = match registry.iter().find(|a| a.p_address == packet.source_p_address) {
        Some(agent) => agent, None => { println!("🔴 Signature Fail: Source agent not found."); return false; }
    };
    let public_key = match VerifyingKey::from_bytes(&hex::decode(&source_agent.public_key).unwrap()) {
        Ok(key) => key, Err(_) => { println!("🔴 Signature Fail: Invalid public key."); return false; }
    };
    let signature = match Signature::from_bytes(&hex::decode(&packet.signature).unwrap()) {
        Ok(sig) => sig, Err(_) => { println!("🔴 Signature Fail: Invalid signature format."); return false; }
    };
    public_key.verify(packet.payload.as_bytes(), &signature).is_ok()
}

// --- API Handlers ---
asnyc fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!("🔵 [SEND] Received POST: from={}, to={}", packet.source_p_address, packet.destination_p_address);
    let registry = kairo_lib::config::load_all_configs().unwrap_or_default();

    if verify_packet_signature(&packet, &registry) {
        println!("🟢 [SIGNATURE VERIFIED]");
        let mut queue = MESSAGE_QUEUE.lock().await;
        let inbox = queue.entry(packet.destination_p_address.clone()).or_insert_with(Vec::new);
        inbox.push(packet);
        Ok(warp::reply::with_status(warp::reply::json(&"packet_queued"), StatusCode::OK))
    } else {
        println!("🔴 [SIGNATURE INVALID] Packet REJECTED");
        Ok(warp::reply::with_status(warp::reply::json(&"invalid_signature"), StatusCode::UNAUTHORIZED))
    }
}

async fn handle_receive(p_address: String) -> Result<impl Reply, Rejection> {
    let mut queue = MESSAGE_QUEUE.lock().await;
    if let Some(inbox) = queue.get_mut(&p_address) {
        let packets = inbox.clone();
        inbox.clear();
        println!("🟡 [RECEIVE] Delivered {} packets to {}", packets.len(), p_address);
        Ok(warp::reply::json(&packets))
    } else {
        Ok(warp::reply::json(&Vec::<AiTcpPacket>::new()))
    }
}

async fn handle_request_address(pool: Arc<StdMutex<AddressPool>>) -> Result<impl Reply, Rejection> {
    let mut pool = pool.lock().unwrap();
    let addr = pool.next_address;
    pool.next_address += 1;
    let p_address = format!("10.0.0.{}", addr);
    println!("🛰️  [DAEMON] Assigned P-Address: {}", p_address);
    Ok(warp::reply::json(&p_address))
}

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");
    let pool = Arc::new(StdMutex::new(AddressPool { next_address: 1 }));

    let get_address = warp::post()
        .and(warp::path("request_address"))
        .and(warp::any().map(move || Arc::clone(&pool)))
        .and_then(handle_request_address);

    let send = warp::post().and(warp::path("send")).and(warp::body::json()).and_then(handle_send);
    let receive = warp::get().and(warp::path("receive")).and(warp::path::param()).and_then(handle_receive);

    let routes = get_address.or(send).or(receive);

    println!("Listening on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
