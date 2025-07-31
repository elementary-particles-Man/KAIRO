use simple_logger;
use log::*;
use warp::Filter;

mod api;
use api::status::status_route;

#[tokio::main]
async fn main() {
    // ログ初期化
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    info!("KAIROBOT: Logger initialized successfully.");

    // ルーティング構築
    let health_check = warp::path("health").map(|| warp::reply::json(&"OK"));
    let routes = status_route().or(health_check);

    // HTTPサーバ起動
    info!("KAIROBOT: Starting warp HTTP server on 127.0.0.1:4040...");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 4040))
        .await;
}
