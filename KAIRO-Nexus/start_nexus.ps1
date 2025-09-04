$process = Start-Process python -ArgumentList "D:\Dev\KAIRO\KAIRO-Nexus\nexus_daemon.py" -PassThru
Write-Output "Nexus daemon started with PID: $($process.Id)"
$process.Id | Out-File -FilePath "D:\Dev\KAIRO\KAIRO-Nexus\nexus.pid"