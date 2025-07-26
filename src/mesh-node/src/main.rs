use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
struct Args {
    /// Log file path
    #[arg(long)]
    log: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut logfile = args
        .log
        .as_ref()
        .and_then(|p| OpenOptions::new().create(true).append(true).open(p).ok());

    println!("Mesh node started.");
    if let Some(file) = logfile.as_mut() {
        let _ = writeln!(file, "Mesh node started.");
    }
}
