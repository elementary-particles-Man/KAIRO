//! src/bot/plugin/shell.rs
// A simple plugin to execute shell commands.

use tokio::process::Command;

/// Execute a shell command asynchronously.
pub async fn execute(command: &str) -> bool {
    println!("Plugin(Shell): Running '{}'", command);

    #[cfg(target_os = "windows")]
    let mut command_builder = Command::new("cmd");
    #[cfg(target_os = "windows")]
    command_builder.arg("/C").arg(command);

    #[cfg(not(target_os = "windows"))]
    let mut command_builder = Command::new(program);
    #[cfg(not(target_os = "windows"))]
    command_builder.args(&args);

    let Ok(mut child) = command_builder.spawn() else {
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

