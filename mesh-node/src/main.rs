use clap::Parser;

/// Simple mesh node example
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Port to bind the mesh node server
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

// main.rs
fn main() {
    // Remove or comment this line:
    // kairo_rust_core::example_function();
    println!("mesh-node runs!");
}
