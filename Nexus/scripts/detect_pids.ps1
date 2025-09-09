param(
  [string]$OutFile
)

$ErrorActionPreference = 'SilentlyContinue'

function Get-TopWindowPid {
  param([string]$ProcessName)
  $p = Get-Process $ProcessName -ErrorAction SilentlyContinue | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
  if ($p) { return @{ pid = $p.Id; title = $p.MainWindowTitle; name = $p.ProcessName } }
  return $null
}

function Find-GeminiInChrome {
  $candidates = Get-Process chrome -ErrorAction SilentlyContinue |
    Where-Object { $_.MainWindowHandle -ne 0 -and $_.MainWindowTitle -match 'Gemini' } |
    Select-Object -First 1
  if ($candidates) { return @{ pid = $candidates.Id; title = $candidates.MainWindowTitle; name = $candidates.ProcessName } }
  return $null
}

$result = [ordered]@{}
$result.chatgpt_app   = Get-TopWindowPid -ProcessName 'ChatGPT'
$result.gemini_chrome = Find-GeminiInChrome
$result.pwsh_current  = @{ pid = $PID; name = 'pwsh'; title = (Get-Process -Id $PID).MainWindowTitle }
$terminal = Get-Process WindowsTerminal -ErrorAction SilentlyContinue | Select-Object -First 1
if ($terminal) { $result.terminal = @{ pid = $terminal.Id; name = $terminal.ProcessName; title = $terminal.MainWindowTitle } }

$json = $result | ConvertTo-Json -Depth 4
if ($OutFile) {
  $dir = Split-Path -Parent $OutFile
  if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir | Out-Null }
  $json | Out-File -FilePath $OutFile -Encoding UTF8
  Write-Host "Wrote PID candidates to: $OutFile"
} else {
  $json
}

