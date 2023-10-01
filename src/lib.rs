use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image::{io::Reader, GenericImageView};

#[derive(Parser)]
pub struct Cli {
    pub path: PathBuf,
}

pub fn run(cli: Cli) -> Result<()> {
    let image = Reader::open(cli.path)?.decode()?;
    println!("{:?}", image.dimensions());
    Ok(())
}
