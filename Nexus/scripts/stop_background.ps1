param(
  [switch]$Force
)

$ErrorActionPreference = 'Continue'

$statePath = Join-Path 'config' 'daemon_pids.json'
if (-not (Test-Path $statePath)) {
  Write-Warning "[bg-stop] No state file: $statePath"
  exit 0
}

try {
  $state = Get-Content -Raw -Path $statePath | ConvertFrom-Json
} catch {
  Write-Warning "[bg-stop] Failed to read state file: $statePath"
  exit 1
}

$stopped = 0
foreach ($k in $state.processes.PSObject.Properties.Name) {
  $info = $state.processes.$k
  if ($null -eq $info) { continue }
  $pid = $info.pid
  if ($pid) {
    try {
      if ($Force) {
        Stop-Process -Id $pid -Force -ErrorAction Stop
      } else {
        Stop-Process -Id $pid -ErrorAction Stop
      }
      Write-Host "[bg-stop] Stopped $k (PID=$pid)"
      $stopped++
    } catch {
      Write-Warning "[bg-stop] Could not stop $k (PID=$pid): $_"
    }
  }
}

try {
  Remove-Item -Path $statePath -Force -ErrorAction Stop
  Write-Host "[bg-stop] Removed $statePath"
} catch {
  Write-Warning "[bg-stop] Failed to remove state file: $statePath"
}

Write-Host "[bg-stop] Stopped=$stopped"

