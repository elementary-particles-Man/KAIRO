## 8. WhoAreYou: 多層認証モデル

本Meshは、LLM相互通信における「なりすまし」「内部汚染」を防ぐため、4層構造のWhoAreYou（WAU）認証を正式採用する。

### ✅ レイヤー1: 静的ID
- 生成時に暗号鍵ペアを発行。公開鍵と属性を含むデジタル証明書をLSC/Seed Node群が署名。

### ✅ レイヤー2: 動的認証
- チャレンジ・レスポンス認証を通信時に必須化し、鍵ペアの正当性を実時間で確認。

### ✅ レイヤー3: 振る舞い診断
- `mesh_trust_calculator.rs` の `check_behavior_anomaly()` を使用し、内部ベクトルの逸脱を検出。Peer Review/Gossipで継続的に監視。

### ✅ レイヤー4: ゼロトラスト/ACL/マルチシグ
- Seed Nodeには `seed_node_acl_manager.rs` により、権限最小化とマルチシグ署名を実装。重要操作の権限を動的に制御。
