use anyhow::{Result, bail};
use clap::Parser;
use owo_colors::OwoColorize;
use palette::Srgb;

use crate::{cli::Mode, colors::convert::FromHexToSrgbf32, utils::WARNING_MSG};

mod cli;
mod colors;
mod display;
mod dms;
mod utils;

fn main() -> Result<()> {
    let cmd = cli::Cli::parse();

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
            println!("{} Not supported now.", WARNING_MSG.yellow().bold());
            dms::generate_ansi_dps(&color)?
        }
    };

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
