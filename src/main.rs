use anyhow::Result;

use clap::Parser;
use lisa::{run, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}
