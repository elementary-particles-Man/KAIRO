KAIRO-Nexus
KAIRO-Nexusは、異なるAI（LLM）インターフェース間でのコミュニケーションを自動化するための「中間管理職」デーモンです。ファイルシステムを介してタスクを受け取り、指定されたUIアプリケーションを操作して、結果を返信します。

概要
このシステムは、LLM主体の自律開発環境を構築するための重要なコンポーネントです。人間が手動で行っていたコピー＆ペーストやアプリケーションの切り替えといった操作を自動化し、Gemini, GPT, CLIといった異なるエージェント間のシームレスな連携を実現します。

現在のメイン実装は Nexus/src/json_daemon.py です。

主な機能
タスク監視: 指定されたディレクトリ (inbox_json/) を監視し、新しいタスクファイル（JSON形式）を自動的に検出します。

UIオートメーション: pywinautoライブラリを利用して、指定されたプロセスID（PID）のウィンドウを特定し、テキストの入力や送信（Enterキー押下）といった操作を自動で行います。

堅牢な出力キャプチャ: 操作対象のUIから応答テキストを、ベースライン比較や安定待機ロジックを用いて堅牢にキャプチャします。

非同期リレー: あるAIからの指示を別のAIのUIに中継し、その応答を元のAIに返信します。

ロギングとアーカイブ: 処理したタスクは、成功・失敗に応じてタイムスタンプ付きでprocessed_json/ディレクトリにアーカイブされます。

設定方法
config/addresses.json の設定:
コミュニケーションに参加する各エージェント（Gemini, GPT, CLIなど）のウィンドウのプロセスID（PID）をあらかじめ調べて、このファイルに記述します。
pid_detect.pyユーティリティによる自動検出もサポートされています。

{
    "GPT": { "process_name": "ChatGPT.exe" },
    "Gemini": { "process_name": "chrome.exe", "title_regex": ".*Gemini.*" },
    "CLI": { "process_name": "WindowsTerminal.exe" }
}

config/settings.json の設定 (任意):
ポーリング間隔やUI操作の待機時間などをカスタマイズできます。

{
    "poll_interval_sec": 5.0,
    "pre_submit_delay_sec": 0.5,
    "response_capture_timeout_sec": 60.0,
    "response_stability_wait_sec": 1.5,
    "response_poll_interval_sec": 0.3,
    "min_growth_chars": 50
}

実行方法
必要なPythonライブラリをインストールします。

pip install pywinauto psutil

デーモンを起動します。

python Nexus/src/json_daemon.py

inbox_json/ディレクトリに、以下のようなJSON形式のタスクファイルを作成します。

task_example.json

{
    "from": "CLI",
    "to": "GPT",
    "intent": "chat",
    "payload": {
        "text": "こんにちは、GPT。KAIRO-Nexusからのテストメッセージです。"
    },
    "trace": {
        "id": "trace-12345"
    }
}

ファイルが作成されると、Nexusデーモンが自動的にタスクを実行します。

このプロジェクトは、THPおよびKAIROの思想に基づき、LLMによる自律的な世界の構築を目指すものです。