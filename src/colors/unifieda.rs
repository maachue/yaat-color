/*
 * THIS FILE IS UNITED THE ANSI PALETTE FROM SPECIAL BACKEND GENERATE.
*/

use palette::Hsv;

/// THE RESULT ONLY
/// Type: Hex
pub struct AnsiPalette {
    pub normal:BasedAnsi,
    pub bright:BasedAnsi,
}

pub struct BasedAnsi {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

impl BasedAnsi {
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        [
            &self.black,
            &self.red,
            &self.green,
            &self.yellow,
            &self.blue,
            &self.magenta,
            &self.cyan,
            &self.white,
        ]
        .into_iter()
    }

    pub fn from_array(array: &[&str; 8]) -> Self {
        Self {
            black: array[0].to_string(),
            red: array[1].to_string(),
            green: array[2].to_string(),
            yellow: array[3].to_string(),
            blue: array[4].to_string(),
            magenta: array[5].to_string(),
            cyan: array[6].to_string(),
            white: array[7].to_string(),
        }
    }
}

impl AnsiPalette {
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        [
            &self.normal.black,
            &self.normal.red,
            &self.normal.green,
            &self.normal.yellow,
            &self.normal.blue,
            &self.normal.magenta,
            &self.normal.cyan,
            &self.bright.black,
            &self.bright.red,
            &self.bright.green,
            &self.bright.yellow,
            &self.bright.blue,
            &self.bright.magenta,
            &self.bright.cyan,
        ]
        .into_iter()
    }

    pub fn from_array(ansi_palette: &[String; 16]) -> Self {
        let (normal, bright) = {
            let mut normal = Vec::with_capacity(8);
            let mut bright = Vec::with_capacity(8);
            for (i, color) in ansi_palette.iter().enumerate() {
                if i >= 7 {
                    bright.push(*color);
                } else {
                    normal.push(*color);
                }
            };
            (normal.try_into().expect("Error"), bright.try_into().expect("fdas"))
        };

        Self {
            normal: BasedAnsi::from_array(&normal),
            bright: BasedAnsi::from_array(&bright),
        }
    }
}

/// ANSI Palette result as type: HSV
pub struct AnsiPaletteHsv {
    pub normal: BasedAnsiHsv,
    pub bright: BasedAnsiHsv,
}

pub struct BasedAnsiHsv {
    pub black: Hsv,
    pub red: Hsv,
    pub green: Hsv,
    pub yellow: Hsv,
    pub blue: Hsv,
    pub magenta: Hsv,
    pub cyan: Hsv,
    pub white: Hsv,
}

impl AnsiPaletteHsv {
    pub fn iter(&self) -> impl Iterator<Item = &Hsv> {
        [
            &self.normal.black,
            &self.normal.red,
            &self.normal.green,
            &self.normal.yellow,
            &self.normal.blue,
            &self.normal.magenta,
            &self.normal.cyan,
            &self.normal.white,
            &self.bright.black,
            &self.bright.red,
            &self.bright.green,
            &self.bright.yellow,
            &self.bright.blue,
            &self.bright.magenta,
            &self.bright.cyan,
            &self.bright.white,
        ]
        .into_iter()
    }
}
// ANSI HSV later