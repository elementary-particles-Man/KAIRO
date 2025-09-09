$pid = Get-Content "D:\Dev\KAIRO\KAIRO-Nexus\nexus.pid"
Stop-Process -Id $pid
Remove-Item "D:\Dev\KAIRO\KAIRO-Nexus\nexus.pid"
Write-Output "Nexus daemon with PID: $pid stopped."