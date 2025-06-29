pub struct Packet<'a> {
    pub version: u8,
    pub ephemeral_key: &'a [u8],
    pub nonce: &'a [u8],
    pub encrypted_sequence_id: &'a [u8],
    pub encrypted_payload: &'a [u8],
    pub signature: &'a [u8],
}

pub fn parse_packet(data: &[u8]) -> Result<Packet, &'static str> {
    if data.len() < 1 { return Err("buffer too small"); }
    let version = data[0];
    let mut idx = 1;
    if idx >= data.len() { return Err("bad packet"); }
    let ek_len = data[idx] as usize;
    idx += 1;
    if idx + ek_len > data.len() { return Err("bad packet"); }
    let ephemeral_key = &data[idx..idx+ek_len];
    idx += ek_len;
    if idx >= data.len() { return Err("bad packet"); }
    let nonce_len = data[idx] as usize;
    idx += 1;
    if idx + nonce_len > data.len() { return Err("bad packet"); }
    let nonce = &data[idx..idx+nonce_len];
    idx += nonce_len;
    if idx + 2 > data.len() { return Err("bad packet"); }
    let seq_len = u16::from_be_bytes([data[idx], data[idx+1]]) as usize;
    idx += 2;
    if idx + seq_len > data.len() { return Err("bad packet"); }
    let encrypted_sequence_id = &data[idx..idx+seq_len];
    idx += seq_len;
    if idx + 2 > data.len() { return Err("bad packet"); }
    let payload_len = u16::from_be_bytes([data[idx], data[idx+1]]) as usize;
    idx += 2;
    if idx + payload_len > data.len() { return Err("bad packet"); }
    let encrypted_payload = &data[idx..idx+payload_len];
    idx += payload_len;
    if idx + 2 > data.len() { return Err("bad packet"); }
    let sig_len = u16::from_be_bytes([data[idx], data[idx+1]]) as usize;
    idx += 2;
    if idx + sig_len > data.len() { return Err("bad packet"); }
    let signature = &data[idx..idx+sig_len];

    Ok(Packet { version, ephemeral_key, nonce, encrypted_sequence_id, encrypted_payload, signature })
}
