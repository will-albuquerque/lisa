use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.path.display());
}
