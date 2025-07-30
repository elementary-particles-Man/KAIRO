//! src/bot/main.rs
// The main entry point for the KAIROBOT.

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::bot::core::{TaskQueue, main_loop};
use crate::bot::api::{receiver, status};

#[tokio::main]
async fn main() {
    // Force logger initialization to ensure visibility
    simple_logger::init_with_level(log::Level::Info).expect("Failed to initialize logger");

    println!("KAIROBOT: Starting bootstrap process...");

    let task_queue = Arc::new(Mutex::new(TaskQueue::load()));

    // Start the core processing loop as a background task
    let core_task_queue = Arc::clone(&task_queue);
    tokio::spawn(async move {
        main_loop(core_task_queue).await;
    });

    // Define API routes
    let add_task_route = receiver::create_task_route(Arc::clone(&task_queue));
    let status_route = status::create_status_route(Arc::clone(&task_queue));
    let routes = add_task_route.or(status_route);

    println!("KAIROBOT API: Listening on http://127.0.0.1:4040");

    // Start the API server in the foreground, keeping the main process alive
    warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
}
