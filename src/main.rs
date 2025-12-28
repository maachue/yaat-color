use std::time::Instant;

use anyhow::{Result, bail};
use clap::Parser;
use owo_colors::OwoColorize;
use palette::Srgb;

use crate::{
    cli::{DumpMode, Mode},
    colors::convert::FromHexToSrgbf32,
    utils::{DOING_WORK_MSG, ERR_MSG, WARN_MSG},
};

mod colors;

mod cli;
mod display;
mod utils;

mod dms;

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

    let colors = match cmd.backend {
        cli::BackEnd::Dms => dms::generate_ansi_dps(&color)?,
        cli::BackEnd::DmsWcag => {
            eprintln!("{}: `{}` backend is not supported", WARN_MSG, cmd.backend);
            eprintln!("{}: using `DMS` backend instead", "note".bold());
            dms::generate_ansi_dps(&color)?
        }
        #[allow(unreachable_patterns)]
        _ => bail!("{}: `{}` backend is not supported", ERR_MSG, cmd.backend),
    };

    if let Some(start) = start {
        let elapsed = start.elapsed();
        eprintln!(
            "{} calculate in {}",
            DOING_WORK_MSG.style("    Finished"),
            utils::format_duration(elapsed),
        );
    }

    if cmd.json_dump {
        display::json_dump_simplified(&colors)?;
        return Ok(());
    }

    match cmd.dump {
        DumpMode::HumanReadable => {
            for (index, color) in colors.normal.iter().chain(colors.bright.iter()).enumerate() {
                println!("{} = \"{}\"", index, color)
            }
        }
        DumpMode::JsonSimplified => display::json_dump_simplified(&colors)?,
        DumpMode::JsonPretty => display::json_dump_pretty(&colors)?,
    }

    Ok(())
}
