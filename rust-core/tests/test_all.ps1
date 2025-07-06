# ===============================
# 📄 test_all.ps1
# KAIRO rust-core 全テスト一括実行
# ===============================

Write-Host "==========================================="
Write-Host " 🚀 Starting KAIRO Rust-Core Full Test Suite"
Write-Host "==========================================="

# ワークスペースルートに移動（必要に応じて変更）
Set-Location -Path "$PSScriptRoot/.."

# Cargo クリーンで古いアーティファクトを削除
Write-Host "`n🧹 Running cargo clean..."
cargo clean

# Cargo ビルド（ビルドエラー事前検知）
Write-Host "`n🔨 Running cargo build..."
cargo build

# Cargo テスト一括実行
Write-Host "`n🧪 Running cargo test --all..."
cargo test --all

Write-Host "`n✅ All tests finished."

# Exitコードを正しく返す（CI/CD対応）
if ($LASTEXITCODE -eq 0) {
    Write-Host "`n🎉 SUCCESS: All tests passed!"
    exit 0
} else {
    Write-Host "`n❌ ERROR: Some tests failed!"
    exit 1
}
