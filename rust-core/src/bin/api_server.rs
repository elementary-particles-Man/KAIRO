use std::io::Read;
use tiny_http::{Server, Response, Method, StatusCode};
use serde::Deserialize;
use lz4_flex::compress_prepend_size;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use ed25519_dalek::{Signer, Keypair};

use rust_core::transmit_packet;

#[derive(Deserialize)]
struct ApiPayload {
    payload: String,
}

struct InnerPacket(Vec<u8>);

fn to_inner_packet(p: ApiPayload) -> InnerPacket {
    InnerPacket(p.payload.into_bytes())
}

fn prepare_packet(data: &InnerPacket, signer: &Keypair) -> Vec<u8> {
    let compressed = compress_prepend_size(&data.0);

    let key = Key::from_slice(&[0u8; 32]);
    let nonce = Nonce::from_slice(&[0u8; 12]);
    let cipher = ChaCha20Poly1305::new(key);
    let mut encrypted = cipher.encrypt(nonce, compressed.as_ref()).unwrap_or_default();

    let sig = signer.sign(&encrypted);
    encrypted.extend_from_slice(sig.as_ref());
    encrypted
}

fn handle_request(mut req: tiny_http::Request, signer: &Keypair) {
    if req.method() == &Method::Post && req.url() == "/api/v1/aitcp" {
        let mut body = Vec::new();
        req.as_reader().read_to_end(&mut body).ok();
        let payload: ApiPayload = serde_json::from_slice(&body).unwrap_or(ApiPayload{payload: String::new()});
        let pkt = to_inner_packet(payload);
        let out = prepare_packet(&pkt, signer);
        transmit_packet(&out);
        let resp = Response::from_string("{\"status\":\"ok\"}").with_status_code(StatusCode(200));
        let _ = req.respond(resp);
    } else {
        let _ = req.respond(Response::empty(404));
    }
}

fn main() -> std::io::Result<()> {
    let keypair = Keypair::generate(&mut rand::rngs::OsRng);
    let server = Server::http("0.0.0.0:8081")?;
    for request in server.incoming_requests() {
        handle_request(request, &keypair);
    }
    Ok(())
}
