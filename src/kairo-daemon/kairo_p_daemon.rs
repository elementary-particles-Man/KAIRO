//! bin/daemon/kairo_p_daemon.rs
//! The persistent KAIRO-P daemon for address assignment.

use warp::Filter;
use std::sync::{Arc, Mutex};
use serde::Serialize;

#[derive(Serialize)]
struct AddressResponse {
    p_address: String,
}

// A very simple in-memory address pool for now.
struct AddressPool {
    next_address: u8,
}

#[tokio::main]
async fn main() {
    println!("KAIRO-P Daemon starting...");

    let pool = Arc::new(Mutex::new(AddressPool { next_address: 1 }));

    let get_address = warp::post()
        .and(warp::path("request_address"))
        .and(warp::any().map(move || Arc::clone(&pool)))
        .map(|pool: Arc<Mutex<AddressPool>>| {
            let mut pool = pool.lock().unwrap();
            let addr = pool.next_address;
            pool.next_address += 1;

            let assigned = format!("10.0.0.{}", addr);
            println!("Assigned P-Address: {}", assigned);

            let response = AddressResponse {
                p_address: assigned,
            };

            warp::reply::json(&response)
        });

    println!("Listening for address requests on http://127.0.0.1:3030");
    warp::serve(get_address).run(([127, 0, 0, 1], 3030)).await;
}
