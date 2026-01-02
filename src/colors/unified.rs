/*
 * THIS FILE IS UNITED THE ANSI PALETTE FROM SPECIAL BACKEND GENERATE.
*/

use indexmap::IndexMap;
use serde::Serialize;

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
    #[cfg(feature = "seq_win")]
    #[allow(dead_code)] // clippy is too annoying
    pub fn get_ro(&self, index: usize) -> &T {
        &self.0[index]
    }
}

use std::{array, ops::Index};

impl<T: Clone> AnsiPalette<T> {
    pub fn from_array(colors: [T; 16]) -> Self {
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
    /// deprecated
    pub fn _read_as_viewer_json_struct(&self) -> ViewerAsIndexMapAnsiPalette<T> {
        let normal = self
            .normal
            .iter()
            .enumerate()
            .map(|(i, c)| (i.to_string(), c.clone()))
            .collect();
        let bright = self
            .bright
            .iter()
            .enumerate()
            .map(|(i, c)| (i.to_string(), c.clone()))
            .collect();

        ViewerAsIndexMapAnsiPalette { normal, bright }
    }
}

impl<T> Index<usize> for BasedAnsi<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// NOTE: refactor?
#[derive(Serialize)]
pub struct ViewerAsIndexMapAnsiPalette<T> {
    pub normal: IndexMap<String, T>,
    pub bright: IndexMap<String, T>,
}

pub type AnsiPaletteHex = AnsiPalette<String>;
