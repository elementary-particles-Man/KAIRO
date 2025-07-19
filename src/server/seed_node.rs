//! bin/dev/seed_node.rs
//! Seed Node with structured JSON persistence.

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;

use kairo_lib::governance::OverridePackage;

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
    registered_at: String,
    status: String, // e.g., "active", "revoked"
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

async fn handle_registration(req: RegisterRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    println!("Received registration for agent_id: {}", req.agent_id);

    let mut registry = read_registry().expect("Failed to read from DB");

    if registry.iter().any(|agent| agent.agent_id == req.agent_id && agent.status == "active") {
        println!("Active agent {} already registered.", req.agent_id);
        let res = RegisterResponse { status: "exists".to_string(), message: "Active agent already registered".to_string() };
        Ok(warp::reply::json(&res))
    } else {
        let new_agent = AgentInfo {
            agent_id: req.agent_id.clone(),
            registered_at: Utc::now().to_rfc3339(),
            status: "active".to_string(),
            replaces: None,
        };
        registry.push(new_agent);
        write_registry(&registry).expect("Failed to write to DB");
        println!("Successfully registered agent {}.", req.agent_id);
        let res = RegisterResponse { status: "success".to_string(), message: "Agent successfully registered".to_string() };
        Ok(warp::reply::json(&res))
    }
}

// Handler for revoking an agent's ID
async fn handle_revocation(req: RegisterRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    println!("Received revocation request for agent_id: {}", req.agent_id);

    let mut registry = read_registry().expect("Failed to read from DB");

    if let Some(agent) = registry.iter_mut().find(|a| a.agent_id == req.agent_id && a.status == "active") {
        agent.status = "revoked".to_string();
        write_registry(&registry).expect("Failed to write to DB");
        println!("Successfully revoked agent {}.", req.agent_id);
        let res = RegisterResponse { status: "success".to_string(), message: "Agent successfully revoked".to_string() };
        Ok(warp::reply::json(&res))
    } else {
        println!("Active agent {} not found for revocation.", req.agent_id);
        let res = RegisterResponse { status: "not_found".to_string(), message: "Active agent not found".to_string() };
        Ok(warp::reply::json(&res))
    }
}

// Handler for reissuing an agent's ID
async fn handle_reissue(req: ReissueRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    println!("Received reissue request for old_agent_id: {}", req.old_agent_id);

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
        registered_at: Utc::now().to_rfc3339(),
        status: "active".to_string(),
        replaces: Some(req.old_agent_id.clone()),
    };
    registry.push(new_agent);
    write_registry(&registry).expect("Failed to write to DB");

    println!("Successfully reissued agent ID {} to {}", req.old_agent_id, req.new_agent_id);
    let res = RegisterResponse { status: "success".to_string(), message: "Agent ID successfully reissued".to_string() };
    Ok(warp::reply::with_status(warp::reply::json(&res), warp::http::StatusCode::OK))
}

// Handler for emergency reissuance by the governance quorum
async fn handle_emergency_reissue(req: OverridePackage) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Received emergency reissue request.");
    // TODO: 1. Verify the multiplicity and diversity of signatures.
    // TODO: 2. Verify each signature against the payload.
    // TODO: 3. If valid, execute the reissuance logic after a cooldown.
    Ok(warp::reply::json(&"received"))
}

#[tokio::main]
async fn main() {
    println!("KAIRO Seed Node [v2: Structured Registry] starting...");

    let db_lock = Arc::new(Mutex::new(()));

    let register_lock = Arc::clone(&db_lock);
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(warp::any().map({
            let register_lock = Arc::clone(&register_lock);
            move || Arc::clone(&register_lock)
        }))
        .and_then(handle_registration);

    let revoke_lock = Arc::clone(&db_lock);
    let revoke = warp::post()
        .and(warp::path("revoke"))
        .and(warp::body::json())
        .and(warp::any().map({
            let revoke_lock = Arc::clone(&revoke_lock);
            move || Arc::clone(&revoke_lock)
        }))
        .and_then(handle_revocation);

    let reissue_lock = Arc::clone(&db_lock);
    let reissue = warp::post()
        .and(warp::path("reissue"))
        .and(warp::body::json())
        .and(warp::any().map({
            let reissue_lock = Arc::clone(&reissue_lock);
            move || Arc::clone(&reissue_lock)
        }))
        .and_then(handle_reissue);

    let emergency_reissue = warp::post()
        .and(warp::path("emergency_reissue"))
        .and(warp::body::json())
        .and_then(handle_emergency_reissue);

    let routes = register.or(revoke).or(reissue).or(emergency_reissue);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}