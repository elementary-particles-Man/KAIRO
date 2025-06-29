'''//! PacketParser: parses AI-TCP binary packet using FlatBuffers,
//! verifies sequence ID and triggers re-transmission if mismatch.

use flatbuffers::FlatBufferBuilder;
use crate::tcp_packet_generated::AITCP::AITcpPacket;

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
        let packet = AITcpPacket::get_root_as_ai_tcp_packet(packet_buffer);
        let encrypted_seq_id = packet.encrypted_sequence_id().ok_or("Missing sequence_id")?;
        let seq_id = self.decrypt_sequence_id(encrypted_seq_id);

        if seq_id != self.last_received_seq_id + 1 {
            return Err("Packet sequence mismatch");
        }

        self.last_received_seq_id = seq_id;

        let payload = packet.encrypted_payload().ok_or("Missing payload")?;
        Ok((payload, seq_id))
    }
}
''