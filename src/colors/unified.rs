/// THE RESULT ONLY
/// Type: Hex
pub struct AnsiPaletteColor {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
}

impl AnsiPaletteColor {
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        [
            &self.black,
            &self.red,
            &self.green,
            &self.yellow,
            &self.blue,
            &self.magenta,
            &self.cyan,
        ]
        .into_iter()
    }
}
