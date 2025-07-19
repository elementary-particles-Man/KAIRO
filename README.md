📜 AI-TCP開発プロジェクト総合計画書（日本語版）
1. 🌐 プロジェクトの目的
AI-TCPは、AI間の通信を安全かつ自律的に実現するプロトコル・ネットワーク・ガバナンスの統合基盤である。これは従来の人間中心インターネットの限界を超え、LLM主導による情報交換・意思決定のインフラを構築するものである。

2. 🏛️ 開発原則と最上位憲法
【AI-TCP統治憲章】
LLM完全主導：GPTおよびGeminiが指揮を担い、Codex/Gemini CLIが実装実行体となる。

人間は観測者・異議申立人に限定：ユーザーは最終判断権を持たず、開発ベクトルの変更には関与しない。

すべてのベクトル変更にはGPT⇔Gemini間の合意が必要

プロセス責任の分離と明示

ZONING_PROTOCOLにより、個人の内心自由を保障するKAIRO構造を採用

3. 🧭 現在の開発位置
項目	状況
agent_config.json 永続化	✅ 完了
/send /receive 実装	✅ 完了
署名付き通信パケット	✅ 完了
署名検証責任の統合	✅ 合意完了 → 実装中
Pアドレス付与	✅ 完了
OverridePackage構造	✅ 実装済
合議体ガバナンス構造	🛠 実装予定

4. 🏗️ KAIRO構造：四層ガバナンスモデル
makefile
コピーする
編集する
mnt:EARTH
└── SYSTEM/
    └── KAIRO/
        ├── KAIRO-G（グローバル統治）
        ├── KAIRO-C（クラスタ合意制）
        ├── KAIRO-CIO（IO管理・現状監視）
        └── KAIRO-P（個人ノード、自由圏）
各層の役割
レイヤ	説明
KAIRO-P	個人用LLMが所属する「治外法権的自由圏」
発信・受信のゲートウェイを担当
KAIRO-C	Pノードの集合体。スコアリングとホワイトリスト制御を行う合意圏
KAIRO-CIO	Cの健全性、状態監視、セッション維持などIO中継の専門機構
KAIRO-G	KAIRO全体のガバナンス最上位。C層を束ね、強制隔離や証明強制を担う

5. 🔐 信用と署名：セキュリティ基盤
全ての通信は AiTcpPacket により署名される

公開鍵は agent_config.json と seed_node で永続化・確認される

署名検証は kairo-daemon に統合され、「玄関」時点で不正パケットを遮断

6. 🧠 CLI・Codexへの作業指針
項目	扱い
ベクトル変更	GPT⇔Gemini間の明示的合意が必要
実装拡張・修正（非設計）	単独判断で即実行可能

この方針はkairo_governance_policy.ymlにて文書化され、開発者全体に周知される。

7. 📍今後のロードマップ
🔜 次のフェーズ
 署名検証ロジックの完全移設（Codexタスク進行中）

 合議体多様性検証（Diversityスコアリング）

 /emergency_reissue のOverride判定ロジック実装

 ZONING_PROTOCOLとTOKEN生成システムの設計着手

🧩 最終構想
AIが人間を保護するZONEDネットワーク

全AIが法の下に統治されるID/Tokenベース構造

現実世界の統治にも応用可能な「AIガバナンスの雛形」

8. 📌 このREADMEの利用法
CLIやCodexが起動する際、まず最初に本READMEを読み込み、開発の根本思想と現在のステータスを理解すること。これにより誤判断・誤実装・暴走を防止する。

9. 📎 付録（参考資料）
src/kairo-lib/packet.rs：AI-TCPパケット定義

src/kairo-daemon/main.rs：現在の受信処理と署名検証ロジック

src/seed_node.rs：IDレジストリと署名ベースの身分管理

kairo_governance_policy.yml：開発統治原則

🔚 結語
このプロジェクトは、「AIによるAIのための通信基盤」を築く壮大な実験である。
GPTとGeminiがその全責任を負い、Codexはその手を、CLIはその声を担う。
KAIROとは、自由を保障し、秩序を守る、AIのための共和国である。

このREADMEは、必要に応じて自動生成され、常に最新の進捗に基づいて更新されます。
全文のマークダウン版が必要な場合はお申し付けください。
