mod gpt_responder;
mod gpt_log_processor;
mod kairo_p_listener;
mod p_signature_validator;
mod p_structure_filter;
mod config;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex as StdMutex};
use chrono::Utc;

use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter, Rejection, Reply};

use log::{info, LevelFilter, error};
use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config, TerminalMode, ColorChoice};


use crate::packet::Packet;
use crate::p_signature_validator;
use crate::gpt_responder;
use kairo_lib::packet::AiTcpPacket;
use serde::{Deserialize, Serialize};
use clap::Parser;

use kairo_lib::config::{load_daemon_config, DaemonConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".kairo/config/daemon_config.json")]
    config: String,
}


/// Simple pool for issuing incremental P-addresses.
struct AddressPool {
    next_address: u8,
}

/// Global in-memory message queue indexed by destination P-address.
static MESSAGE_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vec<AiTcpPacket>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

static LAST_SEEN_SEQUENCE: Lazy<Arc<Mutex<HashMap<String, u64>>>> =
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



/// Verify that a packet's signature matches the sending agent's public key.
fn _verify_packet_signature(packet: &AiTcpPacket, registry: &[AgentInfo]) -> bool {
    let source_agent = match registry
        .iter()
        .find(|a| a.public_key == packet.source_public_key) // public_key を使用
    {
        Some(agent) => agent,
        None => {
            info!("🔴 Signature Fail: Source agent not found in registry for public key: {}", packet.source_public_key);
            return false;
        }
    };

    let public_key_bytes = match hex::decode(&source_agent.public_key) {
        Ok(bytes) => bytes,
        Err(_) => {
            info!("🔴 Signature Fail: Invalid public key format in registry.");
            return false;
        }
    };

    let public_key = match public_key_bytes.as_slice().try_into() {
        Ok(bytes) => match VerifyingKey::from_bytes(bytes) {
            Ok(key) => key,
            Err(e) => {
                info!("🔴 Signature Fail: Invalid public key bytes from registry. Error: {:?}", e);
                return false;
            }
        },
        Err(e) => {
            info!("🔴 Signature Fail: Public key bytes not 32 bytes long. Error: {:?}", e);
            return false;
        }
    };

    let signature_bytes = match hex::decode(&packet.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            info!("🔴 Signature Fail: Invalid signature format.");
            return false;
        }
    };

    let signature_array: [u8; 64] = match signature_bytes.as_slice().try_into() {
        Ok(arr) => arr,
        Err(e) => {
            info!("🔴 Signature Fail: Invalid signature byte length. Error: {:?}", e);
            return false;
        }
    };

    let signature = Signature::from_bytes(&signature_array);

    let message_to_verify = [
        &packet.sequence.to_le_bytes()[..],
        &packet.timestamp_utc.to_le_bytes()[..],
        packet.payload.as_bytes(),
    ].concat();

    public_key
        .verify(&message_to_verify, &signature)
        .is_ok()
}

