# ===============================
# 📄 verify_local_llm.ps1
# ===============================
# ローカルLLMステータスを確認するためのPowerShellスクリプト
# 接続先: http://127.0.0.1:60006
# モデル: google/gemma-3-4b

# curl相当のInvoke-RestMethodを使用
Write-Output "Starting local LLM verification..."

# Define log path explicitly at the beginning
$logPath = "D:\Dev\KAIRO\logs\workingLog.txt"

# リクエスト用のJSONボディ
$body = @{
    model = "google/gemma-3-4b"
    messages = @(
        @{
            role = "system"
            content = "You are a local LLM server for KAIRO Mesh."
        },
        @{
            role = "user"
            content = "Return only your current model name."
        }
    )
} | ConvertTo-Json -Depth 5

# 実行
# Simplify Invoke-RestMethod call to a single line
$response = Invoke-RestMethod -Method Post -Uri http://127.0.0.1:60006/v1/chat/completions -ContentType "application/json" -Body $body

# 結果の出力
$modelName = $response.choices[0].message.content
Write-Output "LLM Response Model Name: $modelName"

# 結果を workingLog.txt に追記
Write-Host "Resolved Log Path: $logPath" # Debugging log path
Add-Content -Path $logPath -Value "===== Local LLM Verification ===="
Add-Content -Path $logPath -Value "Timestamp: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
Add-Content -Path $logPath -Value "System Fingerprint: $modelName"
Add-Content -Path $logPath -Value "Verification complete.`n"

Write-Output "Local LLM verification finished. Log written to $logPath"