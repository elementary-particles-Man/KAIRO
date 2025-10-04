[CmdletBinding()]
param(
    [ValidateSet("Info", "Build", "Run", "Publish", "Stop")]
    [string]$Action = "Build",
    [string]$RootDirectory,
    [string]$LogDirectory
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptRoot
$projectDir = Join-Path $repoRoot "src/Kairo.ClipboardListener"
$dllPath = Join-Path $projectDir "bin\Release\net8.0-windows10.0.19041.0\Kairo.ClipboardListener.dll"
$publishExePath = Join-Path $projectDir "bin\Release\net8.0-windows10.0.19041.0\win-x64\publish\Kairo.ClipboardListener.exe"
$pwshPath = (Get-Process -Id $PID).Path

if (-not $LogDirectory) {
    $LogDirectory = Join-Path $repoRoot ".codex_logs"
}

New-Item -ItemType Directory -Force -Path $LogDirectory | Out-Null

function Invoke-DotnetCommand {
    param(
        [string[]]$Arguments,
        [string]$WorkingDirectory = $repoRoot,
        [string]$LogPath
    )

    $escapedArgs = foreach ($arg in $Arguments) {
        $escaped = $arg -replace '"', '`"'
        '"{0}"' -f $escaped
    }

    $command = "& dotnet {0}" -f ($escapedArgs -join ' ')

    if ($LogPath) {
        $escapedLog = $LogPath -replace '"', '`"'
        $command = "$command 2>&1 | Tee-Object -FilePath `"$escapedLog`" -Encoding utf8NoBom"
    }

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $pwshPath
    $psi.ArgumentList.Add('-NoLogo')
    $psi.ArgumentList.Add('-NoProfile')
    $psi.ArgumentList.Add('-Command')
    $psi.ArgumentList.Add($command)
    $psi.WorkingDirectory = $WorkingDirectory
    $psi.UseShellExecute = $false

    $process = [System.Diagnostics.Process]::Start($psi)
    $process.WaitForExit()
    $exitCode = $process.ExitCode
    $process.Dispose()
    return $exitCode
}

function Invoke-DotnetInfo {
    $infoLog = Join-Path $LogDirectory "dotnet_info.txt"
    $exitCode = Invoke-DotnetCommand -Arguments @("--info") -LogPath $infoLog
    if ($exitCode -ne 0) {
        throw "dotnet --info failed with exit code $exitCode."
    }
}

function Stop-ListenerProcesses {
    param([string]$TargetDllPath)

    try {
        $listener = Get-Process -Name "Kairo.ClipboardListener" -ErrorAction SilentlyContinue
        if ($listener) {
            $listener | Stop-Process -Force -ErrorAction SilentlyContinue
        }
    } catch {
        Write-Warning "Failed to stop Kairo.ClipboardListener.exe: $($_.Exception.Message)"
    }

    try {
        $dotnetProcs = Get-CimInstance Win32_Process -Filter "Name='dotnet.exe'" -ErrorAction SilentlyContinue
        if ($dotnetProcs) {
            foreach ($proc in $dotnetProcs) {
                $commandLine = $proc.CommandLine
                if (-not $commandLine) {
                    continue
                }

                $shouldStop = $false
                if ($TargetDllPath) {
                    $pattern = [Regex]::Escape($TargetDllPath)
                    if ($commandLine -match $pattern) {
                        $shouldStop = $true
                    }
                } elseif ($commandLine -match "Kairo\.ClipboardListener\.dll") {
                    $shouldStop = $true
                }

                if ($shouldStop) {
                    try {
                        Stop-Process -Id $proc.ProcessId -Force -ErrorAction SilentlyContinue
                    } catch {
                        Write-Warning "Failed to stop process $($proc.ProcessId): $($_.Exception.Message)"
                    }
                }
            }
        }
    } catch {
        Write-Warning "Unable to query dotnet processes: $($_.Exception.Message)"
    }
}

function Remove-DirectoryIfExists {
    param([string]$Path)
    if (Test-Path $Path) {
        Remove-Item -Path $Path -Recurse -Force -ErrorAction Stop
    }
}

function Resolve-RootDirectory {
    param([string]$Override)

    if ($Override) {
        return [System.IO.Path]::GetFullPath($Override)
    }

    if ($env:KAIRO_NEXUS_ROOT) {
        try {
            return [System.IO.Path]::GetFullPath($env:KAIRO_NEXUS_ROOT)
        } catch {
            Write-Warning "Failed to resolve KAIRO_NEXUS_ROOT environment variable: $($_.Exception.Message)"
        }
    }

    $local = [Environment]::GetFolderPath('LocalApplicationData')
    return Join-Path $local "KAIRO\Nexus"
}

switch ($Action) {
    'Info' {
        Invoke-DotnetInfo
    }
    'Build' {
        Invoke-DotnetInfo
        Stop-ListenerProcesses -TargetDllPath $dllPath
        Remove-DirectoryIfExists (Join-Path $projectDir 'bin\Release')
        Remove-DirectoryIfExists (Join-Path $projectDir 'obj\Release')
        $buildLog = Join-Path $LogDirectory 'build.log'
        $exitCode = Invoke-DotnetCommand -Arguments @('build', '-c', 'Release') -WorkingDirectory $projectDir -LogPath $buildLog
        if ($exitCode -ne 0) {
            exit $exitCode
        }
    }
    'Run' {
        Invoke-DotnetInfo
        Stop-ListenerProcesses -TargetDllPath $dllPath
        $root = Resolve-RootDirectory -Override $RootDirectory
        $env:KAIRO_NEXUS_ROOT = $root
        Write-Host "Using KAIRO_NEXUS_ROOT=$root"
        $runLog = Join-Path $repoRoot 'KAIRO_ClipboardListener.runlog.txt'
        $exitCode = Invoke-DotnetCommand -Arguments @($dllPath) -WorkingDirectory $projectDir -LogPath $runLog
        exit $exitCode
    }
    'Publish' {
        Invoke-DotnetInfo
        Stop-ListenerProcesses -TargetDllPath $dllPath
        $publishLog = Join-Path $LogDirectory 'publish.log'
        $arguments = @(
            'publish',
            '-c', 'Release',
            '-r', 'win-x64',
            '--self-contained', 'false',
            '-p:PublishSingleFile=false',
            '-p:DebugType=None',
            '-p:ErrorOnDuplicatePublishOutputFiles=false'
        )
        $exitCode = Invoke-DotnetCommand -Arguments $arguments -WorkingDirectory $projectDir -LogPath $publishLog
        if ($exitCode -ne 0) {
            exit $exitCode
        }
        Write-Host "Publish output: $publishExePath"
    }
    'Stop' {
        Stop-ListenerProcesses -TargetDllPath $dllPath
        Write-Host 'Listener processes terminated if running.'
    }
    default {
        throw "Unsupported action: $Action"
    }
}
