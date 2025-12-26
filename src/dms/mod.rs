use crate::{
    colors::{convert::{ColorExt, FromHexToSrgbf32}, unified::AnsiPaletteHex},
    dms::calculator::AnsiResult,
};
use anyhow::{Result};
use palette::{Hsv, Srgb};

mod balance_contrast;
mod calculator;

#[allow(dead_code)]
pub enum BalanceContrastBackEnd {
    Dps,
    Wcag,
}

pub fn derive_container(color: &Hsv) -> Hsv {
    Hsv::new(
        color.hue,
        f32::min(color.saturation * 1.834, 1.0),
        color.value * 0.463,
    )
}

pub fn generate_ansi(primary: &str) -> Result<AnsiPaletteHex> {
    let hsv = Srgb::from_hex(primary)?.to_hsv();
    let based = derive_container(&hsv);

    const NORMAL_TEXT_TARGET: f32 = 40.0;
    const SECONDARY_TARGET: f32 = 35.0;

    let ansi = AnsiResult::get(
        &hsv,
        &based,
    )?;

    let ansi = ansi.balance_dps_itself(NORMAL_TEXT_TARGET, SECONDARY_TARGET);

    Ok(ansi.into_hex())
}