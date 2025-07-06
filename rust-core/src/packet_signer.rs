// D:\dev\KAIRO\rust-core\src\packet_parser.rs

use crate::ai_tcp_packet_generated::aitcp as fb;
use crate::error::KairoError;

pub struct PacketParser {
    // ... （今後の実装で使用）
}

impl PacketParser {
    // バイト列からAITcpPacketをパースする単純な関数
    pub fn parse<'a>(buffer: &'a [u8]) -> Result<fb::AITcpPacket<'a>, KairoError> {
        // flatcが生成した正しい関数名でパケットを読み取る
        let packet = fb::root_as_aitcp_packet(buffer)
            .map_err(|_| KairoError::PacketParseFailed)?;

        // TODO: 今後、署名検証やシーケンスIDのチェックロジックをここに追加する

        Ok(packet)
    }
}