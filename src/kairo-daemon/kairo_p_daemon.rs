//! src/kairo-daemon/kairo_p_daemon.rs

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::Filter;

use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use rand::Rng;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    from_p_address: String,
    to_p_address: String,
    payload: String,
}

type MessageQueue = Arc<Mutex<HashMap<String, VecDeque<Message>>>>;
type AssignedAddresses = Arc<Mutex<Vec<String>>>;

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");

    let message_queue: MessageQueue = Arc::new(Mutex::new(HashMap::new()));
    let assigned_addresses: AssignedAddresses = Arc::new(Mutex::new(Vec::new()));

    let request_address = warp::post()
        .and(warp::path("request_address"))
        .and(with_addresses(assigned_addresses.clone()))
        .map(assign_address);

    let send = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and(with_queue(message_queue.clone()))
        .map(handle_send);

    let receive = warp::post()
        .and(warp::path("receive"))
        .and(warp::body::json())
        .and(with_queue(message_queue.clone()))
        .map(handle_receive);

    let routes = request_address.or(send).or(receive);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_queue(
    queue: MessageQueue,
) -> impl Filter<Extract = (MessageQueue,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || queue.clone())
}

fn with_addresses(
    addresses: AssignedAddresses,
) -> impl Filter<Extract = (AssignedAddresses,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || addresses.clone())
}

fn assign_address(addresses: AssignedAddresses) -> impl warp::Reply {
    let mut addr_list = addresses.lock().unwrap();
    let mut rng = rand::thread_rng();
    let mut new_addr;
    loop {
        let last_byte: u8 = rng.gen_range(1..=254);
        new_addr = format!("10.0.0.{}", last_byte);
        if !addr_list.contains(&new_addr) {
            addr_list.push(new_addr.clone());
            break;
        }
    }
    println!("Assigned P-Address: {}", new_addr);
    warp::reply::json(&new_addr)
}

fn handle_send(message: Message, queue: MessageQueue) -> impl warp::Reply {
    let mut q = queue.lock().unwrap();
    q.entry(message.to_p_address.clone())
        .or_insert_with(VecDeque::new)
        .push_back(message.clone());

    println!(
        "Stored message from {} to {}",
        message.from_p_address, message.to_p_address
    );

    save_message_to_disk(&message);

    warp::reply::json(&json!({"status": "message_sent"}))
}

fn handle_receive(body: serde_json::Value, queue: MessageQueue) -> impl warp::Reply {
    let p_address = match body.get("p_address").and_then(|v| v.as_str()) {
        Some(addr) => addr.to_string(),
        None => return warp::reply::json(&json!({"error": "Invalid request"})),
    };

    let mut q = queue.lock().unwrap();
    if let Some(messages) = q.get_mut(&p_address) {
        if let Some(message) = messages.pop_front() {
            return warp::reply::json(&message);
        }
    }
    warp::reply::json(&json!({"status": "no_message"}))
}

fn save_message_to_disk(message: &Message) {
    let log_dir = Path::new("message_logs");
    if !log_dir.exists() {
        if let Err(e) = fs::create_dir_all(log_dir) {
            eprintln!("Failed to create log directory: {}", e);
            return;
        }
    }

    let file_path = log_dir.join("message_log.json");
    let mut log = if file_path.exists() {
        match fs::read_to_string(&file_path) {
            Ok(content) => serde_json::from_str::<Vec<Message>>(&content).unwrap_or_else(|_| Vec::new()),
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    };

    log.push(message.clone());

    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
    {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", serde_json::to_string_pretty(&log).unwrap_or_default()) {
                eprintln!("Failed to write message log: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to open message log file: {}", e);
        }
    }
}
