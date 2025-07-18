use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    println!("Starting Mesh Node on port: {}", args.port);
    // TODO: ConnectionManager + Packet Validator call
}
