mod handler;
mod clear_mini_state;
mod task_queue;
mod api {
    pub mod controller;
}

use std::net::SocketAddr;
use std::fs::File;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use axum::routing::{get, post};
use axum::Router;

use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config as LogConfig, TerminalMode, ColorChoice, LevelFilter};
use handler::{handle_send, handle_gpt};
use task_queue::TaskQueue;
use api::controller::add_task;

// エントリポイント
#[tokio::main]
async fn main() {
    // ✅ Logger 初期化
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, LogConfig::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(
            LevelFilter::Debug,
            LogConfig::default(),
            File::create("kairo_daemon.log").unwrap(),
        ),
    ])
    .unwrap();

    // ✅ TaskQueue 初期化
    let queue = Arc::new(Mutex::new(TaskQueue::new()));

    // ✅ Router 設定
    let base_app = Router::new()
        .route("/", get(root))
        .route("/send", post(handle_send))
        .route("/gpt", post(handle_gpt))
        .route("/add_task", post(add_task))
        .with_state(queue.clone());

    #[cfg(debug_assertions)]
    let app = {
        log::warn!("Adding debug-only API endpoint: GET /_internal_debug/dump");
        base_app.route("/_internal_debug/dump", get(handle_debug_dump))
    };

    #[cfg(not(debug_assertions))]
    let app = base_app;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    // ✅ サーバ起動
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}


// 🧪 root応答用（テスト用）
async fn root() -> &'static str {
    "KAIRO Daemon Online"
}



// --- デバッグビルド時のみ有効な内部API ---
#[cfg(debug_assertions)]
async fn handle_debug_dump() -> impl axum::response::IntoResponse {
    use axum::Json;
    use crate::clear_mini_state::CLEAR_MINI;

    log::warn!("Executing debug dump API. This MUST NOT appear in release builds.");
    let snapshot = CLEAR_MINI.lock().unwrap().dump_witness_snapshot();
    Json(snapshot)
}
