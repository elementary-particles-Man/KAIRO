//! src/bot/main.rs
// The main entry point for the KAIROBOT.

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::bot::core::{TaskQueue, main_loop};
use crate::bot::api::receiver;

#[tokio::main]
async fn main() {
    println!("KAIROBOT: Starting bootstrap process...");

    let task_queue = Arc::new(Mutex::new(TaskQueue::new()));

    // Start the API server in a separate task
    let api_task_queue = Arc::clone(&task_queue);
    let api_server = tokio::spawn(async move {
        let routes = receiver::create_task_route(api_task_queue);
        println!("KAIROBOT API: Listening on http://127.0.0.1:4040");
        warp::serve(routes).run(([127, 0, 0, 1], 4040)).await;
    });

    // Start the core processing loop
    let core_loop = tokio::spawn(async move {
        main_loop(task_queue).await;
    });

    // Keep the bot alive
    let _ = tokio::try_join!(api_server, core_loop);
}
