use serde::Serialize;

use crate::colors::unified::{AnsiPaletteHex, ViewerAsIndexMapAnsiPalette};

// using trait because this is a extended
pub trait Viewer {
    fn viewer_json_pretty(&self) -> ViewerAsIndexMapAnsiPalette<MatugenColorFormatHex<'_>>;
}

impl Viewer for AnsiPaletteHex {
    fn viewer_json_pretty(&self) -> ViewerAsIndexMapAnsiPalette<MatugenColorFormatHex<'_>> {
        let normal = self
            .normal
            .iter()
            .enumerate()
            .map(|(i, c)| (i.to_string(), MatugenColorFormatHex::from_color(c)))
            .collect();

        let bright = self
            .bright
            .iter()
            .enumerate()
            .map(|(i, c)| (i.to_string(), MatugenColorFormatHex::from_color(c)))
            .collect();

        ViewerAsIndexMapAnsiPalette { normal, bright }
    }
}

#[derive(Serialize)]
pub struct FreshJson<'a> {
    pub yaat: ViewerAsIndexMapAnsiPalette<MatugenColorFormatHex<'a>>,
}

/// We will borrow it to less memory and clone as possiable
#[derive(Serialize)]
pub struct MatugenColorFormatHex<'a> {
    pub hex: &'a str,
    pub hex_stripped: &'a str,
}

impl<'a> MatugenColorFormatHex<'a> {
    pub fn from_color(ansi: &'a str) -> Self {
        Self {
            hex: ansi,
            hex_stripped: ansi.trim_start_matches("#"),
        }
    }
}
