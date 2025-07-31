//! src/bot/main.rs

use simple_logger;
use log::*;
use warp::Filter;

mod api;
use api::status::status_route;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();
    info!("KAIROBOT: Logger initialized successfully. Starting warp server...");

    let health_check = warp::path("health").map(|| warp::reply::json(&"OK"));
    let routes = status_route().or(health_check);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 4040))
        .await;
}
