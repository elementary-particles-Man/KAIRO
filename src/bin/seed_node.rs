//! src/bin/seed_node.rs
//! The actual Seed Node server implementation with persistence.

use warp::Filter;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
struct RegisterRequest {
    agent_id: String,
}

// A simple function to append the agent_id to a file.
fn persist_agent(agent_id: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("registered_agents.txt")?;

    writeln!(file, "{}", agent_id)
}

#[tokio::main]
async fn main() {
    println!("KAIRO Seed Node starting...");

    // Wrap the persistence logic in a mutex for safe concurrent access if needed in the future.
    let log_file = Arc::new(Mutex::new(()));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&log_file)))
        .and_then(|req: RegisterRequest, _lock: Arc<Mutex<()>>| async move {
            println!("Received registration for agent_id: {}", req.agent_id);
            if let Err(e) = persist_agent(&req.agent_id) {
                eprintln!("Error persisting agent: {}", e);
                return Err(warp::reject::custom(RegistrationError));
            }
            Ok(warp::reply::json(&"success"))
        });

    println!("Listening on http://127.0.0.1:8080/register");
    println!("Registrations will be saved to registered_agents.txt");
    warp::serve(register).run(([127, 0, 0, 1], 8080)).await;
}

#[derive(Debug)]
struct RegistrationError;
impl warp::reject::Reject for RegistrationError {}
