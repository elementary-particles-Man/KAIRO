use kairo_core::bot::core::{main_loop, TaskQueue};
use kairo_core::bot::api::receiver::create_task_route;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    let queue = Arc::new(Mutex::new(TaskQueue::load()));

    // APIサーバーの起動
    let api_routes = create_task_route(queue.clone());
    let api_server = warp::serve(api_routes).run(([127, 0, 0, 1], 4040));

    // KAIROBOTのメインループとAPIサーバーを並行して実行
    tokio::join!(main_loop(queue), api_server);
}
