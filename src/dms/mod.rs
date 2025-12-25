use crate::{
    colors::convert::{ColorExt, FromHexToSrgbf32},
    dms::calculator::{AnsiSV, HueWheel},
};
use anyhow::Result;
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
pub struct AnsiNorColor {
    black: Hsv,
    red: Hsv,
    green: Hsv,
    yellow: Hsv,
    blue: Hsv,
    magenta: Hsv,
    cyan: Hsv,
}

impl AnsiNorColor {
    pub fn new(hues: HueWheel, sat_val: AnsiSV) -> Result<Self> {
        Ok(Self {
            black: Srgb::from_hex("#1a1a1a")?.to_hsv(),
            red: Hsv::new(hues.red, sat_val.red.0, sat_val.red.1),
            green: Hsv::new(hues.green, sat_val.green.0, sat_val.green.1),
            yellow: Hsv::new(hues.yellow, sat_val.yellow.0, sat_val.yellow.1),
            blue: Hsv::new(hues.blue, sat_val.blue.0, sat_val.blue.1),
            magenta: Hsv::new(hues.magenta, sat_val.magenta.0, sat_val.magenta.1),
            cyan: Hsv::new(hues.cyan, sat_val.cyan.0, sat_val.cyan.1),
        })
    }
    pub fn iter(&self) -> impl Iterator<Item = &Hsv> {
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

    let ansi = AnsiNorColor::new(
        super::dms::HueWheel::from_color(&hsv, &based),
        super::dms::AnsiSV::from_color(&hsv, &based),
    )?;

    let mut new = Vec::with_capacity(8);
    new.push(ansi.black);
    for color in ansi.iter().skip(1) {
        new.push(balance_contrast::balance_contrast_dps_l_star(
            color,
            &ansi.black,
            NORMAL_TEXT_TARGET,
            false,
        ))
    }

    for (i, color) in new.iter().enumerate() {
        println!("{} = {}", i, color.to_hex())
    }

    Ok(())
}

#[cfg(test)]
#[allow(unused_variables, dead_code)]
pub fn generate_ansi_palette(primary: &str) -> Result<()> {
    let hsv = Srgb::from_hex(primary)?.to_hsv();
    let based = derive_container(&hsv);
    let primary_hue = hsv.hue.into_positive_degrees();
    let hue_shift = (primary_hue - 216.0) * 0.12;

    const SAT_BOOST: f32 = 1.15;
    const NORMAL_TEXT_TARGET: f32 = 40.0; // dark mode only
    const SECONDARY_TARGET: f32 = 35.0;

    let mut ansi = Vec::with_capacity(16);

    let bg = "#1a1a1a";
    ansi.push(bg);

    let red_hue = (0.0 + hue_shift + 360.0) % 360.0;
    let green_hue = (118.8 + hue_shift + 360.0) % 360.0;
    let yellow_hue = (54.0 + hue_shift + 360.0) % 360.0;
    let blue_hue = based.hue.into_positive_degrees();
    let magenta_hue = if based.hue.into_positive_degrees() - 10.8 < 0 as f32 {
        based.hue.into_positive_degrees() - 10.8 + 360.0
    } else {
        based.hue.into_positive_degrees() - 10.8
    };
    let cyan_hue = if based.hue.into_positive_degrees() + 28.8 > 360.0 {
        based.hue.into_positive_degrees() + 28.8 - 360.0
    } else {
        based.hue.into_positive_degrees() + 28.8
    };

    let hues = (
        red_hue,
        green_hue,
        yellow_hue,
        blue_hue,
        based.hue.into_positive_degrees(), // weird
        hsv.value,
    );
    // idk what i coding

    let saturation = (
        f32::min(0.65 * SAT_BOOST, 1.0),
        f32::min(0.42 * SAT_BOOST, 1.0),
        f32::min(0.38 * SAT_BOOST, 1.0),
        f32::max(based.saturation * 0.9, 0.7),
        based.saturation * 0.8,
        hsv.saturation,
    );

    let value = (
        0.80,
        0.84,
        0.86,
        f32::min(based.value * 1.6, 1.0),
        hsv.value * 0.75,
        hsv.value,
    );

    println!("{:?} {:?} {:?}", hues, saturation, value);

    Ok(())
}
