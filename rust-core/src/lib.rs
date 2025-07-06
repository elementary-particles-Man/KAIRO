// D:\dev\KAIRO\rust-core\src\lib.rs
pub mod ai_tcp_packet_generated;
pub mod error;
pub mod packet_parser;
pub mod signature;
pub mod log_recorder;
// pub mod coordination;
#[no_mangle]
pub extern "C" fn force_disconnect() {
    println!("Force disconnect triggered from Go!");
    // 必要に応じて VoV ログ処理などを呼び出す
}
// rust-core/src/lib.rs

// 必要な外部クレートの使用例（FlatBuffers、暗号化など）
// 必要に応じて残しておいてください
// use crate::ai_tcp_packet_generated; // FlatBuffers生成済みなら有効に
// use crate::crypto;                  // crypto.rsがあるなら有効に
// use crate::log_recorder;            // log_recorder.rsがあるなら有効に

// ----------------------------
// ✅ 必須: Go連携用エクスポート関数
// ----------------------------

#[no_mangle]
pub extern "C" fn example_function() {
    println!("Hello from rust-core cdylib! This proves that the DLL exports are working correctly.");
}

// ----------------------------
// ✅ 必要なら追加で公開する関数
// ----------------------------

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// ----------------------------
// ✅ 必要に応じて内部ロジックをここに置く
// ----------------------------

// 例:
// pub mod ai_tcp_packet_generated;
// pub mod crypto;
// pub mod log_recorder;

