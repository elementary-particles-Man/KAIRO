//! src/bot/api/status.rs
// The API endpoint for querying task statuses.

use crate::bot::core::{Task, TaskQueue};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

pub fn create_status_route(queue: Arc<Mutex<TaskQueue>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("status"))
        .and(warp::any().map(move || Arc::clone(&queue)))
        .and_then(status_handler)
}

async fn status_handler(queue: Arc<Mutex<TaskQueue>>) -> Result<impl warp::Reply, warp::Rejection> {
    println!("API: Status request received.");
    let q = queue.lock().await;
    Ok(warp::reply::json(&q.tasks))
}
