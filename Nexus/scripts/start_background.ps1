param(
  [switch]$NoVenv,
  [switch]$Hidden,
  [switch]$OnlyJson,
  [switch]$OnlyMain,
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

function Ensure-Requirements($py) {
  & $py -m pip install --upgrade pip
  if (Test-Path 'requirements.txt') {
    & $py -m pip install -r requirements.txt
  }
}

function Resolve-PythonForBg($py, [switch]$Hidden) {
  # Prefer pythonw.exe when Hidden for no console window
  $pyDir = Split-Path -Parent $py
  $pyw = Join-Path $pyDir "pythonw.exe"
  if ($Hidden -and (Test-Path $pyw)) { return $pyw }
  return $py
}

Write-Host "[bg] Preparing environment..."
$py = Ensure-Venv
Ensure-Requirements $py

# Refresh PID candidates (best effort)
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/detect_pids.ps1 -OutFile config/pid_candidates.json | Out-Null

$bgPy = Resolve-PythonForBg -py $py -Hidden:$Hidden
$windowStyle = if ($Hidden) { 'Hidden' } else { 'Minimized' }

$pids = [ordered]@{}
$ts = Get-Date -Format "yyyy-MM-ddTHH:mm:ssK"

if (-not $OnlyMain) {
  $pJson = Start-Process -PassThru -WindowStyle $windowStyle -FilePath $bgPy -ArgumentList "-m","nexus.json_daemon"
  $pids.json_daemon = @{ pid = $pJson.Id; started = $ts }
  Write-Host "[bg] Started nexus.json_daemon (PID=$($pJson.Id))"
}

if (-not $OnlyJson) {
  $pMain = Start-Process -PassThru -WindowStyle $windowStyle -FilePath $bgPy -ArgumentList "-m","nexus.main"
  $pids.main = @{ pid = $pMain.Id; started = $ts }
  Write-Host "[bg] Started nexus.main (PID=$($pMain.Id))"
}

$meta = @{ updated = $ts; python = $bgPy; windowStyle = $windowStyle; cwd = (Get-Location).Path }
$state = @{ processes = $pids; meta = $meta }

$stateJson = ($state | ConvertTo-Json -Depth 6)
if (-not (Test-Path 'config')) { New-Item -ItemType Directory -Path 'config' | Out-Null }
$stateJson | Out-File -FilePath 'config/daemon_pids.json' -Encoding UTF8
Write-Host "[bg] Wrote process state to config/daemon_pids.json"

Write-Host "[bg] Done. Processes are running in background."

