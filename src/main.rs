use anyhow::Result;
use clap::Parser;
use image::{io::Reader, GenericImageView};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let image = Reader::open(args.path)?.decode()?;
    println!("{:?}", image.dimensions());
    Ok(())
}
