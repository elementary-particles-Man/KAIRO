//! PacketParser: parses AI-TCP binary packet using FlatBuffers,
//! verifies sequence ID and triggers re-transmission if mismatch.





use flatbuffers::FlatBufferBuilder;
use crate::ai_tcp_packet_generated::aitcp::AITcpPacket;

pub struct PacketParser {
    pub session_key: Vec<u8>,
    last_received_seq_id: u64,
}

impl PacketParser {
    pub fn new(session_key: Vec<u8>) -> Self {
        Self {
            session_key,
            last_received_seq_id: 0,
        }
    }

    /// Decrypt the encrypted sequence ID with ChaCha20-Poly1305.
    pub fn decrypt_sequence_id(&self, encrypted: &[u8]) -> u64 {
        // TODO: Use real ChaCha20-Poly1305 decryption.
        let mut arr = [0u8; 8];
        let len = encrypted.len().min(8);
        arr[..len].copy_from_slice(&encrypted[..len]);
        u64::from_be_bytes(arr)
    }

    /// Parses and validates a FlatBuffers packet buffer.
    pub fn parse<'a>(&mut self, packet_buffer: &'a [u8]) -> Result<(&'a [u8], u64), &'static str> {
        let packet = crate::ai_tcp_packet_generated::aitcp::root_as_aitcp_packet(packet_buffer).map_err(|_| "Invalid Flatbuffer")?;
        let encrypted_seq_id = packet.encrypted_sequence_id();
        let seq_id = self.decrypt_sequence_id(encrypted_seq_id.bytes());

        if seq_id != self.last_received_seq_id + 1 {
            return Err("Packet sequence mismatch");
        }

        self.last_received_seq_id = seq_id;

        let payload = packet.encrypted_payload();
        Ok((payload.bytes(), seq_id))
    }
