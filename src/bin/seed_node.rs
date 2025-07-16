//! src/bin/seed_node.rs
//! The actual Seed Node server implementation with JSON persistence and duplicate checking.

use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RegisterRequest {
    agent_id: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    status: String,
    message: String,
}

const DB_FILE: &str = "registered_agents.json";

// Reads agent IDs from the JSON file.
fn read_agents() -> Result<Vec<String>, std::io::Error> {
    let file = File::open(DB_FILE).unwrap_or_else(|_| File::create(DB_FILE).unwrap());
    let reader = BufReader::new(file);
    let agents = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(agents)
}

// Writes the list of agent IDs to the JSON file.
fn write_agents(agents: &[String]) -> std::io::Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(DB_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, agents)?;
    Ok(())
}

async fn handle_registration(req: RegisterRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    println!("Received registration for agent_id: {}", req.agent_id);

    let mut agents = read_agents().expect("Failed to read from DB");

    if agents.contains(&req.agent_id) {
        println!("Agent {} already registered.", req.agent_id);
        let res = RegisterResponse { status: "exists".to_string(), message: "Agent already registered".to_string() };
        Ok(warp::reply::json(&res))
    } else {
        agents.push(req.agent_id.clone());
        write_agents(&agents).expect("Failed to write to DB");
        println!("Successfully registered agent {}.", req.agent_id);
        let res = RegisterResponse { status: "success".to_string(), message: "Agent successfully registered".to_string() };
        Ok(warp::reply::json(&res))
    }
}

#[tokio::main]
async fn main() {
    println!("KAIRO Seed Node starting...");

    let db_lock = Arc::new(Mutex::new(()));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&db_lock)))
        .and_then(handle_registration);

    println!("Listening on http://127.0.0.1:8080/register");
    println!("Registrations will be saved to registered_agents.json");
    warp::serve(register).run(([127, 0, 0, 1], 8080)).await;
}

