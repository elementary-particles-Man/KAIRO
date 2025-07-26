// UDP経由Meshノードへパケット送信（mesh_nodeとペア）
use std::net::UdpSocket;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct MeshPacket {
    from: String,
    to: String,
    payload: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <from> <to> <payload>", args[0]);
        return;
    }
    let packet = MeshPacket {
        from: args[1].clone(),
        to: args[2].clone(),
        payload: args[3].clone(),
    };
    let json = serde_json::to_vec(&packet).unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(&json, "127.0.0.1:5050").unwrap();
    println!("✅ Sent packet to mesh_node.");
}
