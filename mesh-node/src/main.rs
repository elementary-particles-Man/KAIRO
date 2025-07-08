use clap::Parser;

/// Simple mesh node example
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Port to bind the mesh node server
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

fn main() {
    let cli = Cli::parse();
    println!("Starting mesh node on port {}", cli.port);
    // Call into the shared rust_core library as a sanity check
    rust_core::example_function();
}
