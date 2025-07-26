use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;
use kairo_lib::governance::OverridePackage;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegisterRequest {
    pub p_address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReissueRequest {
    pub old_p_address: String,
    pub new_p_address: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AgentInfo {
    pub p_address: String,
    pub public_key: String,
    pub registered_at: String,
    pub status: String,
    pub replaces: Option<String>,
}

const DB_FILE: &str = "registry.json";

pub fn read_registry() -> Result<Vec<AgentInfo>, std::io::Error> {
    let file = File::open(DB_FILE).unwrap_or_else(|_| File::create(DB_FILE).unwrap());
    let reader = BufReader::new(file);
    let registry = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(registry)
}

pub fn write_registry(registry: &[AgentInfo]) -> std::io::Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(DB_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, registry)?;
    Ok(())
}

async fn handle_registration(req: RegisterRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    let mut registry = read_registry().expect("Failed to read from DB");
    if registry.iter().any(|agent| agent.p_address == req.p_address && agent.status == "active") {
        let res = RegisterResponse { status: "exists".to_string(), message: "Active agent already registered".to_string() };
        Ok(warp::reply::json(&res))
    } else {
        let new_agent = AgentInfo {
            p_address: req.p_address.clone(),
            public_key: String::new(),
            registered_at: Utc::now().to_rfc3339(),
            status: "active".to_string(),
            replaces: None,
        };
        registry.push(new_agent);
        write_registry(&registry).expect("Failed to write to DB");
        let res = RegisterResponse { status: "success".to_string(), message: "Agent successfully registered".to_string() };
        Ok(warp::reply::json(&res))
    }
}

async fn handle_revocation(req: RegisterRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    let mut registry = read_registry().expect("Failed to read from DB");
    if let Some(agent) = registry.iter_mut().find(|a| a.p_address == req.p_address && a.status == "active") {
        agent.status = "revoked".to_string();
        write_registry(&registry).expect("Failed to write to DB");
        let res = RegisterResponse { status: "success".to_string(), message: "Agent successfully revoked".to_string() };
        Ok(warp::reply::json(&res))
    } else {
        let res = RegisterResponse { status: "not_found".to_string(), message: "Active agent not found".to_string() };
        Ok(warp::reply::json(&res))
    }
}

async fn handle_reissue(req: ReissueRequest, db_lock: Arc<Mutex<()>>) -> Result<impl Reply, Rejection> {
    let _lock = db_lock.lock().await;
    let mut registry = read_registry().expect("Failed to read from DB");
    if registry.iter().any(|a| a.p_address == req.new_p_address && a.status == "active") {
        let res = RegisterResponse { status: "exists".to_string(), message: "New P address already exists".to_string() };
        return Ok(warp::reply::with_status(warp::reply::json(&res), warp::http::StatusCode::CONFLICT));
    }
    let new_agent = AgentInfo {
        p_address: req.new_p_address.clone(),
        public_key: String::new(),
        registered_at: Utc::now().to_rfc3339(),
        status: "active".to_string(),
        replaces: Some(req.old_p_address.clone()),
    };
    registry.push(new_agent);
    write_registry(&registry).expect("Failed to write to DB");
    let res = RegisterResponse { status: "success".to_string(), message: "Agent ID successfully reissued".to_string() };
    Ok(warp::reply::with_status(warp::reply::json(&res), warp::http::StatusCode::OK))
}

async fn handle_emergency_reissue(_req: OverridePackage) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"received"))
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let db_lock = Arc::new(Mutex::new(()));

    let register_lock = Arc::clone(&db_lock);
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(warp::any().map({
            let register_lock = Arc::clone(&register_lock);
            move || Arc::clone(&register_lock)
        }))
        .and_then(handle_registration);

    let revoke_lock = Arc::clone(&db_lock);
    let revoke = warp::post()
        .and(warp::path("revoke"))
        .and(warp::body::json())
        .and(warp::any().map({
            let revoke_lock = Arc::clone(&revoke_lock);
            move || Arc::clone(&revoke_lock)
        }))
        .and_then(handle_revocation);

    let reissue_lock = Arc::clone(&db_lock);
    let reissue = warp::post()
        .and(warp::path("reissue"))
        .and(warp::body::json())
        .and(warp::any().map({
            let reissue_lock = Arc::clone(&reissue_lock);
            move || Arc::clone(&reissue_lock)
        }))
        .and_then(handle_reissue);

    let emergency_reissue = warp::post()
        .and(warp::path("emergency_reissue"))
        .and(warp::body::json())
        .and_then(handle_emergency_reissue);

    register.or(revoke).or(reissue).or(emergency_reissue)
}
