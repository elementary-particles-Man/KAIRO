//! bin/dev/seed_node.rs
//! Seed Node with structured JSON persistence + signed packet validation.

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use kairo_lib::packet::AiTcpPacket;
use ed25519_dalek::{VerifyingKey, Signature, Verifier};

use kairo_lib::governance::OverridePackage;
use kairo_lib::config::DaemonConfig;
use std::fs;

fn load_node_config(path: &str) -> Result<DaemonConfig, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: DaemonConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}


#[derive(Debug, Deserialize, Serialize, Clone)]
struct RegisterRequest {
    agent_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ReissueRequest {
    old_agent_id: String,
    new_agent_id: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    status: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AgentInfo {
    agent_id: String,
    public_key: String,
    registered_at: String,
    status: String,
    replaces: Option<String>,
}

const DB_FILE: &str = "registry.json";

fn read_registry() -> Result<Vec<AgentInfo>, std::io::Error> {
    let file = File::open(DB_FILE).unwrap_or_else(|_| File::create(DB_FILE).unwrap());
    let reader = BufReader::new(file);
    let registry = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(registry)
}

fn write_registry(registry: &[AgentInfo]) -> std::io::Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(DB_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, registry)?;
    Ok(())
}

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

    let public_key = match VerifyingKey::try_from(&public_key_bytes[..]) {
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

    public_key.verify(packet.payload.as_bytes(), &signature).is_ok()
}

static MESSAGE_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vec<AiTcpPacket>>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

async fn handle_send(packet: AiTcpPacket) -> Result<impl Reply, Rejection> {
    println!("Received packet to: {}, from: {}", packet.destination_p_address, packet.source_public_key);
    let mut queue = MESSAGE_QUEUE.lock().await;
    let registry = read_registry().expect("Config read error during send");
    if verify_packet_signature(&packet, &registry) {
        println!("Signature VERIFIED for packet from {}", packet.source_public_key);
        let inbox = queue.entry(packet.destination_p_address.clone()).or_insert_with(Vec::new);
        inbox.push(packet);
    } else {
        println!("Signature FAILED for packet from {}: Packet REJECTED.", packet.source_public_key);
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

async fn handle_registration(req: RegisterRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    println!("Received registration for agent_id: {}", req.agent_id);
    let mut registry = read_registry().expect("Failed to read from DB");
    if registry.iter().any(|agent| agent.agent_id == req.agent_id && agent.status == "active") {
        let res = RegisterResponse { status: "exists".to_string(), message: "Active agent already registered".to_string() };
        Ok(warp::reply::json(&res))
    } else {
        let new_agent = AgentInfo {
            agent_id: req.agent_id.clone(),
            public_key: req.agent_id.clone(), // agent_id を public_key として使用
            registered_at: Utc::now().to_rfc3339(),
            status: "active".to_string(),
            replaces: None,
        };
        registry.push(new_agent);
        write_registry(&registry).expect("Failed to write to DB");
        let res = RegisterResponse { status: "success".to_string(), message: "Agent successfully registered".to_string() };
        Ok(warp::reply::json(&res))
    }
}

async fn handle_revocation(req: RegisterRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    let mut registry = read_registry().expect("Failed to read from DB");
    if let Some(agent) = registry.iter_mut().find(|a| a.agent_id == req.agent_id && a.status == "active") {
        agent.status = "revoked".to_string();
        write_registry(&registry).expect("Failed to write to DB");
        let res = RegisterResponse { status: "success".to_string(), message: "Agent successfully revoked".to_string() };
        Ok(warp::reply::json(&res))
    } else {
        let res = RegisterResponse { status: "not_found".to_string(), message: "Active agent not found".to_string() };
        Ok(warp::reply::json(&res))
    }
}

async fn handle_reissue(req: ReissueRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    let mut registry = read_registry().expect("Failed to read from DB");
    if let Some(old_agent) = registry.iter().find(|a| a.agent_id == req.old_agent_id) {
        if old_agent.status != "revoked" {
            let res = RegisterResponse { status: "error".to_string(), message: "Old agent is not revoked".to_string() };
            return Ok(warp::reply::with_status(warp::reply::json(&res), warp::http::StatusCode::BAD_REQUEST));
        }
    } else {
        let res = RegisterResponse { status: "not_found".to_string(), message: "Old agent not found".to_string() };
        return Ok(warp::reply::with_status(warp::reply::json(&res), warp::http::StatusCode::NOT_FOUND));
    }
    if registry.iter().any(|a| a.agent_id == req.new_agent_id && a.status == "active") {
        let res = RegisterResponse { status: "exists".to_string(), message: "New agent ID already exists as an active agent".to_string() };
        return Ok(warp::reply::with_status(warp::reply::json(&res), warp::http::StatusCode::CONFLICT));
    }
    let new_agent = AgentInfo {
        agent_id: req.new_agent_id.clone(),
        public_key: req.new_agent_id.clone(), // new_agent_id を public_key として使用
        registered_at: Utc::now().to_rfc3339(),
        status: "active".to_string(),
        replaces: Some(req.old_agent_id.clone()),
    };
    registry.push(new_agent);
    write_registry(&registry).expect("Failed to write to DB");
    let res = RegisterResponse { status: "success".to_string(), message: "Agent ID successfully reissued".to_string() };
    Ok(warp::reply::with_status(warp::reply::json(&res), warp::http::StatusCode::OK))
}

async fn handle_emergency_reissue(req: OverridePackage, db_lock: Arc<Mutex<()>>) -> Result<impl warp::Reply, warp::Rejection> {
    let _lock = db_lock.lock().await;
    println!("Received emergency reissue request.");

    let registry = read_registry().expect("Failed to read from DB");
    let agent_configs = read_registry().expect("Failed to read agent configs");

    // 1. JSONで受け取ったOverridePackageをパース (warp::body::json() で既にパース済み)
    let payload_bytes = serde_json::to_vec(&req.payload)
        .map_err(|_| warp::reject::custom(BadRequest("Failed to serialize payload".to_string())))?;

    // 2. 各署名に対応する公開鍵で署名妥当性を検証
    for sig_package in &req.signatures {
        let signatory_id = &sig_package.signatory_id;
        let signature_hex = &sig_package.signature;

        // 署名者IDが実在し、`agent_registry` に登録済であること
        let signatory_agent = agent_configs.iter().find(|a| a.agent_id == *signatory_id)
            .ok_or_else(|| warp::reject::custom(BadRequest(format!("Signatory ID not found: {}", signatory_id))))?;

        let public_key_bytes: [u8; 32] = hex::decode(&signatory_agent.public_key)
            .map_err(|_| warp::reject::custom(BadRequest("Invalid public key format".to_string())))?
            .try_into()
            .map_err(|_| warp::reject::custom(BadRequest("Invalid public key length".to_string())))?;
        let public_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|_| warp::reject::custom(BadRequest("Invalid public key format".to_string())))?;

        let signature_bytes = hex::decode(signature_hex)
            .map_err(|_| warp::reject::custom(BadRequest("Invalid signature format".to_string())))?;
        let signature = Signature::try_from(signature_bytes.as_slice())
            .map_err(|_| warp::reject::custom(BadRequest("Invalid signature format".to_string())))?;

        if public_key.verify(&payload_bytes, &signature).is_err() {
            return Err(warp::reject::custom(BadRequest(format!("Invalid signature for signatory: {}", signatory_id))));
        }
    }

    // 3. 新旧IDに整合性があるか（reissue元の失効ID→新IDへの正しい変遷か）
    let old_agent_id = &req.payload.old_agent_id;
    let new_agent_id = &req.payload.new_agent_id;

    // old_agent_id が `agent_registry` に存在し、`revoked` 状態であること
    let old_agent_exists_and_revoked = registry.iter().any(|a| a.agent_id == *old_agent_id && a.status == "revoked");
    if !old_agent_exists_and_revoked {
        return Err(warp::reject::custom(BadRequest(format!("Old agent ID '{}' not found or not revoked.", old_agent_id))));
    }

    // new_agent_id が `agent_registry` に存在しない、または `active` 状態でないこと
    let new_agent_exists_and_active = registry.iter().any(|a| a.agent_id == *new_agent_id && a.status == "active");
    if new_agent_exists_and_active {
        return Err(warp::reject::custom(BadRequest(format!("New agent ID '{}' already exists and is active.", new_agent_id))));
    }

    // 正常受理時: 200 OK + {"status": "override_package accepted"}
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "status": "override_package accepted" })),
        warp::http::StatusCode::OK,
    ))
}

