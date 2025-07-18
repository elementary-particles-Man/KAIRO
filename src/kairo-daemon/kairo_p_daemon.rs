use std::collections::{HashMap, VecDeque};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::Filter;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Agent {
    p_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    from_p_address: String,
    to_p_address: String,
    payload: String,
}

type AgentList = Arc<Mutex<HashMap<String, Agent>>>;
type MessageBuffer = Arc<Mutex<HashMap<String, VecDeque<Message>>>>;

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");

    let agent_list: AgentList = Arc::new(Mutex::new(HashMap::new()));
    let message_buffer: MessageBuffer = Arc::new(Mutex::new(HashMap::new()));

    let get_address = warp::path("request_address")
        .and(warp::get())
        .and(with_agent_list(agent_list.clone()))
        .and(with_message_buffer(message_buffer.clone()))
        .and_then(assign_p_address);

    let receive_message = warp::path("receive")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_message_buffer(message_buffer.clone()))
        .and_then(handle_receive);

    let routes = get_address.or(receive_message);

    let addr: SocketAddr = ([127, 0, 0, 1], 3030).into();
    warp::serve(routes).run(addr).await;
}

fn with_agent_list(
    agent_list: AgentList,
) -> impl Filter<Extract = (AgentList,), Error = Infallible> + Clone {
    warp::any().map(move || agent_list.clone())
}

fn with_message_buffer(
    buffer: MessageBuffer,
) -> impl Filter<Extract = (MessageBuffer,), Error = Infallible> + Clone {
    warp::any().map(move || buffer.clone())
}

async fn assign_p_address(
    agent_list: AgentList,
    buffer: MessageBuffer,
) -> Result<impl warp::Reply, Infallible> {
    let mut agents = agent_list.lock().unwrap();
    let mut counter = 1;
    while agents.contains_key(&format!("10.0.0.{}", counter)) {
        counter += 1;
    }

    let p_address = format!("10.0.0.{}", counter);
    println!("Assigned P-Address: {}", p_address);

    let agent = Agent {
        p_address: p_address.clone(),
    };

    agents.insert(p_address.clone(), agent.clone());

    // Initialize message buffer
    let mut buffers = buffer.lock().unwrap();
    buffers.entry(p_address.clone()).or_insert_with(VecDeque::new);

    Ok(warp::reply::json(&agent))
}

#[derive(Debug, Deserialize)]
struct ReceiveRequest {
    p_address: String,
}

async fn handle_receive(
    req: ReceiveRequest,
    buffer: MessageBuffer,
) -> Result<impl warp::Reply, Infallible> {
    let mut buffers = buffer.lock().unwrap();
    if let Some(queue) = buffers.get_mut(&req.p_address) {
        if let Some(message) = queue.pop_front() {
            return Ok(warp::reply::json(&message));
        }
    }

    Ok(warp::reply::json(&serde_json::json!({
        "status": "no_message"
    })))
}
