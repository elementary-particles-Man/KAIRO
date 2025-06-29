use flatbuffers::FlatBufferBuilder;

// Placeholder for generated FlatBuffers schema
pub mod tcp_packet_generated {
    pub mod kairo {
        #[derive(Clone, Copy)]
        pub struct AiTcpPacket<'a> {
            buffer: &'a [u8],
        }

        impl<'a> AiTcpPacket<'a> {
            pub fn root_as_ai_tcp_packet(buf: &'a [u8]) -> Result<AiTcpPacket<'a>, &'static str> {
                Ok(AiTcpPacket { buffer: buf })
            }

            pub fn sequence_id(&self) -> &'a [u8] {
                // For demo purposes, first 8 bytes
                &self.buffer[0..8]
            }

            pub fn payload(&self) -> &'a [u8] {
                &self.buffer[8..]
            }
        }
    }
}

use tcp_packet_generated::kairo::AiTcpPacket;

pub struct PacketParser {
    session_key: Vec<u8>,
    last_received_seq_id: u64,
}

impl PacketParser {
    pub fn new(session_key: Vec<u8>) -> Self {
        Self { session_key, last_received_seq_id: 0 }
    }

    fn decrypt_sequence_id(&self, encrypted: &[u8]) -> u64 {
        let mut arr = [0u8; 8];
        let len = encrypted.len().min(8);
        arr[..len].copy_from_slice(&encrypted[..len]);
        u64::from_be_bytes(arr)
    }

    pub fn parse<'a>(&mut self, packet_buffer: &'a [u8]) -> Result<(&'a [u8], u64), &'static str> {
        let packet = AiTcpPacket::root_as_ai_tcp_packet(packet_buffer)?;
        let encrypted_seq_id = packet.sequence_id();
        let seq_id = self.decrypt_sequence_id(encrypted_seq_id);
        if seq_id != self.last_received_seq_id + 1 {
            return Err("Packet sequence mismatch");
        }
        self.last_received_seq_id = seq_id;
        Ok((packet.payload(), seq_id))
    }
}

