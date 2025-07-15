// D:\dev\KAIRO\rust-core\src\coordination\api.rs
use serde::{Deserialize, Serialize};
use warp::Filter;
use std::sync::Arc;

use super::node_manager::{NodeManager, Node};

#[derive(Debug, Serialize, Deserialize)]
struct RegisterRequest {
    public_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterResponse {
    virtual_ip: String,
}

pub fn register_route(manager: Arc<NodeManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(with_manager(manager))
        .and_then(handle_register)
}

pub fn peers_route(manager: Arc<NodeManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("peers"))
        .and(warp::query::<PeerQuery>())
        .and(with_manager(manager))
        .and_then(handle_peers)
}

#[derive(Debug, Serialize, Deserialize)]
struct PeerQuery {
    id: String,
}

fn with_manager(manager: Arc<NodeManager>) -> impl Filter<Extract = (Arc<NodeManager>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}

async fn handle_register(req: RegisterRequest, manager: Arc<NodeManager>) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: VoV logging of registration event
    let node = manager.register_node(req.public_key);
    let ip = node.map(|n| n.virtual_ip).unwrap_or_default();
    Ok(warp::reply::json(&RegisterResponse { virtual_ip: ip }))
}

async fn handle_peers(query: PeerQuery, manager: Arc<NodeManager>) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: authentication and VoV logging
    let peers: Vec<Node> = manager.get_peers(&query.id);
    Ok(warp::reply::json(&peers))
}
