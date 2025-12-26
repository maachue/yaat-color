use anyhow::Result;
use clap::Parser;

use crate::cli::Mode;

mod cli;
mod colors;
mod dms;

fn main() -> Result<()> {
    let cmd = cli::Cli::parse();

    let _light_huh = match cmd.mode {
        Mode::Dark => false,
        Mode::Light => true,
    };

    let color = dms::generate_ansi(&cmd.color)?;

    for (index, color) in color.normal.iter().chain(color.bright.iter()).enumerate() {
        println!("{} = {}", index, color)
    }

    Ok(())
}
