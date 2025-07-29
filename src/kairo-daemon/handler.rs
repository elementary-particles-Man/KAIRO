// src/kairo-daemon/handler.rs

use axum::{Json, extract::Json as ExtractJson};
use serde_json::Value;

pub async fn handle_send(ExtractJson(payload): ExtractJson<Value>) -> Json<Value> {
    println!("[SEND] Received: {:?}", payload);
    Json(payload)
}

pub async fn handle_gpt(ExtractJson(payload): ExtractJson<Value>) -> Json<Value> {
    println!("[GPT] Received: {:?}", payload);
    Json(payload)
}
