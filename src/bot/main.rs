'''//! src/bot/main.rs

use simple_logger;
use log::*;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();
    info!("KAIROBOT: Logger initialized successfully. This is a test log.");
    debug!("KAIROBOT: This is a debug log.");

    // プログラムがすぐに終了しないように少し待機
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    info!("KAIROBOT: Program exiting.");
}
''