// Custom Rejection for Bad Request
#[derive(Debug)]
struct BadRequest(String);

impl warp::reject::Reject for BadRequest {}

// Rejection handler
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = err.find::<BadRequest>() {
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({ "status": "error", "message": e.0 })),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        // Fallback to default error handling
        Err(err)
    }
}

#[tokio::main]
async fn main() {
    println!("KAIRO Seed Node [Unified: Registry + Messaging] starting...");

    let config = load_node_config(".kairo/config/node_config.json").unwrap_or_else(|e| {
        eprintln!("Failed to load node_config.json: {}", e);
        DaemonConfig { listen_address: "10.0.0.254".to_string(), listen_port: 8000 }
    });

    println!("Listening on {}:{}", config.listen_address, config.listen_port);

    let db_lock = Arc::new(Mutex::new(()));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(warp::any().map({
            let register_lock = Arc::clone(&db_lock);
            move || Arc::clone(&register_lock)
        }))
        .and_then(handle_registration);

    let revoke = warp::post()
        .and(warp::path("revoke"))
        .and(warp::body::json())
        .and(warp::any().map({
            let revoke_lock = Arc::clone(&db_lock);
            move || Arc::clone(&revoke_lock)
        }))
        .and_then(handle_revocation);

    let reissue = warp::post()
        .and(warp::path("reissue"))
        .and(warp::body::json())
        .and(warp::any().map({
            let reissue_lock = Arc::clone(&db_lock);
            move || Arc::clone(&reissue_lock)
        }))
        .and_then(handle_reissue);

    let emergency_reissue = warp::post()
        .and(warp::path("emergency_reissue"))
        .and(warp::body::json())
        .and(warp::any().map({
            let emergency_reissue_lock = Arc::clone(&db_lock);
            move || Arc::clone(&emergency_reissue_lock)
        }))
        .and_then(handle_emergency_reissue);

    let send = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and_then(handle_send);

    let receive = warp::get()
        .and(warp::path("receive"))
        .and(warp::path::param())
        .and_then(handle_receive);

    let routes = register.or(revoke).or(reissue).or(emergency_reissue).or(send).or(receive).recover(handle_rejection);

    warp::serve(routes).run((config.listen_address.parse::<std::net::IpAddr>().unwrap(), config.listen_port)).await;
}
