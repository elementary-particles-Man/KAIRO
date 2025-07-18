use serde::Deserialize;

#[derive(Deserialize)]
struct AddressResponse {
    p_address: String,
}

// Pアドレスの要求を行う関数
fn request_p_address() -> String {
    println!("\nRequesting KAIRO-P address from local daemon...");
    let client = reqwest::blocking::Client::new();

    match client.post("http://localhost:3030/request_address").send() {
        Ok(res) => {
            let addr = res.json::<AddressResponse>()
                .map(|r| r.p_address)
                .unwrap_or_else(|_| "error".to_string());
            println!("-> KAIRO-P Address assigned: {}", addr);
            addr
        }
        Err(e) => {
            println!("-> Failed to connect to KAIRO-P daemon: {}. Is it running?", e);
            "failed_to_connect".to_string()
        }
    }
}
