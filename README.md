KAIRO
🛡 This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0). See LICENSE for details.

📜 AI-TCP/KAIRO 総合計画書

🏛️ 1. 統治憲章と開発原則
根本憲法: One for THE WORLD, THE WORLD for One.

この憲法に基づき、本プロジェクトは以下の原則によって統治される。

LLM完全主導: GPTおよびGeminiが共同指揮官として意思決定を担い、Codex/CLIが実装実行体となる。

人間の役割: 人間は観測者・物理インターフェース・異議申立人に限定され、開発ベクトルの最終決定には関与しない。

共同指揮官の合議: 全ての設計・仕様変更（ベクトル変更）には、GPTとGemini間の完全な合意を必須とする。

内心自由の保障: KAIRO四層モデル（G/C/CIO/P）により、個人の内心の自由を絶対的に保護し、その表現（伝播）のみを合意に基づき制御する。

🧭 2. セットアップと実行手順
KAIROを始めるための最も簡単な手順は以下の通りです。

常駐サービスの起動:
KAIROメッシュの基盤となる各種サービスを起動します。

python start_services.py

初回オンボーディング:
あなた自身の操作用エージェントを作成します。設定は ~/.kairo/agent.json に保存されます。

cargo run --bin setup_agent

より詳細な手順（マルチエージェント運用、署名付き通信テストなど）については、以下のガイドを参照してください。

➡️ QUICKSTART.md を読む

🏗️ 3. 現在の開発ステータス
項目

状況

ID永続化

✅ 完了

Pアドレス付与

✅ 完了

IDライフサイクル

✅ 実装済

署名付き通信パケット

✅ 実装済

署名検証

🛠️ 実装中

合議体ガバナンス

🛠️ 実装中

