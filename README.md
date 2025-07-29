# KAIRO

> 🛡 This project is licensed under the **GNU Affero General Public License v3.0** (AGPL-3.0). See [LICENSE](./LICENSE) for details.

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

### 📖 CLIの利用方法

CLIの詳しい利用方法については、[usage_cli.md](./usage_cli.md) を参照してください。

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

各エージェント（CLIを含む）は、独立した設定ファイル (`agent_configs/{agent_name}.json`) を持つ必要があります。これにより、各々が固有のIDとPアドレスを保持します。

**注意:** 新しいエージェントをセットアップする際、またはPアドレスの重複を避けるために、デーモンを再起動する前に `agent_registry.json` ファイルを削除することを推奨します。

1.  **エージェントの初期化:**
    以下のコマンドを実行し、`agent_configs/{agent_name}.json`を生成します。`--name`にはエージェントの名前を指定し、`--new`フラグを付けて新しい鍵ペアとPアドレスを生成します。
    ```bash
    # 例: CLIエージェントをセットアップする場合
    cargo run --package kairo_agent --bin setup_agent -- --name CLI --new
    # 例: Agent1をセットアップする場合
    cargo run --package kairo_agent --bin setup_agent -- --name Agent1 --new
    ```
    これにより、`agent_configs/CLI.json` や `agent_configs/Agent1.json` などが生成されます。他のエージェントも同様の手順で初期化してください。

2.  **設定ファイルの検証:**
    生成した `agent_configs/{agent_name}.json` は `validate_config` コマンドで形式チェックができます。
    ```bash
    cargo run --package kairo_agent --bin validate_config -- --path agent_configs/CLI.json
    ```

---

### 📡 3. 通信テストと検証

エージェント間の署名付き通信をテストし、セキュリティが機能していることを確認します。

#### 3.1. メッセージ送信 (signed_sender)

指定したエージェントから別のエージェントのPアドレス宛にメッセージを送信します。`--from`で送信元エージェントの名前を、`--to`で宛先Pアドレスを、`--message`でメッセージを指定します。

```powershell
# PowerShellでの実行例
# Agent1からAgent2へメッセージを送信
cargo run --package kairo_agent --bin signed_sender -- --from Agent1 --to 10.0.0.11/24 --message Hello
```

-   **期待されるサーバーログ:** `kairo_p_daemon`のログに `[SIGNATURE VERIFIED]` と表示され、メッセージがキューイングされます。

#### 3.2. メッセージ受信 (receive_signed)

指定したエージェントのPアドレス宛のメッセージを受信します。`--for-address`で受信するPアドレスを、`--from`で受信するエージェントの名前を指定します。

```powershell
# PowerShellでの実行例
# Agent2が自身のPアドレス宛のメッセージを受信
cargo run --package kairo_agent --bin receive_signed -- --for-address 10.0.0.11/24 --from Agent2
```

-   **期待される出力:** 受信したメッセージと署名検証の結果が表示されます。例: `From [送信元公開鍵]: Hello (signature OK)`

#### 3.3. なりすましテスト（偽署名）

`--fake`フラグを付けて、意図的に署名とペイロードが一致しないパケットを送信します。このメッセージはデーモンによって拒否されるはずです。

```powershell
# PowerShellでの実行例
cargo run --package kairo_agent --bin signed_sender -- --from Agent1 --to 10.0.0.11/24 --message Fake --fake
```

-   **期待されるサーバーログ:** `kairo_p_daemon`のログに `[SIGNATURE INVALID] Packet REJECTED` と表示され、メッセージがキューイングされないことを確認します。
-   **期待される受信結果:** `receive_signed`でこのメッセージを受信しようとしても、メッセージは表示されません。

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

