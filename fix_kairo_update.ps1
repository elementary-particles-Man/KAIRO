# ================================
# KAIRO 依存バージョン整理＆ログ保存 (修正版)
# ================================

# 正しく true / false で設定する
$Env:CARGO_HTTP_CHECK_REVOKE = "false"

# タイムスタンプ付きログファイル名
$ts = Get-Date -Format "yyyyMMdd_HHmmss"
$logfile = "kairo_update_$ts.log"

Write-Output "=== Current Directory ===" | Tee-Object -FilePath $logfile -Append
Get-Location | Tee-Object -FilePath $logfile -Append

Write-Output "`n=== Cargo.toml [dependencies] ===" | Tee-Object -FilePath $logfile -Append
Get-Content Cargo.toml | Tee-Object -FilePath $logfile -Append

Write-Output "`n=== cargo update ===" | Tee-Object -FilePath $logfile -Append
cargo update --verbose 2>&1 | Tee-Object -FilePath $logfile -Append

Write-Output "`n=== cargo tree (rand_core) ===" | Tee-Object -FilePath $logfile -Append
cargo tree | Select-String "rand_core" | Tee-Object -FilePath $logfile -Append

Write-Output "`n=== cargo build ===" | Tee-Object -FilePath $logfile -Append
cargo build 2>&1 | Tee-Object -FilePath $logfile -Append

Write-Output "`n=== cargo test ===" | Tee-Object -FilePath $logfile -Append
cargo test 2>&1 | Tee-Object -FilePath $logfile -Append

Write-Output "`n=== All steps done. Log saved: $logfile ===" | Tee-Object -FilePath $logfile -Append
