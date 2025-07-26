//! src/kairo_daemon/src/main.rs
//! The primary agent interface, handling P-address assignment and secure packet reception.

use warp::{Filter, Rejection, Reply, http::StatusCode};
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use kairo_lib::packet::AiTcpPacket;
use kairo_lib::config::{self, AgentConfig};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};

// --- Shared State ---
struct AddressPool { next_address: u8 }
static MESSAGE_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vec<AiTcpPacket>>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// --- Signature Verification Logic (Final Version) ---
fn verify_packet_signature(packet: &AiTcpPacket, registry: &[AgentConfig]) -> bool {
    let source_agent = match registry.iter().find(|a| a.p_address == packet.source_p_address) {
        Some(agent) => agent,
        None => {
            println!(" Signature Fail: Source agent not found in registry for p_address {}.", packet.source_p_address);
            return false;
        }
    };

    let public_key_bytes = match hex::decode(&source_agent.public_key) {
        Ok(bytes) => bytes,
        Err(_) => { println!(" Signature Fail: Could not decode public key from agent config."); return false; }
    };

    let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => { println!(" Signature Fail: Invalid public key format for {}.", source_agent.p_address); return false; }
    };

    let signature_bytes = match hex::decode(&packet.signature) {
        Ok(bytes) => bytes,
        Err(_) => { println!(" Signature Fail: Could not decode signature from packet."); return false; }
    };
    
    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => { println!(" Signature Fail: Invalid signature format in packet."); return false; }
    };

    // The signature must be verified against the payload.
    public_key.verify(packet.payload.as_bytes(), &signature).is_ok()
}

// --- API Handlers ---
async fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!(" [SEND] Received POST: from={}, to={}", packet.source_p_address, packet.destination_p_address);
    let registry = config::load_all_configs().unwrap_or_default();

    if verify_packet_signature(&packet, &registry) {
        println!(" [SIGNATURE VERIFIED]");
        let mut queue = MESSAGE_QUEUE.lock().await;
        let inbox = queue.entry(packet.destination_p_address.clone()).or_insert_with(Vec::new);
        inbox.push(packet);
        Ok(warp::reply::with_status(warp::reply::json(&"packet_queued"), StatusCode::OK))
    } else {
        println!(" [SIGNATURE INVALID] Packet REJECTED");
        Ok(warp::reply::with_status(warp::reply::json(&"signature_invalid"), StatusCode::BAD_REQUEST))
    }
}

async fn handle_receive(p_address: String) -> Result<impl Reply, Rejection> {
    println!(" [RECEIVE] Received GET for p_address={}", p_address);
    let mut queue = MESSAGE_QUEUE.lock().await;
    let packets = queue.remove(&p_address).unwrap_or_else(Vec::new);
    Ok(warp::reply::json(&packets))
}

async fn handle_request_address(pool: Arc<StdMutex<AddressPool>>) -> Result<impl Reply, Rejection> {
    let mut pool = pool.lock().unwrap();
    let p_address = format!("10.0.0.{}", pool.next_address);
    pool.next_address += 1;
    println!(" [ADDRESS] Assigned p_address: {}", p_address);
    Ok(warp::reply::json(&p_address))
}

#[tokio::main]
async fn main() {
    println!("KAIRO Daemon starting...");
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