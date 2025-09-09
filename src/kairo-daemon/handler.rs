// src/kairo-daemon/handler.rs

use axum::{Json, extract::Json as ExtractJson};
use kairo_lib::packet::AiTcpPacket;

pub async fn handle_send(ExtractJson(packet): ExtractJson<AiTcpPacket>) -> Json<String> {
    println!("[SEND] From {} to {}: {}", packet.source, packet.destination, packet.payload);
    Json(format!("Packet relayed to {}", packet.destination))
}

pub async fn handle_gpt(ExtractJson(packet): ExtractJson<AiTcpPacket>) -> Json<String> {
    println!("[GPT] From {} to {}: {}", packet.source, packet.destination, packet.payload);
    Json(format!("GPT processed for {}", packet.destination))
}
