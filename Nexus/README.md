KAIRO-Nexus MVP (Windows)

シンプルなファイル監視型IPCツール（MVP）。Windowsで動作し、`inbox/`に置いたテキストを指定プロセス（例: ChatGPT, Gemini）へ送信します。

構成
- `nexus/`: メインループ、@stakキュー、Windows送信スタブ
- `config/settings.json.example`: ポーリング間隔・タイムアウト等
- `config/addresses.json.example`: 宛先（PID）テンプレート
- `scripts/detect_pids.ps1`: ChatGPT/GeminiのPID検出ヘルパー
- 監視フォルダ: `inbox/`（実行時自動作成）
- アーカイブ: `processed/ok/`, `processed/error/`（実行時自動作成）

前提
- Windows 10/11
- Python 3.10+
- （推奨）`pip install pywinauto`

使い方
1. 設定を用意
   - `config/settings.json.example` を参考に `config/settings.json` を作成（任意）。
   - `auto_detect_pids` が `true` なら、起動時にChatGPT/GeminiのPIDを自動検出してメモリ上で反映します。
2. 宛先の設定
   - `config/addresses.json.example` を参考に `config/addresses.json` を作成（任意）。
   - 未作成でも `auto_detect_pids` が有効なら自動検出結果を使用可能です。
   - 明示的に固定したい場合は `pid` を記入してください。
3. PID 検出（手動で確認したい場合）
   - `pwsh -NoProfile -File scripts/detect_pids.ps1` を実行。
   - `-OutFile config/pid_candidates.json` を付けるとJSONに保存します。
4. 起動
   - `python -m nexus.main` を実行。
5. 送信
   - `inbox/` に `.txt` を置くと、先頭の非空行が `@stak` ならメモリキューへ積み、そうでなければ宛先にタイプ送信します（Enter送信）。

仕様（MVP）
- ポーリング: 5秒（デフォルト）
- 宛先: 単一（`default_address_key`）
- 独自命令: `@stak` のみ（メモリ内キュー）
- 応答取得: 未実装（スタブ）。タイムアウト到達でエラーとしてアーカイブ。
- エラー処理: 送信失敗/タイムアウト時は `processed/error/` に移動。

既知の制約
- ブラウザ/Electron由来のPIDは変動します。安定運用には `auto_detect_pids` か、起動直前の手動更新を推奨。
- `pywinauto` 未導入の場合、送信はスキップされます。

OCR（任意機能）
- 設定: `ocr_enabled: true`, `tesseract_path: "C:\\Program Files\\Tesseract-OCR\\tesseract.exe"`, `ocr_lang: "jpn"`
- 待機: `response_timeout_sec` 経過後に対象ウィンドウをOCRし、サイドカー `*-ocr.txt` を `processed/ok/` に保存。
- 依存: `pillow`, `pytesseract`（`pip install -r requirements.txt`）。Tesseract本体は手動インストールが必要。

---
## KAIRO-Nexus 構想 (v2)

KAIRO-Nexusは、AI、CLI、各種UIアプリケーション間の連携を自動化する「中間管理職」として機能する。
主な要件は以下の通り。

- **ネットワーク非依存**: `127.0.0.1`のローカル環境で完結し、外部ネットワークを必要としない。
- **アドレス・PID管理**: ニックネームとして機能する「アドレス」と、WindowsのプロセスID (PID) を紐付ける。マッピングは手動管理を基本とする。
- **ファイルベースIPC**: 特定のフォルダ (`inbox_json`) を監視。JSON形式の「通信ファイル」をトリガーとして動作する。
- **通信ワークフロー**:
    1. **検知**: `inbox_json`にファイルが作成されると、処理を開始。
    2. **解析**: ファイルを読み込み、送信元 (`from`)、送信先 (`to`)、ペイロード (`payload`) を解析。
    3. **擬似送信**: 「送信先」アドレスからPIDを特定し、そのウィンドウをフォアグラウンド化。ペイロードを貼り付け、Enterキーで送信をシミュレートする。
    4. **応答キャプチャ**: 一定時間（例: 30秒）待機後、「送信先」ウィンドウをOCRで読み取り、出力テキストをすべて取得する。
    5. **返信**: 「送信元」アドレスのPIDを特定し、そのウィンドウをフォアグラウンド化。キャプチャした応答テキストを貼り付け、Enterキーで返信する。
    6. **アーカイブ**: 処理済みの通信ファイルを `processed_json` フォルダに移動する。
- **高度な機能**:
    - **コマンドシステム**: `@stak`, `@wait`, `@stop` のような内部コマンドを実装し、処理フローを制御する。
    - **状況認識**: 全ての参加UIが、他のUIの存在とNexusの操作方法を把握している状態を目指す。

---

## 実装計画 (Phase 1)

上記の構想を実現するため、まずは中核機能である**「リクエスト→レスポンス」の往復通信サイクル**を実装する。

- **対象モジュール**: `nexus/json_daemon.py`
- **実装ステップ**:
    1. 通信ファイルから「送信元」と「送信先」のアドレスを特定するロジックを強化する。
    2. 「送信先」へのメッセージ送信後、指定時間待機し、OCRで応答をキャプチャする処理を追加する。
    3. キャプチャした応答を「送信元」のウィンドウへ返信する処理を追加する。
    4. 上記の処理が完了した後、通信ファイルを正常にアーカイブする。

この第一段階が完了次第、`@stak`以外の高度なコマンドシステムや、より柔軟なエラーハンドリング機能の実装に進む。
