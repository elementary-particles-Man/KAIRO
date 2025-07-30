//! src/bot/main.rs
// The main entry point for the KAIROBOT, with corrected startup logic.

use std::sync::Arc;
use tokio::sync::Mutex;
// Corrected path to the bot's core logic in the 'kairo_core' crate.
use kairo_core::bot::core::{TaskQueue, main_loop};
use crate::bot::api::{receiver, status};
use warp::Filter;

#[tokio::main]
async fn main() {
    // Initialize logger to ensure visibility of all subsequent logs.
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init().expect("Failed to initialize logger");

    log::info!("KAIROBOT: Starting bootstrap process...");

    // Load the persistent task queue.
    let task_queue = Arc::new(Mutex::new(TaskQueue::load()));

    // --- Start Core Logic in the Background ---
    let core_task_queue = Arc::clone(&task_queue);
    tokio::spawn(async move {
        main_loop(core_task_queue).await;
    });

    // --- Start API Server in the Foreground ---
    let add_task_route = receiver::create_task_route(Arc::clone(&task_queue));
    let status_route = status::create_status_route(Arc::clone(&task_queue));
    let health_check_route = warp::path::end().map(|| warp::reply::json(&"KAIROBOT is alive"));
    let routes = add_task_route.or(status_route).or(health_check_route);

    log::info!("KAIROBOT API: Listening on http://127.0.0.1:4040");

    // Run the server in the foreground. This keeps the main process alive.
    warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
}
