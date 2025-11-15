use anyhow::{anyhow, Error};
use kairo_lib::packet::Packet;
use log::{error, info, warn};
use reqwest::Client;
use std::net::SocketAddr;
use std::time::Duration;

const NO_COST_ENDPOINT: &str = "https://example.com/";
const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Zero-cost A-scheme: send payload to example.com to capture remote_addr without incurring GPT charges.
pub async fn gpt_log_and_respond(packet: &Packet) -> Result<(String, SocketAddr), Error> {
    info!(
        "  [GPT_Subsystem/NoCost] Processing packet seq={} from {}",
        packet.sequence, packet.source_p_address
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
        .build()?;

    let response = client
        .post(NO_COST_ENDPOINT)
        .body(packet.payload.clone())
        .send()
        .await
        .map_err(|e| {
            error!("Failed to reach {}: {}", NO_COST_ENDPOINT, e);
            anyhow!("Failed to reach {}: {}", NO_COST_ENDPOINT, e)
        })?;

    let remote_addr = response.remote_addr().unwrap_or_else(|| {
        warn!("[NoCost] remote_addr unavailable, using 0.0.0.0:0 fallback");
        "0.0.0.0:0".parse().unwrap()
    });

    info!(
        "  [GPT_Subsystem/NoCost] Actual remote addr observed: {}",
        remote_addr
    );

    Ok(("OK".to_string(), remote_addr))
}
