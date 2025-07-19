use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    from: String,
    to: String,
    message: String,
    signature: String,
}

type PAddress = String;
type MessageQueue = Arc<Mutex<HashMap<PAddress, Vec<Message>>>>;

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");

    let message_queues: MessageQueue = Arc::new(Mutex::new(HashMap::new()));
    let queues_filter = warp::any().map(move || message_queues.clone());

    let send = warp::path("send")
        .and(warp::post())
        .and(warp::body::json())
        .and(queues_filter.clone())
        .map(|message: Message, queues: MessageQueue| {
            println!("\n🔵 [SEND] Received POST: from={}, to={}, message={}\n", message.from, message.to, message.message);

            let mut queues = queues.lock().unwrap();
            println!("🟢 [SEND] Queuing message for {}", message.to);

            queues.entry(message.to.clone()).or_insert_with(Vec::new).push(message);
            warp::reply::json(&serde_json::json!({ "status": "Message queued" }))
        });

    let receive = warp::path("receive")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(queues_filter.clone())
        .map(|params: HashMap<String, String>, queues: MessageQueue| {
            let p_address = params.get("for").cloned().unwrap_or_default();
            println!("\n🟡 [RECEIVE] Request for {}\n", p_address);

            let mut queues = queues.lock().unwrap();
            let messages = queues.remove(&p_address).unwrap_or_default();
            warp::reply::json(&messages)
        });

    let routes = send.or(receive);

    println!("Listening on http://127.0.0.1:3030\n");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}