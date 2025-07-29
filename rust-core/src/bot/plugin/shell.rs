//! src/bot/plugin/shell.rs
// A simple plugin to execute shell commands.

use tokio::process::Command;

/// Execute a shell command asynchronously.
pub async fn execute(command: &str) -> bool {
    println!("Plugin(Shell): Running '{}'", command);

    let mut parts = command.split_whitespace();
    let program = parts.next().unwrap_or_default();
    let args: Vec<&str> = parts.collect();

    let Ok(mut child) = Command::new(program).args(&args).spawn() else {
        eprintln!("Plugin(Shell): Failed to spawn command.");
        return false;
    };

    match child.wait().await {
        Ok(status) => status.success(),
        Err(_) => {
            eprintln!("Plugin(Shell): Command failed to run.");
            false
        }
    }
}

