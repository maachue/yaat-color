use color_eyre::eyre::Result;
use clap::ValueEnum;
use owo_colors::OwoColorize;
use palette::Srgb;

use crate::{
    colors::{
        convert::{ColorExt, FromHexToSrgbf32},
        unified::AnsiPaletteHex,
    },
    utils::DOING_WORK_MSG,
};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum DumpMode {
    #[value(name = "HumanReadable", alias = "human-readable", alias = "human")]
    HumanReadable,

    #[value(
        name = "Block",
        alias = "HumanReadableBlock",
        alias = "human-readable-block",
        alias = "block"
    )]
    Block,

    #[value(name = "JsonSimplifed", alias = "json-simplified", alias = "json")]
    JsonSimplified,

    #[value(name = "JsonPretty", alias = "json-pretty", alias = "matugen")]
    JsonPretty,
}

mod template;

pub fn json_dump_simplified(ansi: &AnsiPaletteHex) -> Result<()> {
    let temp_colors = ansi.read_as_indexmap();
    let json = serde_json::to_string_pretty(&temp_colors)?;

    eprintln!("{} json", DOING_WORK_MSG.style("   Dumping"));
    eprintln!("{} json dumping", DOING_WORK_MSG.style("    Finished"));

    println!("{}", json);

    Ok(())
}

pub fn json_dump_pretty(ansi: &AnsiPaletteHex) -> Result<()> {
    use template::FreshJson;

    let json = FreshJson {
        yaat: ansi.read_as_viewer_json_struct(),
    };

    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}

pub fn block(ansi: &AnsiPaletteHex) -> Result<()> {
    const FORMAT: &str = "    ";
    println!();
    for c in ansi.normal.iter() {
        print!("{}", FORMAT.on_color(Srgb::from_hex(c)?.to_owo()));
    }
    println!();
    for c in ansi.bright.iter() {
        print!("{}", FORMAT.on_color(Srgb::from_hex(c)?.to_owo()));
    }
    println!();

    Ok(())
}

pub fn human_readable(ansi: &AnsiPaletteHex) -> Result<()> {
    const FORMAT: &str = "    ";
    println!();
    for (i, c) in ansi.normal.iter().chain(ansi.bright.iter()).enumerate() {
        println!(
            "{:02} = {} {c}",
            i,
            FORMAT.on_color(Srgb::from_hex(c)?.to_owo())
        );
    }

    Ok(())
}
