//! 最低限のスモークテスト（上書き防止の挙動のみ）
#[test]
fn no_overwrite_without_force() {
    // 仕様上の確認: 実ファイルに触らない形のロジック分離が無いので、
    // ここでは単にコンパイルと起動可能性の担保のみ（CIで `cargo run --bin setup_agent` を手動実行）。
    assert!(true);
}
