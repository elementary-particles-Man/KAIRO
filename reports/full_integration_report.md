# KAIRO repository fully cleaned
- 削除対象として指定されていた legacy ディレクトリおよびファイルを完全に除去しました。
- `src/kairo_daemon*`, `vov`, `web/static_html`, `rust-core/tests`, `rust-core/examples`、`tools/clear-mini/clear-mini-daemon-job.json`、`rust-core/src/log_recorder.rs` を整理済みです。

# CLEAR-Mini module injected
- 既存の `clear-mini` ディレクトリを再作成し、指定された Cargo 設定および API/構成モジュールを追加しました。
- Witness Ring、時間管理、PAddress 構造体、ISE 署名ユーティリティ等を含む軽量モジュールとして統合しました。

# PUT→Witness pipeline added
- `src/kairo-daemon/handle_send.rs` の送信処理冒頭に `// KAIRO_SEND_PATH_START` フックを挿入し、ClearMini への記録呼び出しを追加しました。
- ClearMini への依存関係を `src/kairo-daemon/Cargo.toml` に追加し、ビルド可能な形でクリップしました。

# Detector module added
- `clear-mini/src/detector.rs` を作成し、時間窓を用いてヒット数を追跡する `Window` 実装を追加しました。
- `lib.rs` に detector モジュールを公開し、将来的な利用が可能です。
