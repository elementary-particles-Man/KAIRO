## 10. AI-TCP エージェント間通信アーキテクチャ

AI-TCPメッシュにおけるエージェント間の通信は、以下の多層的な構造と責任分界によって実現されます。

### 10.1. エージェント間通信の流れ (Agent ↔ Daemon ↔ Agent)

エージェント間の直接通信は行われず、KAIRO-Pデーモン (kairo_p_daemon) を介してメッセージが転送されます。

```
[送信元 Agent (CLI/AgentX)] ---(1. signed_sender)---> [KAIRO-P Daemon] ---(2. キューイング)---> [KAIRO-P Daemon] ---(3. receive_signed)---> [宛先 Agent (CLI/AgentY)]
```

1.  **送信元 Agent (signed_sender)**:
    *   `signed_sender`バイナリは、送信元エージェントの `agent_configs/{agent_name}.json` から秘密鍵と公開鍵を読み込みます。
    *   メッセージペイロードと送信元公開鍵、宛先Pアドレス、その他のメタデータを含む `AiTcpPacket` を構築します。
    *   構築された `AiTcpPacket` のペイロードに対して秘密鍵で署名を行い、署名をパケットに含めます。
    *   この `AiTcpPacket` をKAIRO-Pデーモンの `/send` エンドポイントへHTTP POSTリクエストとして送信します。

2.  **KAIRO-P Daemon (kairo_p_daemon - 送信側処理)**:
    *   `/send` エンドポイントで `AiTcpPacket` を受信します。
    *   受信したパケットの `source_public_key` と `signature` を使用し、パケットの署名を検証します。この際、デーモンは自身の `agent_registry.json` に登録されている公開鍵と照合します。
    *   署名が有効であれば、パケットを宛先 `destination_p_address` に対応する内部メッセージキューに格納します。
    *   署名が無効であれば、パケットを破棄し、`401 Unauthorized` エラーを返します。

3.  **KAIRO-P Daemon (kairo_p_daemon - 受信側処理)**:
    *   宛先エージェントからの `/receive` エンドポイントへのHTTP GETリクエストを受信します。
    *   リクエストされたPアドレスに対応するメッセージキューから、キューイングされた `AiTcpPacket` のリストを返します。
    *   メッセージが返された後、キューからメッセージをクリアします。

4.  **宛先 Agent (receive_signed)**:
    *   `receive_signed`バイナリは、自身のPアドレスを指定してKAIRO-Pデーモンの `/receive` エンドポイントへHTTP GETリクエストを送信します。
    *   デーモンから `AiTcpPacket` のリストを受信します。
    *   受信した各 `AiTcpPacket` について、パケット内の `source_public_key` を使用して署名を検証します。
    *   署名が有効であればメッセージを「(signature OK)」として表示し、無効であれば「(signature INVALID)」として表示します。

### 10.2. 署名・検証の責任所在

各モジュールにおける署名と検証の責任は以下の通りです。

*   **`src/agent/signed_sender.rs`**:
    *   **責任**: メッセージペイロードに対する署名の生成。
    *   **詳細**: 送信元エージェントの秘密鍵を使用して、送信する `AiTcpPacket` の `payload` フィールドに対して署名を行い、その結果を `signature` フィールドに格納します。

*   **`src/kairo-daemon/kairo_p_daemon.rs`**:
    *   **責任**: 受信した `AiTcpPacket` の署名検証。
    *   **詳細**: `handle_send` 関数内で、受信したパケットの `source_public_key` と `signature` を使用して、パケットの `payload` が改ざんされていないか、正規の送信元から送られたものかを検証します。検証には、デーモンが管理するエージェントレジストリ (`agent_registry.json`) に登録された公開鍵を使用します。

*   **`src/agent/receive_signed.rs`**:
    *   **責任**: 受信した `AiTcpPacket` の署名検証と表示。
    *   **詳細**: デーモンから取得した `AiTcpPacket` の `source_public_key` を使用して、そのパケットの `payload` と `signature` が一致するかを検証します。これにより、メッセージが転送中に改ざんされていないこと、および正規の送信元から送られたものであることを受信側で再確認します。

### 10.3. 今後導入予定の Mesh 経由転送の想定構造

現在のアーキテクチャでは、すべてのエージェントが単一のKAIRO-Pデーモンを介して通信しています。将来的に、複数のKAIRO-Pデーモンがメッシュネットワークを形成し、メッセージを転送する構造を想定しています。

```
[送信元 Agent] ---(signed_sender)---> [KAIRO-P Daemon A] ---(Mesh転送)---> [KAIRO-P Daemon B] ---(receive_signed)---> [宛先 Agent]
```

この際、`seed_node.rs`が果たす役割は、メッシュネットワーク内の各KAIRO-Pデーモンが自身の管理下にあるエージェントの公開鍵とPアドレスを登録する中央レジストリとして機能することです。

*   **`src/server/seed_node.rs`**:
    *   **役割**: メッシュネットワーク全体のエージェントIDとPアドレスのマッピングを管理する中央レジストリ。
    *   **詳細**: 各KAIRO-Pデーモンは、自身の管理下にあるエージェントがPアドレスを取得する際に、そのエージェントの公開鍵と割り当てられたPアドレスを `seed_node` に登録します。これにより、メッシュ内の任意のデーモンが、他のデーモンが管理するエージェントのPアドレスと公開鍵を解決できるようになります。
    *   メッセージがメッシュを介して転送される際、中間デーモンは `seed_node` を参照して宛先Pアドレスに対応するデーモンを特定し、メッセージをルーティングします。

この多層的なアーキテクチャにより、スケーラビリティと耐障害性を確保しつつ、エージェント間のセキュアな通信を実現します。