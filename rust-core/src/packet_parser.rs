// KAIRO/rust-core/src/packet_parser.rs
// use serde::{Deserialize, Serialize}; // FlatBuffersを使うので通常は不要になります
use bytes::Bytes;
use crate::error::KairoError;
use flatbuffers; // FlatBuffersクレートをインポート

// FlatBuffersによって生成されるEphemeralSessionモジュールをインポート
// これはflatcがai-tcp/flatbuffers/ephemeral_session.fbsから生成したRustコードです。
// 生成されたファイル名とパスに合わせて調整してください。
// 例: `flatbuffers/ephemeral_session_generated.rs` が生成されている場合
//     mod ephemeral_session_generated;
//     use ephemeral_session_generated::aitcp as fb_aitcp; // aitcpはスキーマで定義したnamespace

// ここでは、生成されたFlatBuffersのRustモジュールを直接インポートします。
// プロジェクトの構造に応じてパスを調整してください。
// 例: src/api_serverと同じ階層にflatbuffers/aitcp/ephemeral_session_generated.rsがある場合
#[path = "../../../flatbuffers/ephemeral_session_generated.rs"] // aitcpサブディレクトリは不要です
#[allow(dead_code)]
#[allow(unused_imports)]
mod ephemeral_session_generated;
use ephemeral_session_generated::aitcp as fb_aitcp; // aitcpはephemeral_session.fbsで定義されたnamespace

// 既存のPacket, PacketHeader, PacketPayload構造体/enumは、
// FlatBuffersの構造を使う場合は不要になりますが、
// KAIROの他の部分でまだ使われている可能性があるので、
// 必要に応じてリファクタリングしてください。
// 今回のテストではFlatBuffersのパースに焦点を当てます。
// 例えば、以下のようにダミーのまま残すか、完全に削除します。
#[derive(Debug, Clone)] // derive Clone for PacketPayload if used elsewhere
pub struct Packet {
    pub header: PacketHeader,
    pub payload: PacketPayload,
}

#[derive(Debug, Clone)] // derive Clone for PacketHeader if used elsewhere
pub struct PacketHeader {
    pub version: u8,
    pub packet_type: u8,
    pub length: u16,
    pub transaction_id: String,
}

#[derive(Debug, Clone)] // derive Clone for PacketPayload if used elsewhere
pub enum PacketPayload {
    AuthRequest { username: String },
    AuthResponse { success: bool, message: String },
    Data { data: Vec<u8> },
    // FlatBuffersペイロードを表す新しいバリアントを追加することも検討
    FlatBuffersEphemeralSession { session_id: String, public_key: Vec<u8> },
}


// PacketParser構造体自体はそのまま
pub struct PacketParser {
    session_key: Vec<u8>,
}

impl PacketParser {
    pub fn new(session_key: Vec<u8>) -> Self {
        PacketParser { session_key }
    }

    pub fn parse(&mut self, data: &Bytes) -> Result<Packet, Box<dyn std::error::Error>> {
        // ここでFlatBuffersのバイナリデータをデシリアライズ
        let ephemeral_session = match fb_aitcp::root_as_ephemeral_session(data) {
            Ok(session) => session,
            Err(e) => {
                // パースエラーの場合はKairoErrorに変換して返す
                eprintln!("FlatBuffers parsing error: {:?}", e); // デバッグ用
                return Err(Box::new(KairoError::PacketParseFailed));
            }
        };

        // パースが成功したら、受信したセッションIDと公開鍵の情報を表示
        println!("Successfully parsed FlatBuffers EphemeralSession:");
        println!("  Session ID: {}", ephemeral_session.session_id().unwrap_or("[N/A]"));
        println!("  Public Key Length: {}", ephemeral_session.public_key().map_or(0, |key| key.len()));
        println!("  Expiration Unix: {}", ephemeral_session.expiration_unix());

        // KAIROの既存のPacket構造体に合わせて結果をラップして返す
        // これはFlatBuffersへの完全移行までの暫定的な措置です
        let header = PacketHeader {
            version: 1, // ダミーのバージョン
            packet_type: 1, // ダミーのタイプ
            length: data.len() as u16, // 受信したデータの実際の長さ
            transaction_id: ephemeral_session.session_id().unwrap_or("").to_string(),
        };

        let payload = PacketPayload::FlatBuffersEphemeralSession {
            session_id: ephemeral_session.session_id().unwrap_or("").to_string(),
            public_key: ephemeral_session.public_key().map_or(vec![], |key| key.bytes().to_vec()),
        };

        Ok(Packet { header, payload })
    }
}