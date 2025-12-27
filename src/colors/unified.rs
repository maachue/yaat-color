/*
 * THIS FILE IS UNITED THE ANSI PALETTE FROM SPECIAL BACKEND GENERATE.
*/

use indexmap::IndexMap;
use palette::Hsv;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AnsiIndex {
    Black = 0,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
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
    pub fn read_as_indexmap(&self) -> IndexMap<u8, &T> {
        self.normal
            .iter()
            .chain(self.bright.iter())
            .enumerate()
            .map(|(i, c)| (i as u8, c))
            .collect()
    }
}

pub type AnsiPaletteHex = AnsiPalette<String>;
#[allow(dead_code)]
pub type AnsiPaletteHsv = AnsiPalette<Hsv>;
