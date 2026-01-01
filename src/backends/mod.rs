use std::ops::Index;

use color_eyre::eyre::Result;
use clap::ValueEnum;
use palette::Srgb;

use crate::colors::{
    convert::ColorExt,
    unified::{AnsiPalette, AnsiPaletteHex},
};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum BackendEnum {
    Dms,
}

impl std::fmt::Display for BackendEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendEnum::Dms => write!(f, "DMS"),
        }
    }
}

#[derive(PartialEq, ValueEnum, Clone, Copy, Debug)]
pub enum BalanceContrast {
    Dps,
    // Wcag,
    None,
}

pub type AnsiPaletteSrgbf32 = AnsiPalette<Srgb<f32>>;

impl AnsiPaletteSrgbf32 {
    pub fn to_hex(&self) -> AnsiPaletteHex {
        AnsiPaletteHex::from_array(std::array::from_fn(|i| {
            if i < 8 {
                self.normal.index(i).to_hex()
            } else {
                self.bright.index(i - 8).to_hex()
            }
        }))
    }
}

pub struct Calculator<T> {
    pub normal: [T; 8],
    pub bright: [T; 8],
}

pub struct BackEnds {
    pub colors: AnsiPaletteSrgbf32,

    pub score_normal: Option<f32>,
    pub score_bright: Option<f32>,
}

pub trait ResultBackEnd {
    fn generate(&self, color: &Srgb<f32>, balance: &BalanceContrast) -> Result<BackEnds>;
}

mod dms;

pub fn generate(
    color: &Srgb<f32>,
    backend: &BackendEnum,
    balance_strag: &Option<BalanceContrast>,
) -> Result<AnsiPaletteHex> {
    let balance_strag = match balance_strag {
        Some(v) => v,
        None => &match backend {
            BackendEnum::Dms => BalanceContrast::Dps,
        },
    };

    let ansi_srgb = match backend {
        BackendEnum::Dms => dms::DMS.generate(color, balance_strag)?,
    };

    let ansi_srgb = match backend {
        BackendEnum::Dms => match balance_strag {
            BalanceContrast::Dps => dms::balance::dps::balance_dps(
                ansi_srgb.colors,
                (
                    ansi_srgb.score_normal.unwrap(),
                    ansi_srgb.score_bright.unwrap(),
                ),
                false,
            ),
            BalanceContrast::None => ansi_srgb.colors,
        },
    };

    Ok(ansi_srgb.to_hex())
}
