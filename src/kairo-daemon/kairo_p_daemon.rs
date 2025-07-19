use std::collections::HashMap;
use std::fs::File;
use std::sync::{Arc, Mutex as StdMutex};

use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter, Rejection, Reply};

use kairo_lib::packet::AiTcpPacket;
use kairo_lib::AgentConfig;
use serde_json::from_reader;

/// Simple pool for issuing incremental P-addresses.
struct AddressPool {
    next_address: u8,
}

/// Global in-memory message queue indexed by destination P-address.
static MESSAGE_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vec<AiTcpPacket>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Load all agent configuration files from `agent_configs/` directory.
fn read_configs() -> Result<Vec<AgentConfig>, std::io::Error> {
    let mut configs = Vec::new();
    if let Ok(entries) = std::fs::read_dir("agent_configs") {
        for entry in entries {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file = File::open(&path)?;
                if let Ok(cfg) = from_reader(file) {
                    configs.push(cfg);
                }
            }
        }
    }
    Ok(configs)
}

/// Verify that a packet's signature matches the sending agent's public key.
fn verify_packet_signature(packet: &AiTcpPacket, registry: &[AgentConfig]) -> bool {
    let source_agent = match registry
        .iter()
        .find(|a| a.p_address == packet.source_p_address)
    {
        Some(agent) => agent,
        None => {
            println!("\u{1f534} Signature Fail: Source agent not found.");
            return false;
        }
    };

    let public_key_bytes = match hex::decode(&source_agent.public_key) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("\u{1f534} Signature Fail: Invalid public key.");
            return false;
        }
    };

    let public_key = match VerifyingKey::from_bytes(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => {
            println!("\u{1f534} Signature Fail: Invalid public key bytes.");
            return false;
        }
    };

    let signature_bytes = match hex::decode(&packet.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("\u{1f534} Signature Fail: Invalid signature format.");
            return false;
        }
    };

    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            println!("\u{1f534} Signature Fail: Invalid signature bytes.");
            return false;
        }
    };

    public_key
        .verify(packet.payload.as_bytes(), &signature)
        .is_ok()
}

/// Handle an incoming packet POST request.
async fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!(
        "\u{1f539} [SEND] Received POST: from={}, to={}",
        packet.source_p_address, packet.destination_p_address
    );
    let registry = read_configs().unwrap_or_default();

    if verify_packet_signature(&packet, &registry) {
        println!("\u{1f7e2} [SIGNATURE VERIFIED]");
        let mut queue = MESSAGE_QUEUE.lock().await;
        let inbox = queue
            .entry(packet.destination_p_address.clone())
            .or_insert_with(Vec::new);
        inbox.push(packet);
        Ok(warp::reply::with_status(
            warp::reply::json(&"packet_queued"),
            StatusCode::OK,
        ))
    } else {
        println!("\u{1f534} [SIGNATURE INVALID] Packet REJECTED");
        Ok(warp::reply::with_status(
            warp::reply::json(&"invalid_signature"),
            StatusCode::UNAUTHORIZED,
        ))
    }
}

/// Deliver all queued packets for the requested P-address.
async fn handle_receive(p_address: String) -> Result<impl Reply, Rejection> {
    let mut queue = MESSAGE_QUEUE.lock().await;
    if let Some(inbox) = queue.get_mut(&p_address) {
        let packets = inbox.clone();
        inbox.clear();
        println!(
            "\u{1f7e1} [RECEIVE] Delivered {} packets to {}",
            packets.len(),
            p_address
        );
        Ok(warp::reply::json(&packets))
    } else {
        Ok(warp::reply::json(&Vec::<AiTcpPacket>::new()))
    }
}

/// Assign a new P-address from the pool.
async fn handle_request_address(pool: Arc<StdMutex<AddressPool>>) -> Result<impl Reply, Rejection> {
    let mut pool = pool.lock().unwrap();
    let addr = pool.next_address;
    pool.next_address += 1;
    let p_address = format!("10.0.0.{}", addr);
    println!("\u{1f6f8}  [DAEMON] Assigned P-Address: {}", p_address);
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

    let send = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and_then(handle_send);

    let receive = warp::get()
        .and(warp::path("receive"))
        .and(warp::path::param())
        .and_then(handle_receive);

    let routes = get_address.or(send).or(receive);

    println!("Listening on http://127.0.0.1:8082");
    warp::serve(routes).run(([127, 0, 0, 1], 8082)).await;
}
