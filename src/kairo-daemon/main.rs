mod handler;
mod task_queue;
mod api {
    pub mod controller;
}

use std::net::SocketAddr;
use std::fs::File;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use axum::{
    routing::{get, post},
    Router,
};

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
    let app = Router::new()
        .route("/", get(root))
        .route("/send", post(handle_send))
        .route("/gpt", post(handle_gpt))
        .route("/add_task", post(add_task))
        .with_state(queue.clone());

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

