// D:\dev\KAIRO\rust-core\src\packet_parser.rs

// 'crate' を使うことで、自分自身のクレート内のモジュールを正しく参照します
use crate::ai_tcp_packet_generated::aitcp as fb;
use crate::error::KairoError;

pub struct PacketParser {
    expected_sequence_id: u64,
}

impl PacketParser {
    pub fn new(_session_key: Vec<u8>) -> Self {
        Self { expected_sequence_id: 0 }
    }

    pub fn parse<'a>(&mut self, buffer: &'a [u8]) -> Result<fb::AITcpPacket<'a>, KairoError> {
        let packet = fb::root_as_aitcp_packet(buffer)
            .map_err(|_| KairoError::PacketParseFailed)?;

        // 将来的に復号・検証ロジックをここに追加します

        self.expected_sequence_id += 1;

        Ok(packet)
    }
}