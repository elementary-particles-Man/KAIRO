use warp::{Filter, Rejection, Reply};
use tokio::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;
use kairo_lib::packet::AiTcpPacket;
use kairo_lib::config::AgentConfig;
use serde_json::from_reader;
use std::fs::File;

mod seed_node;

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

// --- Signature Verification Logic ---
use ed25519_dalek::{VerifyingKey, Signature, Verifier};

fn verify_packet_signature(packet: &kairo_lib::packet::AiTcpPacket, registry: &[AgentConfig]) -> bool {
    let source_agent = match registry.iter().find(|a| a.p_address == packet.source_p_address) {
        Some(agent) => agent,
        None => {
            println!("Signature Fail: Source agent {} not found in registry.", packet.source_p_address);
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

static MESSAGE_QUEUE: Lazy<Arc<Mutex<std::collections::HashMap<String, Vec<AiTcpPacket>>>>> = Lazy::new(|| Arc::new(Mutex::new(std::collections::HashMap::new())));

async fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!("Received packet to: {}, from: {}", packet.destination_p_address, packet.source_p_address);
    let mut queue = MESSAGE_QUEUE.lock().await;
    let registry = read_configs().expect("Config read error during send");
    if verify_packet_signature(&packet, &registry) {
        println!("Signature VERIFIED for packet from {}", packet.source_p_address);
        let inbox = queue.entry(packet.destination_p_address.clone()).or_insert_with(Vec::new);
        inbox.push(packet);
    } else {
        println!("Signature FAILED for packet from {}: Packet REJECTED.", packet.source_p_address);
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
