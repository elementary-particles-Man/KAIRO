# ClearMini singleton化完了
- `src/kairo-daemon/Cargo.toml` に `once_cell = "1.20"` と `log = "0.4"` を明示し、実行時に `CLEAR_MINI` を `Lazy<Mutex<_>>` で共有できるようにしました。
- `handle_send.rs` へ `CLEAR_MINI`/`DET_DST_10S` のシングルトンを配置し、再初期化なしで Witness リングとウィンドウを共有します。

# handle_send に witness 記録関数を追加
- `SendRequest` 構造体を導入し、パケットから payload 長や P アドレス情報を抽出する `from_packet()` を実装しました。
- `record_witness()` で `CLEAR_MINI` をロックし、PAddressRecord 化した送信元/宛先を用いて `ClearMini::record()` を実行します。

# 実値 route_flags/dst_ip/dst_port を受け取る構造を整備
- 宛先 P アドレスから `SocketAddr`/`IpAddr` を推定する補助関数を用意し、`SendRequest` に `[u8;16]` の IP と `u16` ポートを格納します。
- `derive_agent_id()` で source P アドレスから安定 ID を算出し、`route_flags` を将来的に拡張できるフィールドとして保持します。

# Detector(10s sliding window) を導入し burst detection を有効化
- `Window::new(10)` をシングルトン化して `detect_burst()` から利用し、宛先ごとの 10 秒ウィンドウでヒット数を監視します。
- 50 ヒット／10 秒を超過した際は、宛先 IP/Port を含む `log::warn!` を出力するようにしました。

# KAIRO-daemon 全送信パスで PUT メタデータが監視される状態になった
- `handle_send()` の `// KAIRO_SEND_PATH_START` フックで `record_witness()` → `detect_burst()` を連続呼び出しすることで、送信処理のフロントで必ず記録・検出が走るようになりました。
- 既存の署名検証や GPT ルーティングロジックには手を触れず、副作用なく観測系を追加しています。
