# Patched src/kairo-daemon/main.rs (Axum version).
- `main.rs` に `mod handle_send;` を追加して CLEAR_MINI シングルトンへアクセスできるようにし、axum の `use` セクションを整理しました。

# Added 'use axum::routing::get'.
- 既存の `get/post` ルートに加えて、デバッグルート追加のため `axum::routing::get` を明示的に利用しています。

# Appended debug-only 'handle_debug_dump' handler, protected by #[cfg(debug_assertions)].
- `handle_debug_dump()` をファイル末尾に追加し、`cfg(debug_assertions)` 付きでビルド時にのみ有効になるようにしました。HTTP から CLEAR_MINI のスナップショットを JSON で返します。

# Patched 'main' fn to conditionally add the '/_internal_debug/dump' route only if cfg!(debug_assertions) is true.
- `Router` 構築後に `#[cfg(debug_assertions)]` ブロックでのみ `/_internal_debug/dump` を追加することで、リリースビルドではルート自体が生成されません。

# The C->P security model is preserved; dump API is excluded from release builds.
- 内部ダンプ機能はデバッグ実行 (`cargo run`) のみ有効であり、リリース配布物には含まれないため、C->P ラインのセキュリティ要件を維持できます。
