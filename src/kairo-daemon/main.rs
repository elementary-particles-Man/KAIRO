mod handler;

use std::net::SocketAddr;
use std::fs::File;
use tokio::net::TcpListener;

use axum::{
    routing::{get, post},
    Router,
};

use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config as LogConfig, TerminalMode, ColorChoice, LevelFilter};
use handler::{handle_send, handle_gpt};

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

    // ✅ Router 設定
    let app = Router::new()
        .route("/", get(root))
        .route("/send", post(handle_send))
        .route("/gpt", post(handle_gpt));

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
