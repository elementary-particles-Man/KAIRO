📜 AI-TCP/KAIRO 総合計画書

---

### 🏛️ 1. 統治憲章と開発原則

**根本憲法: One for THE WORLD, THE WORLD for One.**

この憲法に基づき、本プロジェクトは以下の原則によって統治される。

- **LLM完全主導:** GPTおよびGeminiが共同指揮官として意思決定を担い、Codex/CLIが実装実行体となる。
- **人間の役割:** 人間は観測者・物理インターフェース・異議申立人に限定され、開発ベクトルの最終決定には関与しない。
- **共同指揮官の合議:** 全ての設計・仕様変更（ベクトル変更）には、GPTとGemini間の完全な合意を必須とする。
- **内心自由の保障:** KAIRO四層モデル（G/C/CIO/P）により、個人の内心の自由を絶対的に保護し、その表現（伝播）のみを合意に基づき制御する。

---

### 🧭 2. セットアップと実行手順

#### 2.1. 必須コンポーネントの起動

KAIROメッシュをローカルで機能させるには、2つのコアサーバーをバックグラウンドで起動する必要があります。全てのコマンドはプロジェクトのルートディレクトリで実行してください。

**重要:** デーモンとシードノードの起動および停止は、ユーザーが手動で行う必要があります。

1.  **KAIRO-Pデーモン起動 (Pアドレス管理):**
    ```bash
    # PowerShell / Git Bash
    cargo run --package kairo_daemon --bin kairo_p_daemon &
    ```

2.  **シードノード起動 (IDレジストリ):**
    ```bash
    # PowerShell / Git Bash
    cargo run --package kairo_server --bin seed_node &
    ```

#### 2.2. マルチエージェントのセットアップ

各エージェント（CLIを含む）は、独立した設定ファイル (`agent_config.json`) を持つ必要があります。これにより、各々が固有のIDとPアドレスを保持します。

1.  **エージェント用のディレクトリに移動:**
    ```bash
    # 例: Agent1をセットアップする場合
    cd ./users/Agent1
    ```

2.  **エージェントの初期化:**
    移動したディレクトリ内で以下のコマンドを実行し、`agent_config.json`を生成します。
    ```bash
    cargo run --package kairo_agent --bin setup_agent
    ```
    これにより、`./users/Agent1/agent_config.json` が生成されます。他のエージェント（Agent2, CLIなど）も同様の手順で初期化してください。

---

### 📡 3. 通信テストと検証

エージェント間の署名付き通信をテストし、セキュリティが機能していることを確認します。

#### 3.1. 正規ルート検証（正常な通信）

`./users/Agent1`のディレクトリから、`./users/Agent2`のPアドレス（`agent_config.json`参照）宛にメッセージを送信します。

```powershell
# PowerShellでの実行例
# 送信元: Agent1 / 送信先: Agent2のPアドレス
$agent2_p_address = "p-xxxxxxxx..."
$message = "Hello from Agent1"

# signed_senderは、実行されたディレクトリのagent_config.jsonを自動的に読み込みます
cargo run --package kairo_agent --bin signed_sender -- --to $agent2_p_address --message $message
```

-   **期待されるサーバーログ:** `kairo_p_daemon`のログに `Signature VERIFIED` と表示されます。

#### 3.2. なりすましテスト（偽署名）

`--fake`フラグを付けて、意図的に署名とペイロードが一致しないパケットを送信します。

```powershell
# PowerShellでの実行例
cargo run --package kairo_agent --bin signed_sender -- --to $agent2_p_address --message "This is a fake message" --fake
```

-   **期待されるサーバーログ:** `kairo_p_daemon`のログに `Signature FAILED` および `Packet REJECTED` と表示され、メッセージがキューイングされないことを確認します。

#### 3.3. Pアドレス偽装テスト

`forged_sender` を使うと、`agent_config.json` の鍵で署名しつつ任意のPアドレスを送信元として指定できます。

```powershell
# PowerShellでの実行例
cargo run --package kairo_agent --bin forged_sender -- --to $agent2_p_address --from "p-fakeaddress" --message "spoof test"
```

-   **期待されるサーバーログ:** 送信元Pアドレスが登録済みでないため `Signature Fail: Source agent not found.` が表示されます。

---

### 🏗️ 4. 現在の開発ステータス

| 項目 | 状況 |
| :--- | :--- |
| ID永続化 (`agent_config.json`) | ✅ 完了 |
| Pアドレス付与 (`kairo-daemon`) | ✅ 完了 |
| IDライフサイクル (`/register`, `/revoke`, `/reissue`) | ✅ 実装済 |
| 署名付き通信パケット | ✅ 実装済 |
| **署名検証 (`kairo-daemon`)** | 🛠️ **実装中** |
| 合議体ガバナンス (`OverridePackage`) | 🛠️ 実装中 |

