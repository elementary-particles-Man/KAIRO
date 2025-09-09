use axum::{extract::State, Json, http::StatusCode};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::task_queue::{TaskQueue, Task};

#[derive(Deserialize, Debug, Clone)]
pub struct AddTaskRequest {
    pub id: String,
    pub name: String,
    pub command: String,
}

pub async fn add_task(
    State(queue): State<Arc<Mutex<TaskQueue>>>,
    Json(req): Json<AddTaskRequest>,
) -> (StatusCode, &'static str) {
    {
        let mut q = queue.lock().expect("TaskQueue lock");
        q.add_task(Task { id: req.id.clone(), name: req.name.clone(), command: req.command.clone() });
    }
    println!("[ADD_TASK] id={} name={} command={}", req.id, req.name, req.command);
    (StatusCode::OK, "ok")
}
