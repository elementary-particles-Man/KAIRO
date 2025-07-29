//! src/bot/api/receiver.rs
// The API endpoint for receiving new tasks.

use crate::bot::core::{Task, TaskQueue};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

pub fn create_task_route(queue: Arc<Mutex<TaskQueue>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("add_task"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&queue)))
        .and_then(add_task_handler)
}

async fn add_task_handler(task: Task, queue: Arc<Mutex<TaskQueue>>) -> Result<impl warp::Reply, warp::Rejection> {
    println!("API: Received new task -> {}", task.name);
    let mut q = queue.lock().await;
    q.add_task(task);
    Ok(warp::reply::json(&"Task added successfully"))
}
