//! src/bot/api/status.rs
// The API endpoint for querying task statuses.

use warp::Filter;

/// Simple status endpoint for the warp server.
pub fn status_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .map(|| warp::reply::json(&"All systems nominal"))
}