/// Handle POST /send_packet
async fn handle_send(packet: Packet) -> Result<impl Reply, Rejection> {
    info!("DEBUG: handle_send called");
    info!("\u{1f535} [SEND] Received POST: from_public_key={}, to={}", packet.source_p_address, packet.destination_p_address);
    info!("DEBUG: packet.destination_p_address = {:?}", packet.destination_p_address);

    // 署名検証（現段階では常に true）
    let valid = p_signature_validator::validate(&packet);
    if !valid {
        error!("\u{274c} Invalid signature from {}", packet.source_p_address);
        return Ok(warp::reply::with_status("Forbidden", warp::http::StatusCode::FORBIDDEN));
    }

    if packet.destination_p_address == "gpt://main" {
        // GPT 処理を同期的に実行し、HTTP応答を即時返却
        match gpt_responder::gpt_log_and_respond(&packet).await {
            Ok(resp) => {
                info!("\u{2705} [GPT] Response delivered");
                Ok(warp::reply::with_status(resp.as_str(), warp::http::StatusCode::OK))
            },
            Err(e) => {
                error!("\u{274c} [GPT] Failed to handle packet: {}", e);
                Ok(warp::reply::with_status("Internal Server Error", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    } else {
        error!("\u{274c} Unsupported destination: {}", packet.destination_p_address);
        Ok(warp::reply::with_status("Not Implemented", warp::http::StatusCode::NOT_IMPLEMENTED))
    }
}

/// Deliver all queued packets for the requested P-address.
async fn handle_receive(p_address: String) -> Result<impl Reply, Rejection> {
    info!("🔵 [RECEIVE] Request received for P-address: {}", p_address);
    let mut queue = MESSAGE_QUEUE.lock().await;
    if let Some(inbox) = queue.get_mut(&p_address) {
        let packets = inbox.clone();
        inbox.clear();
        info!(
            "🟡 [RECEIVE] Delivered {} packets to {}",
            packets.len(),
            p_address
        );
        Ok(warp::reply::json(&packets))
    } else {
        info!("🟡 [RECEIVE] No inbox found for P-address: {}", p_address);
        Ok(warp::reply::json(&Vec::<AiTcpPacket>::new()))
    }
}

/// Assign a new P-address from the pool.
async fn assign_p_address_handler(req: AssignPAddressRequest, pool: Arc<StdMutex<AddressPool>>) -> Result<impl Reply, Rejection> {
    let mut registry = read_agent_registry().unwrap_or_default();

    // Check if public_key already exists in registry
    if let Some(existing_agent) = registry.iter().find(|agent| agent.public_key == req.public_key) {
        info!("🟢 [DAEMON] Public Key already registered: {} with P-Address: {}", req.public_key, existing_agent.p_address);
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({ "public_key": req.public_key, "p_address": existing_agent.p_address })),
            StatusCode::OK,
        ));
    }

    let mut pool_guard = pool.lock().unwrap();
    let addr = pool_guard.next_address;
    pool_guard.next_address += 1;
    let p_address = format!("10.0.0.{}/24", addr); // CIDR表記を追加

    let new_agent_info = AgentInfo {
        public_key: req.public_key.clone(),
        p_address: p_address.clone(),
        registered_at: Utc::now().to_rfc3339(),
        status: "active".to_string(),
    };

    registry.push(new_agent_info);
    write_agent_registry(&registry).expect("Failed to write agent registry");

    info!("🟢 [DAEMON] Assigned NEW P-Address: {} for public_key: {}", p_address, req.public_key);
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "public_key": req.public_key, "p_address": p_address })),
        StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // simplelog の初期化
    let log_file = File::create("kairo_daemon.log").expect("Failed to create log file");
    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Debug,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Debug,
                Config::default(),
                log_file,
            ),
        ]
    ).expect("Failed to initialize logger");

    info!("KAIRO-P Daemon starting...");
    info!("Loading configuration from: {}", args.config);

    let config = load_daemon_config(".kairo/.config/daemon_config.json")
        .unwrap_or_else(|_| {
            info!("WARN: daemon_config.json not found or invalid. Falling back to default bootstrap address.");
            DaemonConfig {
                listen_address: "127.0.0.1".to_string(),
                listen_port: 3030,
            }
        });
    let initial_next_address = {
        let registry = read_agent_registry().unwrap_or_default();
        let max_p_address_num = registry.iter()
            .filter_map(|agent| {
                agent.p_address.split('/').next() // "10.0.0.X/24" から "10.0.0.X" を取得
                    .and_then(|s| s.split('.').last()) // "10.0.0.X" から "X" を取得
                    .and_then(|s| s.parse::<u8>().ok()) // "X" を数値に変換
            })
            .max()
            .unwrap_or(0); // 登録がない場合は0
        max_p_address_num + 1
    };
    let pool = Arc::new(StdMutex::new(AddressPool { next_address: initial_next_address }));

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
        .and(warp::query::<HashMap<String, String>>())
        .and_then(|params: HashMap<String, String>| async move {
            if let Some(p_address) = params.get("for") {
                handle_receive(p_address.clone()).await
            } else {
                Err(warp::reject::not_found())
            }
        });

    let routes = assign_p_address.or(send).or(receive);

    let listen_addr: std::net::IpAddr = config.listen_address.parse().expect("Invalid listen address");

    info!("Listening for address requests on http://{}:{}", config.listen_address, config.listen_port);
    tokio::spawn(async move {
        let _ = kairo_p_listener::run_listener().await;
    });
    warp::serve(routes).run((listen_addr, config.listen_port)).await;
}