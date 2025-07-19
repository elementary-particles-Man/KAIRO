use std::convert::Infallible;
use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct RegistrationRequest {
    agent_id: String,
    public_key: String,
}

#[derive(Debug, Serialize)]
struct RegistrationResponse {
    p_address: String,
}

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");

    // /status ルート
    let status_route = warp::path("status")
        .and(warp::get())
        .and_then(handle_status);

    // /register ルート
    let register_route = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_register);

    let routes = status_route.or(register_route);

    println!("Listening on http://127.0.0.1:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_status() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&json!({
        "status": "ok"
    })))
}

async fn handle_register(req: RegistrationRequest) -> Result<impl warp::Reply, Infallible> {
    let generated_address = format!("p-{}", Uuid::new_v4().to_simple());
    println!(
        "Registered agent: {} => KAIRO-P address assigned: {}",
        req.agent_id, generated_address
    );

    let response = RegistrationResponse {
        p_address: generated_address,
    };

    Ok(warp::reply::json(&response))
}
