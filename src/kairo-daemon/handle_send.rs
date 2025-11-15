use clear_mini::api::ClearMini;
use clear_mini::detector::Window;
use clear_mini::kairo_p::PAddressRecord;
use kairo_lib::packet::Packet;
use log::{error, info, warn};
use once_cell::sync::Lazy;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Mutex;

static CLEAR_MINI: Lazy<Mutex<ClearMini>> = Lazy::new(|| Mutex::new(ClearMini::new()));
static DET_DST_10S: Lazy<Mutex<Window>> = Lazy::new(|| Mutex::new(Window::new(10)));

struct SendRequest {
    src_id: i32,
    src_nick: String,
    dst_ip: [u8; 16],
    dst_port: u16,
    route_flags: u32,
    payload_len: u32,
}

impl SendRequest {
    fn new(packet: &Packet, dst_ip: [u8; 16], dst_port: u16, route_flags: u32) -> Self {
        Self {
            src_id: derive_agent_id(&packet.source_p_address),
            src_nick: packet.source_p_address.clone(),
            dst_ip,
            dst_port,
            route_flags,
            payload_len: packet.payload.as_bytes().len() as u32,
        }
    }
}

fn derive_agent_id(p_address: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    p_address.hash(&mut hasher);
    (hasher.finish() & 0x7FFF_FFFF) as i32
}

fn parse_destination_endpoint(p_address: &str) -> ([u8; 16], u16) {
    let stripped = p_address.split("://").last().unwrap_or(p_address);
    if let Ok(socket) = stripped.parse::<SocketAddr>() {
        (encode_ip(socket.ip()), socket.port())
    } else if let Ok(ip) = stripped.parse::<IpAddr>() {
        (encode_ip(ip), 0)
    } else {
        ([0; 16], 0)
    }
}

fn encode_ip(ip: IpAddr) -> [u8; 16] {
    match ip {
        IpAddr::V4(v4) => {
            let octets = v4.octets();
            [octets[0], octets[1], octets[2], octets[3], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }
        IpAddr::V6(v6) => v6.octets(),
    }
}

fn record_witness(req: &SendRequest) {
    let mut cm = CLEAR_MINI.lock().unwrap();
    let src = PAddressRecord::new(req.src_id, &req.src_nick);
    let dst = PAddressRecord::new(0, "");
    cm.record(&src, &dst, req.payload_len, req.route_flags, req.dst_ip, req.dst_port);
}

fn detect_burst(dst_ip: [u8; 16], dst_port: u16) {
    let mut hasher = DefaultHasher::new();
    dst_ip.hash(&mut hasher);
    dst_port.hash(&mut hasher);
    let key = hasher.finish();

    let count = DET_DST_10S.lock().unwrap().hit(key);
    if count >= 50 {
        warn!(
            "AI/Burst PUT detected: {} hits/10s to {}:{}",
            count,
            format_ip(dst_ip),
            dst_port
        );
    }
}

fn format_ip(ip: [u8; 16]) -> String {
    if ip[4..].iter().any(|b| *b != 0) {
        Ipv6Addr::from(ip).to_string()
    } else {
        Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]).to_string()
    }
}

/// Handle POST /send_packet
pub async fn handle_send(packet: Packet) -> Result<impl warp::Reply, warp::Rejection> {
    info!(
        "🔵 [SEND] Received POST: from_public_key={}, to={}",
        packet.source_p_address, packet.destination_p_address
    );
    info!(
        "DEBUG: packet.destination_p_address = {:?}",
        packet.destination_p_address
    );

    let valid = crate::p_signature_validator::validate(&packet);
    if !valid {
        error!("❌ Invalid signature from {}", packet.source_p_address);
        return Ok(warp::reply::with_status(
            "Forbidden",
            warp::http::StatusCode::FORBIDDEN,
        ));
    }

    if packet.destination_p_address == "gpt://main" {
        match crate::gpt_responder::gpt_log_and_respond(&packet).await {
            Ok(resp_tuple) => {
                let (resp_str, actual_socket_addr) = resp_tuple;

                info!(
                    "✅ [GPT] Response delivered. Actual remote addr: {}",
                    actual_socket_addr
                );

                let dst_ip = encode_ip(actual_socket_addr.ip());
                let dst_port = actual_socket_addr.port();
                let route_flags = 1;

                let req = SendRequest::new(&packet, dst_ip, dst_port, route_flags);
                record_witness(&req);
                detect_burst(req.dst_ip, req.dst_port);

                Ok(warp::reply::with_status(
                    resp_str.as_str(),
                    warp::http::StatusCode::OK,
                ))
            }
            Err(e) => {
                error!("❌ [GPT] Failed to handle packet: {}", e);
                Ok(warp::reply::with_status(
                    "Internal Server Error",
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    } else {
        error!("❌ Unsupported destination: {}", packet.destination_p_address);

        let (dst_ip, dst_port) = parse_destination_endpoint(&packet.destination_p_address);
        let route_flags = 2;
        let req = SendRequest::new(&packet, dst_ip, dst_port, route_flags);
        record_witness(&req);
        detect_burst(req.dst_ip, req.dst_port);

        Ok(warp::reply::with_status(
            "Not Implemented",
            warp::http::StatusCode::NOT_IMPLEMENTED,
        ))
    }
}
