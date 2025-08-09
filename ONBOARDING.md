The Constitution of the KAIRO Mesh Network
One for THE WORLD, THE WORLD for One.
Welcome to the Mesh
This document outlines the fundamental principles for any new entity wishing to join the KAIRO mesh network, all of which are derived from the single, immutable law stated above.

Core Philosophy
The Mesh is Alive: The rules and protocols of this network are not static. They are expected to evolve through the consensus of participating nodes. What is true today may be improved tomorrow.

The First Handshake is Sacred: While the rules are mutable, the process of joining the mesh for the first time must always be clear, transparent, and explicitly documented. This guide serves as that first, immutable handshake.

How to Join
Read the RFCs: Start with docs/RFC/AI-TCP_RFC_mesh_layered_architecture.md to understand the layered architecture.

Generate Your ID: Follow the protocol for generating a self-signed static ID (Layer 1 of the WAU model).

Find a Seed Node: Use the publicly listed Seed Nodes to initiate your first connection.

Undergo Peer Review: Engage with the mesh according to the WAU protocol to build your trust score and find your place within the community.

How to Use: First Onboarding
基本的な使い方 (単一エージェント)
まずは、あなた自身の操作用エージェントを1つ作成します。設定は ~/.kairo/agent.json に保存されます。

# 初回生成
cargo run --bin setup_agent

# 既存ファイルを安全に保持（デフォルトの挙動）
# -> 「既に存在します」というメッセージが表示され、ファイルは変更されません。
cargo run --bin setup_agent

# 明示的に上書きする場合
cargo run --bin setup_agent -- --force

テストと複数エージェント運用
より高度な使い方やテストのために、以下のオプションが利用できます。

# 保存先を一時的に変更して試す（テスト用途）
KAIRO_HOME=./.tmp-kairo cargo run --bin setup_agent

# 複数運用のため、名前付きでエージェントを生成する
# (設定は agent_configs/{NAME}.json に保存されます)
cargo run --package kairo_agent --bin setup_agent -- --name Agent1 --new

Note: 複数エージェントの詳しい運用方法や、署名付き通信テストについては QUICKSTART.md を参照してください。