# ファイルパス定義
$senderPath = "src\agent\signed_sender.rs"
$mainPath = "src\kairo-daemon\main.rs"

# ① payload未定義の修正：payload行の上にlet文を挿入
(Get-Content $senderPath) | ForEach-Object {
    if ($_ -match 'payload:\s*payload\.clone\(\)') {
        '        let payload = message.clone();'  # 仮に message が payload の元
        $_
    } else {
        $_
    }
} | Set-Content $senderPath

# ② signature型不一致：signature -> hex::encode(signature)
(Get-Content $senderPath) -replace '^\s*signature\s*,', '        signature: hex::encode(signature),' | Set-Content $senderPath

# ③ hyper::Server の import 修正（axum前提）
(Get-Content $mainPath) -replace 'use hyper::Server;', 'use axum::Server;' | Set-Content $mainPath

Write-Host "✅ パッチ適用完了 - 再度 `cargo build` を実行してください"
