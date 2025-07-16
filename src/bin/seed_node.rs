//! src/bin/seed_node.rs
//! The actual Seed Node server implementation.

use warp::Filter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    agent_id: String,
}

#[tokio::main]
async fn main() {
    println!("KAIRO Seed Node starting...");

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .map(|req: RegisterRequest| {
            println!("Received registration for agent_id: {}", req.agent_id);
            warp::reply::json(&"success")
        });

    println!("Listening on http://127.0.0.1:8080/register");
    warp::serve(register).run(([127, 0, 0, 1], 8080)).await;
}
