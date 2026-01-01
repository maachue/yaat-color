use std::{fs::File, io::Write};

use color_eyre::eyre::{Result, bail};
use palette::Srgb;

use crate::{
    colors::{convert::FromHexToSrgbf32, unified::AnsiPaletteHex},
    utils::WARN_MSG,
};

pub fn _apply_ecapse_code(ansi: &AnsiPaletteHex) -> Result<()> {
    for (i, c) in ansi.normal.iter().chain(ansi.bright.iter()).enumerate() {
        let rgb: Srgb<u8> = Srgb::from_hex(c)?.into_format();
        println!(
            "\x1b]4;{i};rgb:{:02x}/{:02x}/{:02x}\x1b\\",
            rgb.red, rgb.green, rgb.blue
        );
    }
    Ok(())
}

fn format_as_sequences(color: &Srgb<u8>, index: u32) -> String {
    format!(
        "\x1b]4;{index};rgb:{:02x}/{:02x}/{:02x}\x1b\\",
        color.red, color.green, color.blue
    )
}

fn get_seq(ansi: &AnsiPaletteHex) -> Result<String> {
    let srgbs: [Srgb<u8>; 16] = {
        let mut temp: Vec<Srgb<u8>> = Vec::with_capacity(16);
        for color in ansi.normal.iter().chain(ansi.bright.iter()) {
            temp.push(Srgb::from_hex(color)?.into_format());
        }
        temp.try_into().expect("Err!6")
    };
    let str: String = {
        let mut temp: String = "".to_string();
        for (i, c) in srgbs.iter().enumerate() {
            temp.push_str(&format_as_sequences(c, i as u32));
        }
        temp
    };
    Ok(str)
}

/// TODO: support BSD, MacOS
pub fn unix_term(ansi: &AnsiPaletteHex) -> Result<()> {
    let sequences = get_seq(ansi)?;

    let tty_pattern = "/dev/pts/[0-9]*";
    let devices = glob::glob(tty_pattern).expect("glob pattern is ok");

    for entry in devices {
        match entry {
            Ok(path) => {
                if let Err(e) =
                    File::create(&path).and_then(|mut o| o.write_all(sequences.as_bytes()))
                {
                    eprintln!(
                        "{w}: couldn't write to {p}: {e}",
                        p = path.display(),
                        w = WARN_MSG,
                    );
                    continue;
                }
            }
            Err(e) => bail!("Error while sending sequences to terminals:\n{e}"),
        }
    }
    Ok(())
}
