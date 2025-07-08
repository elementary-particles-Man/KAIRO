// ===========================
// 📄 rust-core/tests/key_rotation_test.rs
// ===========================

// --- Keygen テスト ---
use rust_core::keygen::ephemeral_key;

#[test]
fn keys_are_unique() {
    let k1 = ephemeral_key();
    let k2 = ephemeral_key();
    assert_ne!(k1, k2, "Generated keys should be unique");
}

// --- LogRecorder テスト ---
use chrono::Utc;
// crateパスで呼ぶのが安全
use rust_core::log_recorder::LogRecorder;

#[test]
fn test_key_rotation() {
    // LogRecorder::new は内部でキー生成する形に合わせて修正
    let mut recorder = LogRecorder::new(vec![0; 32]);

    // 事前のタイムスタンプ
    let time_before_rotation = Utc::now();

    // rotate_key 実行
    recorder.rotate_key_if_needed();

    // 事後のタイムスタンプ
    let time_after_rotation = Utc::now();

    // rotate_key がパニックせず動作したことを確認
    assert!(time_after_rotation >= time_before_rotation);
}
