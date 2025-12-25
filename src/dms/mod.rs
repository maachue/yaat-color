use crate::{
    colors::convert::{ColorExt, FromHexToSrgbf32, ToSrgb},
    dms::calculator::{AnsiNormalHSV},
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

/// THE RESULT ONLY
/// Type: HSV
pub struct AnsiNormalColor {
    black: Srgb<f32>,
    red: Srgb<f32>,
    green: Srgb<f32>,
    yellow: Srgb<f32>,
    blue: Srgb<f32>,
    magenta: Srgb<f32>,
    cyan: Srgb<f32>,
}

impl AnsiNormalColor {
    pub fn new(hsvs: &[Hsv; 7]) -> Result<Self> {
        Ok(Self {
            black: hsvs[0].to_srgb(),
            red: hsvs[1].to_srgb(),
            green: hsvs[2].to_srgb(),
            yellow: hsvs[3].to_srgb(),
            blue: hsvs[4].to_srgb(),
            magenta: hsvs[5].to_srgb(),
            cyan: hsvs[6].to_srgb(),
        })
    }
    pub fn iter(&self) -> impl Iterator<Item = &Srgb<f32>> {
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

pub fn derive_container(color: &Hsv) -> Hsv {
    Hsv::new(
        color.hue,
        f32::min(color.saturation * 1.834, 1.0),
        color.value * 0.463,
    )
}

pub fn generate_ansi(primary: &str) -> Result<()> {
    let hsv = Srgb::from_hex(primary)?.to_hsv();
    let based = derive_container(&hsv);

    const NORMAL_TEXT_TARGET: f32 = 40.0;

    let ansi = AnsiNormalHSV::get(
        &hsv,
        &based,
    )?;

    let mut new = Vec::with_capacity(7);
    new.push(ansi.color[0]);
    for color in ansi.color.iter().skip(1) {
        new.push(balance_contrast::balance_contrast_dps_l_star(color, &ansi.color[0], NORMAL_TEXT_TARGET, false));
    }

    let newa: [Hsv; 7] = new.try_into().expect("fdas");
    let ansi = AnsiNormalColor::new(&newa)?;
    for (i, ansi) in ansi.iter().enumerate() {
        println!("{} = \"{}\"", i, ansi.to_hex())
    }

    Ok(())
}