use clap::Parser;

// Arguments for the clap-test application
#[derive(Parser)]
#[command(name = "clap-test-name", version)]
struct Args {}

fn main() {
    Args::parse();
}