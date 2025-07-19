use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgentMapping {
    agent_id: String,
    p_address: String,
}

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let assign_route = warp::path!("assign_address" / String)
        .and(with_db(db.clone()))
        .and_then(handle_assign_address);

    let receive_route = warp::path("receive")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handle_receive_messages);

    let routes = assign_route.or(receive_route);

    println!("Listening on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn handle_assign_address(agent_id: String, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db_lock = db.lock().unwrap();

    // 既に登録されていればそのアドレスを返す
    if let Some(addr) = db_lock.get(&agent_id) {
        return Ok(warp::reply::json(&AgentMapping {
            agent_id,
            p_address: addr.clone(),
        }));
    }

    // 新規生成
    let new_address = format!("p-{}", Uuid::new_v4().simple().to_string());
    db_lock.insert(agent_id.clone(), new_address.clone());

    Ok(warp::reply::json(&AgentMapping {
        agent_id,
        p_address: new_address,
    }))
}

async fn handle_receive_messages(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db_lock = db.lock().unwrap();
    let messages: Vec<(String, String)> = db_lock
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    Ok(warp::reply::json(&messages))
}
