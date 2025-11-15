# Replaced gpt_responder.rs with a *zero-cost* A-Scheme implementation.
- `gpt_responder.rs` を GPT API ではなく example.com へPOSTして remote_addr を取得する軽量実装に差し替えました。

# GPT API is no longer called; all communication goes to https://example.com/.
- `NO_COST_ENDPOINT` を `https://example.com/` に固定し、課金対象 API を完全にバイパスしています。

# Remote SocketAddr is captured from real TLS handshake without any billing.
- `reqwest::Response::remote_addr()` を利用して実際の接続先 IP/Port を取得し、取れない場合はフォールバックを記録します。

# Returned value is now ("OK", remote_addr), suitable for ClearMini witness.
- `gpt_log_and_respond()` は応答本文を固定文字列 `"OK"` とし、第二要素に取得した `SocketAddr` を返します。

# A-Scheme monitoring is now fully *free* and fully accurate.
- GPT への実通信を行わないためコストゼロで接続情報を監視できるようになりました。
