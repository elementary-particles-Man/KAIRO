use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex as StdMutex};
use chrono::Utc;

use ed25519_dalek::{Signature, VerifyingKey};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter, Rejection, Reply};
use base64::Engine;
use hyper::{Body, Request, Response, StatusCode as HyperStatusCode};
use sha2::Digest;

use kairo_lib::packet::AiTcpPacket;
use kairo_lib::AgentConfig;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use clap::Parser;

use kairo_lib::config::{load_daemon_config, DaemonConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".kairo/config/daemon_config.json")]
    config: String,
}


/// Simple pool for issuing incremental P-addresses.
struct AddressPool {
    next_address: u8,
}

/// Global in-memory message queue indexed by destination P-address.
static MESSAGE_QUEUE: Lazy<Arc<Mutex<HashMap<String, Vec<AiTcpPacket>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// Shared application state for Hyper handlers.
struct AppState {
    queue: Mutex<HashMap<String, Vec<Vec<u8>>>>,
}

impl AppState {
    async fn enqueue(&self, to: String, data: Vec<u8>) {
        let mut q = self.queue.lock().await;
        q.entry(to).or_default().push(data);
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AssignPAddressRequest {
    public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AgentInfo {
    public_key: String,
    p_address: String,
    registered_at: String,
    status: String,
}

const AGENT_REGISTRY_FILE: &str = "agent_registry.json";

fn read_agent_registry() -> Result<Vec<AgentInfo>, std::io::Error> {
    let file = File::open(AGENT_REGISTRY_FILE).unwrap_or_else(|_| File::create(AGENT_REGISTRY_FILE).unwrap());
    let reader = BufReader::new(file);
    let registry = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());
    Ok(registry)
}

fn write_agent_registry(registry: &[AgentInfo]) -> std::io::Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(AGENT_REGISTRY_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, registry)?;
    Ok(())
}



/// Verify that a packet's signature matches the sending agent's public key.
fn verify_packet_signature(packet: &AiTcpPacket, registry: &[AgentInfo]) -> bool {
    let source_agent = match registry
        .iter()
        .find(|a| a.public_key == packet.source_public_key) // public_key を使用
    {
        Some(agent) => agent,
        None => {
            println!("🔴 Signature Fail: Source agent not found in registry for public key: {}", packet.source_public_key);
            return false;
        }
    };

    let public_key_bytes = match hex::decode(&source_agent.public_key) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("🔴 Signature Fail: Invalid public key format in registry.");
            return false;
        }
    };

    let public_key = match public_key_bytes.as_slice().try_into() {
        Ok(bytes) => match VerifyingKey::from_bytes(bytes) {
            Ok(key) => key,
            Err(e) => {
                println!("🔴 Signature Fail: Invalid public key bytes from registry. Error: {:?}", e);
                return false;
            }
        },
        Err(e) => {
            println!("🔴 Signature Fail: Public key bytes not 32 bytes long. Error: {:?}", e);
            return false;
        }
    };

    let signature_bytes = match hex::decode(&packet.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("🔴 Signature Fail: Invalid signature format.");
            return false;
        }
    };

    let signature_array: [u8; 64] = match signature_bytes.as_slice().try_into() {
        Ok(arr) => arr,
        Err(e) => {
            println!("🔴 Signature Fail: Invalid signature byte length. Error: {:?}", e);
            return false;
        }
    };

    let signature = Signature::from_bytes(&signature_array);

    public_key
        .verify(packet.payload.as_bytes(), &signature)
        .is_ok()
}

/// Handle an incoming packet POST request with signature verification.
async fn handle_send(req: Request<Body>, state: Arc<AppState>) -> Result<Response<Body>, eyre::Error> {
    // 署名付きパケットの検証を追加
    use ed25519_dalek::{Signature, VerifyingKey};
    use base64::Engine;
    use hyper::{Response, Body, StatusCode};
    use sha2::Digest;
    let bytes = hyper::body::to_bytes(req.into_body()).await?;
    // リクエストボディを JSON として解析
    let incoming: serde_json::Value = serde_json::from_slice(&bytes)?;
    // packet と signature を抽出
    let packet: crate::packet::AiTcpPacket = serde_json::from_value(incoming["packet"].clone())?;
    let signature_b64 = incoming["signature"].as_str().unwrap_or("");
    let signature_bytes = base64::engine::general_purpose::STANDARD.decode(signature_b64)?;
    let signature = Signature::from_bytes(&signature_bytes.try_into().map_err(|_| eyre::eyre!("Invalid signature length"))?);
    // 公開鍵は Agent の設定ファイルに保存された hex 文字列と一致している想定
    let public_key_bytes = hex::decode(&packet.from)?;
    let verify_key = VerifyingKey::from_bytes(&public_key_bytes.try_into().map_err(|_| eyre::eyre!("Invalid public key length"))?)?;
    // ペイロードに対してハッシュを計算
    let mut hasher = sha2::Sha512::new();
    hasher.update(packet.payload.as_bytes());
    let digest = hasher.finalize();
    // 署名検証
    if verify_key.verify(&digest, &signature).is_ok() {
        state.enqueue(packet.to.clone(), bytes.to_vec()).await;
        Ok(Response::new(Body::from("[SIGNATURE VERIFIED] queued")))
    } else {
        Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("[SIGNATURE INVALID] rejected"))?)
    }
}

