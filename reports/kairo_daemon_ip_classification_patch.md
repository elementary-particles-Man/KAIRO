# Added ip_classifier.rs with EndpointClass enum and classify_ip() function.
- `src/kairo-daemon/ip_classifier.rs` を新設し、`EndpointClass` 列挙と `classify_ip()` を定義しました。

# handle_send.rs now classifies actual_socket_addr.ip() before burst detection.
- GPT 宛てパスで `actual_socket_addr` を用いて分類し、バースト検知の前にクリアミニ監視へ組み込みます。

# Classification result is logged with appropriate severity.
- `EndpointClass` に応じて `info!/warn!/error!` を発行し、監視ログから素早く判別できるようにしました。

# Detector remained unchanged; classification stays strictly in KAIRO-P layer.
- バースト検知のキーやリングには影響を与えず、分類情報はログのみで扱います。

# Architecture preserves CLEAR-Mini purity and follows your KAIRO-P / KAIRO-C model.
- CLEAR-Mini のリング構造は変更せず、KAIRO-P レイヤの内部ロジックとしてのみ機能追加しました。
