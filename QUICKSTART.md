KAIRO - Quickstart Guide
このガイドは、KAIROプロジェクトを最短でセットアップし、主要な機能を試すための手順書です。

0. 前提
プロジェクトのルートディレクトリでコマンドを実行してください。

Rust、Pythonの環境がセットアップ済みであること。

1. 常駐サービスの起動・停止
KAIROメッシュは、バックグラウンドで動作する複数の常駐サービスに依存します。

起動 (デーモン/シード/メッシュ):

python start_services.py

停止:

python stop_services.py

2. 初回オンボーディング（単一エージェント）
まず、あなた自身の操作用エージェントを1つ作成します。

鍵生成＆ID保存:
~/.kairo/agent.json に設定が保存されます。既存の場合は上書きされません。

cargo run --bin setup_agent

強制的に上書きしたい場合:

cargo run --bin setup_agent -- --force

3. マルチエージェント運用（任意）
複数の自律エージェントを運用する場合、それぞれに名前を付けて管理します。

任意名で新規エージェント作成:
agent_configs/{NAME}.json が生成されます。

# "CLI" という名前のエージェントを作成
cargo run --package kairo_agent --bin setup_agent -- --name CLI --new

設定ファイル検証:

cargo run --package kairo_agent --bin validate_config -- --path agent_configs/CLI.json

4. 署名付き通信テスト
エージェント間の通信が正しく署名され、検証されることを確認します。

送信 (Agent1 → 宛先へ):

cargo run --package kairo_agent --bin signed_sender \
  -- --from Agent1 --to 10.0.0.11/24 --message "Hello"

受信 (Agent2 が自分宛のメッセージを確認):

cargo run --package kairo_agent --bin receive_signed \
  -- --for-address 10.0.0.11/24 --from Agent2

なりすまし試験 (偽署名):
このメッセージはデーモン側で拒否されるはずです。

cargo run --package kairo_agent --bin signed_sender \
  -- --from Agent1 --to 10.0.0.11/24 --message "Fake" --fake

5. つまずいたら
レジストリのクリーンアップ:
エージェントの構成を大きく変更した際などに、agent_registry.json を削除してからサービスを再起動すると、状態がリセットされ問題が解決することがあります。

# Linux/macOS
rm -f agent_registry.json

# Windows
del agent_registry.json

サービスの再起動:

python stop_services.py && python start_services.py
