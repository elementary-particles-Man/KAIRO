# Patched clear-mini/Cargo.toml: Added 'serde' dependency.
- `clear-mini/Cargo.toml` の `[dependencies]` に `serde = { version = "1.0", features = ["derive"] }` を追加し、WitnessRecord を JSON などへシリアライズ可能にしました。

# Patched clear-mini/src/witness.rs: Added 'Serialize' derive to WitnessRecord.
- WitnessRecord に `#[derive(Serialize)]` を付与し、リングバッファのスナップショットをそのまま外部出力できるよう準備しました。

# Patched clear-mini/src/api.rs: Added internal 'dump_witness_snapshot()' function.
- `record()` 実装を整理しつつ、オンメモリリングの `Vec<WitnessRecord>` を返す `dump_witness_snapshot()` を追加しました（HTTP には公開せず内部制御用）。

# KAIRO-P node now has a secure internal entry point for future KAIRO-C dump commands.
- これにより KAIRO-C の制御コマンドから Witness 情報を取得する足場が整い、将来の監査やフォレンジック機能を安全に拡張できます。