/// Deliver all queued packets for the requested P-address.
async fn handle_receive(p_address: String) -> Result<impl Reply, Rejection> {
    println!("🔵 [RECEIVE] Request received for P-address: {}", p_address);
    let mut queue = MESSAGE_QUEUE.lock().await;
    if let Some(inbox) = queue.get_mut(&p_address) {
        let packets = inbox.clone();
        inbox.clear();
        println!(
            "🟡 [RECEIVE] Delivered {} packets to {}",
            packets.len(),
            p_address
        );
        Ok(warp::reply::json(&packets))
    } else {
        println!("🟡 [RECEIVE] No inbox found for P-address: {}", p_address);
        Ok(warp::reply::json(&Vec::<AiTcpPacket>::new()))
    }
}

/// Assign a new P-address from the pool.
async fn assign_p_address_handler(req: AssignPAddressRequest, pool: Arc<StdMutex<AddressPool>>) -> Result<impl Reply, Rejection> {
    let mut registry = read_agent_registry().unwrap_or_default();

    // Check if public_key already exists in registry
    if let Some(existing_agent) = registry.iter().find(|agent| agent.public_key == req.public_key) {
        println!("🟢 [DAEMON] Public Key already registered: {} with P-Address: {}", req.public_key, existing_agent.p_address);
        return Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({ "public_key": req.public_key, "p_address": existing_agent.p_address })),
            StatusCode::OK,
        ));
    }

    let mut pool_guard = pool.lock().unwrap();
    let addr = pool_guard.next_address;
    pool_guard.next_address += 1;
    let p_address = format!("10.0.0.{}/24", addr); // CIDR表記を追加

    let new_agent_info = AgentInfo {
        public_key: req.public_key.clone(),
        p_address: p_address.clone(),
        registered_at: Utc::now().to_rfc3339(),
        status: "active".to_string(),
    };

    registry.push(new_agent_info);
    write_agent_registry(&registry).expect("Failed to write agent registry");

    println!("🟢 [DAEMON] Assigned NEW P-Address: {} for public_key: {}", p_address, req.public_key);
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "public_key": req.public_key, "p_address": p_address })),
        StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("KAIRO-P Daemon starting...");
    println!("Loading configuration from: {}", args.config);

    let config = load_daemon_config(".kairo/.config/daemon_config.json")
        .unwrap_or_else(|_| {
            println!("WARN: daemon_config.json not found or invalid. Falling back to default bootstrap address.");
            DaemonConfig {
                listen_address: "127.0.0.1".to_string(),
                listen_port: 3030,
            }
        });
    let initial_next_address = {
        let registry = read_agent_registry().unwrap_or_default();
        let max_p_address_num = registry.iter()
            .filter_map(|agent| {
                agent.p_address.split('/').next() // "10.0.0.X/24" から "10.0.0.X" を取得
                    .and_then(|s| s.split('.').last()) // "10.0.0.X" から "X" を取得
                    .and_then(|s| s.parse::<u8>().ok()) // "X" を数値に変換
            })
            .max()
            .unwrap_or(0); // 登録がない場合は0
        max_p_address_num + 1
    };
    let pool = Arc::new(StdMutex::new(AddressPool { next_address: initial_next_address }));

    let assign_p_address = warp::post()
        .and(warp::path("assign_p_address"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&pool)))
        .and_then(assign_p_address_handler);

    let send = warp::post()
        .and(warp::path("send"))
        .and(warp::body::json())
        .and_then(handle_send);

    let receive = warp::get()
        .and(warp::path("receive"))
        .and(warp::query::<HashMap<String, String>>()) // クエリパラメータを処理
        .and_then(|params: HashMap<String, String>| async move {
            if let Some(p_address) = params.get("for") {
                handle_receive(p_address.clone()).await
            } else {
                Err(warp::reject::not_found())
            }
        });

    let routes = assign_p_address.or(send).or(receive);

    let listen_addr: std::net::IpAddr = config.listen_address.parse().expect("Invalid listen address");

    println!("Listening for address requests on http://{}:{}", config.listen_address, config.listen_port);
    warp::serve(routes).run((listen_addr, config.listen_port)).await;
}