# ================================
# merge_codex_with_main.ps1
# ================================
# 例: PS D:\Dev\KAIRO> .\merge_codex_with_main.ps1
# ================================

# 設定値（自分のCodexブランチ名を必ず合わせて下さい）
$codexBranch = "codex/pcap-template"  # ← 実際のブランチ名に置換

Write-Host "=== STEP 1: ルートで実行されているか確認 ==="
$pwd.Path

Write-Host "=== STEP 2: 現在のブランチ確認 ==="
git branch -a

Write-Host "`n=== STEP 3: Codexブランチに切り替え ==="
git checkout $codexBranch

Write-Host "`n=== STEP 4: 最新 main をフェッチ ==="
git fetch origin

Write-Host "`n=== STEP 5: main をマージ ==="
git merge origin/main

Write-Host "`n=== STEP 6: コンフリクトがあれば手動解決して Enter を押す ==="
Read-Host "競合を解決したら Enter を押して続行"

Write-Host "`n=== STEP 7: 変更をステージ ==="
git add .

Write-Host "`n=== STEP 8: マージコミット ==="
git commit -m "Resolved merge conflict with main"

Write-Host "`n=== STEP 9: 強制プッシュ ==="
git push origin HEAD --force

Write-Host "`n=== 完了！=== "
