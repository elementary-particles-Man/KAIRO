use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use chrono::Utc;
use log::info;

/// GPT応答をログファイルに追記保存する
pub async fn log_gpt_response(response: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let log_dir = Path::new("logs");
    let log_file = log_dir.join("gpt_response_log.jsonl");

    // ログディレクトリがなければ作成
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)?;
        info!("📁 Created log directory: {:?}", log_dir);
    }

    // 書き込みオープン
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)?;

    // タイムスタンプ付きで記録
    let timestamp = Utc::now().to_rfc3339();
    let log_entry = format!(r#"{{"timestamp": "{}", "response": {}}}"#, timestamp, response);

    writeln!(file, "{}", log_entry)?;

    info!("📝 Logged GPT response to: {:?}", log_file);
    Ok(())
}
