//! bin/dev/seed_node_mock.rs
//! Simple Seed Node Mock API Server using warp

use warp::Filter;
use serde_json::Value;

#[tokio::main]
async fn main() {
    // POST /register endpoint
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .map(|body: Value| {
            println!("✅ Received registration request: {:?}", body);
            warp::reply::json(&serde_json::json!({
                "status": "success",
                "message": "Registered successfully!"
            }))
        });

    println!("🚀 Seed Node Mock API running at http://localhost:8080/register");
    warp::serve(register).run(([127, 0, 0, 1], 8080)).await;
}
