use std::time::Instant;

use clap::Parser;
use color_eyre::eyre::{Result, bail};
use owo_colors::OwoColorize;
use palette::Srgb;

use crate::{
    cli::Mode,
    colors::convert::{ColorExt, FromHexToSrgbf32},
    display::DumpMode,
    utils::{DOING_WORK_MSG, NOTE_MSG, WARN_MSG, read_stdin},
};

mod colors;

mod cli;
mod display;
mod utils;

mod term;

mod backends;

// TODO: refactor this
fn main() -> Result<()> {
    color_eyre::install()?;

    let mut cmd = cli::Cli::parse();

    if cmd.json_dump {
        cmd.dump = DumpMode::JsonSimplified;
    }

    if cmd.apply && std::env::consts::OS == "windows" {
        eprintln!(
            "{}: `--apply` flag hasn't support fully for windows yet.",
            WARN_MSG
        )
    }

    if cmd.color.is_none()
        && let Some(stdin) = read_stdin()
    {
        if !cmd.quiet {
            eprintln!("{NOTE_MSG}: found pipeline! Value is: {stdin}")
        }

        cmd.color = Some(stdin)
    }

    if let Some(v) = cmd.color.as_deref()
        && v.is_empty()
    {
        bail!(
            "Something went wrong. {} {}",
            "<COLOR>".green(),
            "is empty!".bright_red(),
        )
    }

    let start: Option<Instant> = match cmd.quiet {
        false => Some(Instant::now()),
        true => None,
    };

    let _light_huh = match cmd.mode {
        Mode::Dark => false,
        Mode::Light => true,
    };

    let color: Srgb<f32> = match cmd.color.as_deref() {
        Some(color) => {
            if cmd.from_srgb {
                let rgb: Vec<&str> = color.split(",").collect();

                let (r, g, b) = match rgb.as_slice() {
                    [first, second, third] => (
                        first.parse::<u8>()?,
                        second.parse::<u8>()?,
                        third.parse::<u8>()?,
                    ),
                    _ => bail!("Rgb value is incorrect!"),
                };
                Srgb::new(r, g, b).into_format()
            } else {
                Srgb::from_hex(color)?.into_format()
            }
        }
        None => bail!(
            "The following required arguments were not provided:\n  {}",
            "<COLOR>".green()
        ),
    };

    if !cmd.quiet {
        eprintln!(
            "{} ANSI palette with `{}` backend, {} mode, with input {} {}",
            DOING_WORK_MSG.style("   Calculating"),
            cmd.backend,
            cmd.mode,
            "  ".on_color(color.to_owo()),
            match cmd.color.as_deref() {
                Some(v) => v,
                None => bail!("How?"), // unreachable
            },
        );
    }

    let colors = backends::generate(&color, &cmd.backend, &cmd.balance)?;

    if let Some(start) = start {
        let elapsed = start.elapsed();
        eprintln!(
            "{} calculate in {}",
            DOING_WORK_MSG.style("    Finished"),
            utils::format_duration(elapsed),
        );
    }

    if !cmd.quiet {
        eprintln!("{} the palette", DOING_WORK_MSG.style("   Printing"));
    }

    match cmd.dump {
        DumpMode::HumanReadable => display::human_readable(&colors)?,
        DumpMode::Block => display::block(&colors)?,
        DumpMode::JsonSimplified => display::json_dump_simplified(&colors)?,
        DumpMode::JsonPretty => display::json_dump_pretty(&colors)?,
    }

    if !cmd.quiet {
        eprintln!(
            "\n{}{}",
            DOING_WORK_MSG.style("    Finished"),
            " ENJOY THE PALETTE".bold(),
        );
    }

    if cmd.apply {
        term::apply(&colors)?;
    }

    Ok(())
}
