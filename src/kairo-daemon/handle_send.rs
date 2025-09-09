use warp::Filter;
use log::{info, error};
use kairo_lib::packet::Packet;

/// Handle POST /send_packet
pub async fn handle_send(packet: Packet) -> Result<impl warp::Reply, warp::Rejection> {
    info!("DEBUG: handle_send called");
    info!("🔵 [SEND] Received POST: from_public_key={}, to={}", packet.source_p_address, packet.destination_p_address);
    info!("DEBUG: packet.destination_p_address = {:?}", packet.destination_p_address);

    // 署名検証（現段階では常に true）
    let valid = crate::p_signature_validator::validate(&packet);
    if !valid {
        error!("❌ Invalid signature from {}", packet.source_p_address);
        return Ok(warp::reply::with_status("Forbidden", warp::http::StatusCode::FORBIDDEN));
    }

    if packet.destination_p_address == "gpt://main" {
        match crate::gpt_responder::gpt_log_and_respond(&packet).await {
            Ok(resp) => {
                info!("✅ [GPT] Response delivered");
                Ok(warp::reply::with_status(resp.as_str(), warp::http::StatusCode::OK))
            },
            Err(e) => {
                error!("❌ [GPT] Failed to handle packet: {}", e);
                Ok(warp::reply::with_status("Internal Server Error", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    } else {
        error!("❌ Unsupported destination: {}", packet.destination_p_address);
        Ok(warp::reply::with_status("Not Implemented", warp::http::StatusCode::NOT_IMPLEMENTED))
    }
}
