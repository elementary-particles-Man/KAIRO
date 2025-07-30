use kairo_core::bot::core::{main_loop, TaskQueue};
use kairo_core::bot::api::receiver::create_task_route;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, fs::dir};

#[tokio::main]
async fn main() {
    let queue = Arc::new(Mutex::new(TaskQueue::load()));

    // APIサーバーの起動
    let api_routes = create_task_route(queue.clone());
    let ui_routes = dir("vov/kairobot_ui");
    let routes = api_routes.or(ui_routes);
    let api_server = warp::serve(routes).run(([127, 0, 0, 1], 4040));

    // KAIROBOTのメインループとAPIサーバーを並行して実行
    tokio::join!(main_loop(queue), api_server);
}
