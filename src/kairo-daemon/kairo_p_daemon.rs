use std::collections::{HashMap, VecDeque};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::fs::{self, File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use warp::Filter;

use kairo_lib::packet::AiTcpPacket;

const STORE_DIR: &str = "message_store";
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Agent {
    p_address: String,
}

type AgentList = Arc<Mutex<HashMap<String, Agent>>>;
type MessageQueues = Arc<Mutex<HashMap<String, VecDeque<AiTcpPacket>>>>;

fn load_persistent_messages(queues: &MessageQueues) {
    if !Path::new(STORE_DIR).exists() {
        if let Err(e) = fs::create_dir_all(STORE_DIR) {
            eprintln!("Failed to create message store directory: {}", e);
            return;
        }
    }

    if let Ok(entries) = fs::read_dir(STORE_DIR) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(file) = File::open(&path) {
                        let reader = BufReader::new(file);
                        if let Ok(messages) = serde_json::from_reader::<_, Vec<AiTcpPacket>>(reader) {
                            let mut lock = queues.lock().unwrap();
                            lock.insert(stem.to_string(), VecDeque::from(messages));
                        }
                    }
                }
            }
        }
    }
}

fn persist_queue(p_address: &str, queue: &VecDeque<AiTcpPacket>) {
    if !Path::new(STORE_DIR).exists() {
        if let Err(e) = fs::create_dir_all(STORE_DIR) {
            eprintln!("Failed to create message store directory: {}", e);
            return;
        }
    }

    let path = format!("{}/{}.json", STORE_DIR, p_address);
    if let Ok(file) = OpenOptions::new().write(true).truncate(true).create(true).open(&path) {
        if let Err(e) = serde_json::to_writer_pretty(&file, &queue.iter().cloned().collect::<Vec<_>>()) {
            eprintln!("Failed to write message store for {}: {}", p_address, e);
        }
    } else {
        eprintln!("Failed to open message store file for {}", p_address);
    }
}

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");

    let agent_list: AgentList = Arc::new(Mutex::new(HashMap::new()));
    let message_queues: MessageQueues = Arc::new(Mutex::new(HashMap::new()));
    load_persistent_messages(&message_queues);

    let get_address = warp::path("request_address")
        .and(warp::get())
        .and(with_agent_list(agent_list.clone()))
        .and(with_message_queues(message_queues.clone()))
        .and_then(assign_p_address);

    let send_message = warp::path("send")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_message_queues(message_queues.clone()))
        .and_then(handle_send);

    let receive_message = warp::path("receive")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_message_queues(message_queues.clone()))
        .and_then(handle_receive);

    let routes = get_address.or(send_message).or(receive_message);

    let addr: SocketAddr = ([127, 0, 0, 1], 3030).into();
    warp::serve(routes).run(addr).await;
}

fn with_agent_list(
    agent_list: AgentList,
) -> impl Filter<Extract = (AgentList,), Error = Infallible> + Clone {
    warp::any().map(move || agent_list.clone())
}

fn with_message_queues(
    buffer: MessageQueues,
) -> impl Filter<Extract = (MessageQueues,), Error = Infallible> + Clone {
    warp::any().map(move || buffer.clone())
}

async fn assign_p_address(
    agent_list: AgentList,
    buffer: MessageQueues,
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

async fn handle_send(
    packet: AiTcpPacket,
    buffer: MessageQueues,
) -> Result<impl warp::Reply, Infallible> {
    {
        let mut buffers = buffer.lock().unwrap();
        let queue = buffers
            .entry(packet.destination_p_address.clone())
            .or_insert_with(VecDeque::new);
        queue.push_back(packet.clone());
        persist_queue(&packet.destination_p_address, queue);
    }
    Ok(warp::reply::json(&serde_json::json!({ "status": "queued" })))
}

#[derive(Debug, Deserialize)]
struct ReceiveRequest {
    p_address: String,
}

async fn handle_receive(
    req: ReceiveRequest,
    buffer: MessageQueues,
) -> Result<impl warp::Reply, Infallible> {
    let mut buffers = buffer.lock().unwrap();
    if let Some(queue) = buffers.get_mut(&req.p_address) {
        if let Some(message) = queue.pop_front() {
            persist_queue(&req.p_address, queue);
            return Ok(warp::reply::json(&message));
        }
    }

    Ok(warp::reply::json(&serde_json::json!({
        "status": "no_message"
    })))
}
