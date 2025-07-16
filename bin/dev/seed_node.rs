//! bin/dev/seed_node.rs
//! Seed Node with structured JSON persistence.

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RegisterRequest {
    agent_id: String,
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

#[tokio::main]
async fn main() {
    println!("KAIRO Seed Node [v2: Structured Registry] starting...");

    let db_lock = Arc::new(Mutex::new(()));

    let register_lock = Arc::clone(&db_lock);
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&register_lock)))
        .and_then(handle_registration);

    println!("Listening on http://127.0.0.1:8080/register");
    println!("Registrations will be saved to registry.json");
    let revoke_lock = Arc::clone(&db_lock);
    let revoke = warp::post()
        .and(warp::path("revoke"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&revoke_lock)))
        .and_then(handle_revocation);

    let routes = register.or(revoke);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
