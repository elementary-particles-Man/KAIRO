//! bin/daemon/kairo_p_daemon.rs
//! The persistent KAIRO-P daemon for address assignment.

use warp::Filter;
use std::sync::{Arc, Mutex};

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
            println!("Assigned P-Address: 10.0.0.{}", addr);
            warp::reply::json(&format!("10.0.0.{}", addr))
        });

    println!("Listening for address requests on http://127.0.0.1:3030");
    warp::serve(get_address).run(([127, 0, 0, 1], 3030)).await;
}
