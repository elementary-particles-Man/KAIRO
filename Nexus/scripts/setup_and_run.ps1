param(
  [switch]$DetectOnly,
  [switch]$NoVenv,
  [switch]$RunJson,          # JSONデーモンも起動
  [switch]$JsonOnly,         # JSONデーモンのみ起動
  [string]$PythonExe = "python"
)

$ErrorActionPreference = 'Stop'

function Ensure-Venv {
  if ($NoVenv) { return $PythonExe }
  $venvPy = Join-Path ".venv" "Scripts/python.exe"
  if (-not (Test-Path $venvPy)) {
    & $PythonExe -m venv .venv
  }
  return $venvPy
}

function Ensure-Requirements($py){
  & $py -m pip install --upgrade pip
  if (Test-Path 'requirements.txt') {
    & $py -m pip install -r requirements.txt
  }
}

Write-Host "[setup] Python executable: $PythonExe"

try {
  $py = Ensure-Venv
} catch {
  Write-Error "Pythonが見つかりません。python.orgからインストールし、PATHに追加してください。"
  exit 1
}

if (-not $DetectOnly) {
  Ensure-Requirements $py
}

# PID検出（情報表示のみ）
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/detect_pids.ps1 -OutFile config/pid_candidates.json | Out-Null
Write-Host "[setup] PID候補を config/pid_candidates.json に書き出しました。"

if ($DetectOnly) { exit 0 }

if ($JsonOnly) {
  # JSONデーモンのみ
  & $py -m nexus.json_daemon
  exit $LASTEXITCODE
}

# 既存メイン起動
if ($RunJson) {
  # JSONデーモンは別プロセスで起動（ウィンドウを増やさない）
  Start-Process -WindowStyle Minimized -FilePath $py -ArgumentList "-m","nexus.json_daemon" | Out-Null
  Write-Host "[setup] nexus.json_daemon を別プロセスで起動しました。"
}

& $py -m nexus.main
