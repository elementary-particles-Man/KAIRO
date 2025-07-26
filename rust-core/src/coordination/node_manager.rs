// D:\dev\KAIRO\rust-core\src\coordination\node_manager.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String, // 128-bit Unique ID represented as hex string
    pub public_key: Vec<u8>,
    pub virtual_ip: String,
}

pub struct NodeManager {
    pub nodes: Arc<Mutex<HashMap<String, Node>>>,
}

impl NodeManager {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a new node and assign a virtual IP within 100.64.0.0/10.
    /// Returns the created [`Node`] on success.
    pub fn register_node(&self, public_key: Vec<u8>) -> Option<Node> {
        let id = Uuid::new_v4().as_simple().to_string();

        let mut nodes = self.nodes.lock().ok()?;

        // Allocate a virtual IP starting from 100.64.0.1
        let mut suffix: u16 = 1;
        loop {
            let third = (suffix / 256) as u8;
            let fourth = (suffix % 256) as u8;
            let ip = format!("100.64.{}.{}", third, fourth);
            if !nodes.values().any(|n| n.virtual_ip == ip) {
                let node = Node {
                    id: id.clone(),
                    public_key,
                    virtual_ip: ip.clone(),
                };
                nodes.insert(id.clone(), node.clone());
                return Some(node);
            }

            // Prevent infinite loops if IP space is exhausted
            if suffix == u16::MAX {
                break;
            }
            suffix += 1;
        }

        None
    }

    // TODO: return list of peers (public_key and virtual_ip) for authenticated node
    pub fn get_peers(&self, _id: &str) -> Vec<Node> {
        todo!("Return peer list for authenticated node");
    }
}