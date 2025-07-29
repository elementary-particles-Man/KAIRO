// src/kairo-daemon/main.rs

mod handler;

use std::net::SocketAddr;
use std::fs::File;

use axum::{
    routing::{get, post},
    Router,
};
use axum::Server;
use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config as LogConfig, TerminalMode, ColorChoice, LevelFilter};

use handler::{handle_send, handle_gpt};

#[tokio::main]
async fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, LogConfig::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Debug, LogConfig::default(), File::create("kairo_daemon.log").unwrap()),
    ]).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/send", post(handle_send))
        .route("/gpt", post(handle_gpt));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "KAIRO Daemon Online"
}
