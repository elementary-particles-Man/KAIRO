use chrono::{Local, TimeZone};
use serde::Serialize;
use std::{collections::HashMap, fs::OpenOptions, io::Write, net::Ipv4Addr};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Peer {
    pub uuid: Uuid,
    pub mesh_ip: Ipv4Addr,
    pub public_key: String,
}

pub struct CoordinationNode {
    pub node_id: Uuid,
    peers: HashMap<Uuid, Peer>,
}

impl CoordinationNode {
    pub fn new_from_env() -> Self {
        let node_id = std::env::var("KAIRO_NODE_ID")
            .ok()
            .and_then(|v| Uuid::parse_str(&v).ok())
            .unwrap_or_else(Uuid::new_v4);
        Self { node_id, peers: HashMap::new() }
    }

    pub fn add_peer(&mut self, public_key: String) -> Peer {
        let uuid = Uuid::new_v4();
        let ip_suffix = (self.peers.len() + 1) as u8;
        let mesh_ip = Ipv4Addr::new(100, 64, 0, ip_suffix);
        let peer = Peer { uuid, mesh_ip, public_key };
        self.peers.insert(uuid, peer.clone());
        self.log_event("peer_added", Some(uuid), Some(mesh_ip));
        peer
    }

    pub fn remove_peer(&mut self, uuid: &Uuid) -> bool {
        if let Some(peer) = self.peers.remove(uuid) {
            self.log_event("peer_removed", Some(peer.uuid), Some(peer.mesh_ip));
            true
        } else {
            false
        }
    }

    pub fn get_peers(&self) -> &HashMap<Uuid, Peer> {
        &self.peers
    }

    pub fn log_event(&self, event: &str, peer: Option<Uuid>, ip: Option<Ipv4Addr>) {
        let dir = std::path::Path::new("logs");
        std::fs::create_dir_all(dir).ok();
        let file_name = format!(
            "CoordinationNode_{}.log",
            Local::now().format("%Y%m%d")
        );
        let path = dir.join(file_name);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("open log file");
        let entry = EventLog {
            node_id: self.node_id.to_string(),
            event: event.to_string(),
            peer_uuid: peer.map(|p| p.to_string()),
            mesh_ip: ip.map(|p| p.to_string()),
            timestamp: Local.timestamp_millis(Local::now().timestamp_millis()).to_rfc3339(),
        };
        let line = serde_json::to_string(&entry).unwrap();
        let _ = writeln!(file, "{}", line);
    }
}

#[derive(Serialize)]
struct EventLog {
    node_id: String,
    timestamp: String,
    event: String,
    peer_uuid: Option<String>,
    mesh_ip: Option<String>,
}

