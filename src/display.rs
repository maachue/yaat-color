use anyhow::Result;
use owo_colors::OwoColorize;

use crate::colors::unified::AnsiPaletteHex;

pub fn json_dump(ansi: &AnsiPaletteHex) -> Result<()> {
    let temp_colors = ansi.read_as_indexmap();
    let json = serde_json::to_string_pretty(&temp_colors)?;

    eprintln!("{} dumping json:", "Successfully".green());
    println!("{}", json);

    Ok(())
}
