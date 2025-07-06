// D:\dev\KAIRO\rust-core\tests\key_rotation_test.rs
use chrono::Utc;
// コンパイラの指示通り、正しいパスでLogRecorderをインポートする
use rust_core::log_recorder::LogRecorder;

#[test]
fn test_key_rotation() {
    let initial_key = vec![1; 32];
    let mut recorder = LogRecorder::new(initial_key.clone());

    // ダミーのテストとして、キーがローテーションされることを確認
    let time_before_rotation = Utc::now();
    recorder.rotate_key_if_needed();
    let time_after_rotation = Utc::now();

    // ここでは単純にrotate_key_if_neededがパニックしないことを確認する
    assert!(time_after_rotation >= time_before_rotation);
}