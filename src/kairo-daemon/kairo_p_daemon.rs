use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex as StdMutex};
use chrono::Utc;

use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter, Rejection, Reply};

use kairo_lib::packet::AiTcpPacket;
use kairo_lib::AgentConfig;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct DaemonConfig {
    #[clap(long, default_value = "127.0.0.1")]
    listen_address: String,

    #[clap(long, default_value_t = 3030)]
    listen_port: u16,
}

/// Simple pool for issuing incremental P-addresses.
struct AddressPool {
    next_address: u8,
}

/// Global in-memory message queue indexed by destination P-address.
static MESSAGE_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vec<AiTcpPacket>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AssignPAddressRequest {
    public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AgentInfo {
    public_key: String,
    p_address: String,
    registered_at: String,
    status: String,
}

const AGENT_REGISTRY_FILE: &str = "agent_registry.json";

fn read_agent_registry() -> Result<Vec<AgentInfo>, std::io::Error> {
    let file = File::open(AGENT_REGISTRY_FILE).unwrap_or_else(|_| File::create(AGENT_REGISTRY_FILE).unwrap());
    let reader = BufReader::new(file);
    let registry = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(registry)
}

fn write_agent_registry(registry: &[AgentInfo]) -> std::io::Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(AGENT_REGISTRY_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, registry)?;
    Ok(())
}

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
            println!("🔴 Signature Fail: Source agent not found.");
            return false;
        }
    };

    let public_key_bytes = match hex::decode(&source_agent.public_key) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("🔴 Signature Fail: Invalid public key.");
            return false;
        }
    };

    let public_key = match VerifyingKey::from_bytes(public_key_bytes.as_slice().try_into().unwrap()) {
        Ok(key) => key,
        Err(_) => {
            println!("🔴 Signature Fail: Invalid public key bytes.");
            return false;
        }
    };

    let signature_bytes = match hex::decode(&packet.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("🔴 Signature Fail: Invalid signature format.");
            return false;
        }
    };

    let signature_array: [u8; 64] = match signature_bytes.as_slice().try_into() {
        Ok(arr) => arr,
        Err(_) => {
            println!("🔴 Signature Fail: Invalid signature byte length.");
            return false;
        }
    };

    let signature = Signature::from_bytes(&signature_array);

    public_key
        .verify(packet.payload.as_bytes(), &signature)
        .is_ok()
}

/// Handle an incoming packet POST request.
async fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!(
        "🔵 [SEND] Received POST: from={}, to={}",
        packet.source_p_address, packet.destination_p_address
    );
    let registry = read_configs().unwrap_or_default();

    if verify_packet_signature(&packet, &registry) {
        println!("🟢 [SIGNATURE VERIFIED]");
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
        println!("🔴 [SIGNATURE INVALID] Packet REJECTED");
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
            "🟡 [RECEIVE] Delivered {} packets to {}",
            packets.len(),
            p_address
        );
        Ok(warp::reply::json(&packets))
    } else {
        Ok(warp::reply::json(&Vec::<AiTcpPacket>::new()))
    }
}

/// Assign a new P-address from the pool.
async fn assign_p_address_handler(req: AssignPAddressRequest, pool: Arc<StdMutex<AddressPool>>) -> Result<impl Reply, Rejection> {
    let mut registry = read_agent_registry().unwrap_or_default();

    // Check for duplicate public_key
    if registry.iter().any(|agent| agent.public_key == req.public_key) {
        println!("🔴 [DAEMON] Public Key already exists: {}", req.public_key);
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({ "status": "error", "message": "Public Key already exists" })),
            StatusCode::CONFLICT,
        ));
    }

    let mut pool_guard = pool.lock().unwrap();
    let addr = pool_guard.next_address;
    pool_guard.next_address += 1;
    let p_address = format!("10.0.0.{}", addr);

    let new_agent_info = AgentInfo {
        public_key: req.public_key.clone(),
        p_address: p_address.clone(),
        registered_at: Utc::now().to_rfc3339(),
        status: "active".to_string(),
    };

    registry.push(new_agent_info);
    write_agent_registry(&registry).expect("Failed to write agent registry");

    println!("🟢 [DAEMON] Assigned P-Address: {} for public_key: {}", p_address, req.public_key);
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "public_key": req.public_key, "p_address": p_address })),
        StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {
    let config = DaemonConfig::parse();
    println!("KAIRO-P Daemon starting...");
    let pool = Arc::new(StdMutex::new(AddressPool { next_address: 1 }));

    let assign_p_address = warp::post()
        .and(warp::path("assign_p_address"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&pool)))
        .and_then(assign_p_address_handler);

    let send = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and_then(handle_send);

    let receive = warp::get()
        .and(warp::path("receive"))
        .and(warp::path::param())
        .and_then(handle_receive);

    let routes = assign_p_address.or(send).or(receive);

    let listen_addr: std::net::IpAddr = config.listen_address.parse().expect("Invalid listen address");

    println!("Listening for address requests on http://{}:{}", config.listen_address, config.listen_port);
    warp::serve(routes).run((listen_addr, config.listen_port)).await;
}