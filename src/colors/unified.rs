/*
 * THIS FILE IS UNITED THE ANSI PALETTE FROM SPECIAL BACKEND GENERATE.
*/

use palette::Hsv;

#[repr(u8)]
enum AnsiIndex {
    Black, Red, Green, Yellow,
    Blue, Magenta, Cyan, White,
}

pub struct BasedAnsi<T>([T; 8]);

pub struct AnsiPalette<T> {
    pub normal: BasedAnsi<T>,
    pub bright: BasedAnsi<T>,
}

impl<T> BasedAnsi<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

use std::array;

impl<T: Clone> AnsiPalette<T> {
    pub fn from_array(colors: &[T; 16]) -> Self {
        let normal = BasedAnsi(array::from_fn(|i| colors[i].clone()));
        let bright = BasedAnsi(array::from_fn(|i| colors[i + 8].clone()));

        Self { normal, bright }
    }
}

pub type AnsiPaletteHex = AnsiPalette<String>;
pub type AnsiPaletteHsv = AnsiPalette<Hsv>;
