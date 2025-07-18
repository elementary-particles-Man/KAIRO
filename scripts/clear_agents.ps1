$agentDir = "agent_configs"
if (Test-Path $agentDir) {
    Get-ChildItem -Path $agentDir -Filter "agent_config_*.json" | Remove-Item -Force
    Write-Host "All agent_config_*.json files removed from $agentDir"
} else {
    Write-Host "Directory '$agentDir' does not exist."
}
