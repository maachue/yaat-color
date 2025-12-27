use std::time::Instant;

use anyhow::{Result, bail};
use clap::Parser;
use owo_colors::OwoColorize;
use palette::Srgb;

use crate::{
    cli::Mode,
    colors::convert::FromHexToSrgbf32,
    utils::{DOING_WORK_MSG, WARN_MSG},
};

mod cli;
mod colors;
mod display;
mod dms;
mod utils;

fn main() -> Result<()> {
    let cmd = cli::Cli::parse();

    if cmd.verbose {
        eprintln!(
            "{} ANSI palette with `{}` backend and {} mode",
            DOING_WORK_MSG.style("   Calculating"),
            cmd.backend,
            cmd.mode
        )
    }

    let start: Option<Instant> = match cmd.verbose {
        true => Some(Instant::now()),
        false => None,
    };

    let _light_huh = match cmd.mode {
        Mode::Dark => false,
        Mode::Light => true,
    };

    let color: Srgb<f32> = if cmd.from_srgb {
        let rgb: Vec<&str> = cmd.color.split(",").collect();

        let (r, g, b) = match rgb.as_slice() {
            [first, second, third] => (
                first.parse::<f32>()?,
                second.parse::<f32>()?,
                third.parse::<f32>()?,
            ),
            _ => bail!("Rgb value is incorrect!"),
        };
        Srgb::new(r, g, b).into_format()
    } else {
        Srgb::from_hex(&cmd.color)?.into_format()
    };

    let color = match cmd.backend {
        cli::BackEnd::Dms => dms::generate_ansi_dps(&color)?,
        cli::BackEnd::DmsWcag => {
            eprintln!("{}: `{}` backend is not supported", WARN_MSG, cmd.backend);
            eprintln!("{}: using `DMS` backend instead", "note".bold());
            dms::generate_ansi_dps(&color)?
        }
    };

    if let Some(start) = start {
        let elapsed = start.elapsed();
        eprintln!(
            "{} calculate in {}",
            DOING_WORK_MSG.style("    Finished"),
            utils::format_duration(elapsed),
        );
    }

    match cmd.json_dump {
        true => display::json_dump(&color)?,
        false => {
            for (index, color) in color.normal.iter().chain(color.bright.iter()).enumerate() {
                println!("{} = \"{}\"", index, color)
            }
        }
    }

    Ok(())
}
