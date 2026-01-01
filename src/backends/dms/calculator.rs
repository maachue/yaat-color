use color_eyre::eyre::Result;
use palette::{GetHue, Hsv, Lab, RgbHue, Srgb};

use crate::{
    backends::Calculator,
    colors::{
        convert::{ColorExt, FromHexToSrgbf32},
        unified::AnsiIndex,
    },
};

// =============== UTILS ===============

pub fn retone_to_l(color: &Hsv, l_target: f32) -> Hsv {
    let (l, a, b) = color.to_lab().into_components();

    let scale = if l != 0.0 { l_target / l } else { 1.0 };

    let (mut a2, mut b2) = (a * scale, b * scale);

    const MAX_CHROMA: f32 = 0.4 * 128.0; // hmmm ...

    let chroma = a2.hypot(b2);
    if chroma > MAX_CHROMA {
        let k = MAX_CHROMA / chroma;
        a2 *= k;
        b2 *= k;
    }

    Lab::new(l_target, a2, b2).to_hsv()
}

// =============== HUES ===============

pub fn get_hues(color: &Hsv, container: &Hsv, bright_blue: &Hsv) -> Result<Calculator<RgbHue>> {
    let base = container.hue.into_positive_degrees();

    let shift = (base - 216.0) * 0.12;

    let mut normal = [color.hue; 8];

    normal[AnsiIndex::Black as usize]  =  Srgb::from_hex("#1a1a1a")?.get_hue();
    normal[AnsiIndex::Red as usize]    =  RgbHue::from_degrees((0.0 + shift).rem_euclid(360.0));
    normal[AnsiIndex::Green as usize]  =  RgbHue::from_degrees((118.8 + shift).rem_euclid(360.0));
    normal[AnsiIndex::Yellow as usize] =  RgbHue::from_degrees((54.0 + shift).rem_euclid(360.0));
    normal[AnsiIndex::Blue as usize]   =  RgbHue::from_degrees(base);
    normal[AnsiIndex::White as usize]  =  Srgb::from_hex("#abb2bf")?.get_hue();

    let magh = {
        let default = base - 10.8;
        if default < 0.0 {
            default + 360.0
        } else {
            default
        }
    };

    let bcyanh = {
        let default = base + 7.2;
        if default > 360.0 {
            default - 360.0
        } else {
            default
        }
    };

    let mut bright = normal;

    bright[AnsiIndex::Black as usize]   =  Srgb::from_hex("#5c6370")?.get_hue();
    bright[AnsiIndex::Blue as usize]    =  bright_blue.hue;
    bright[AnsiIndex::Magenta as usize] =  RgbHue::from_degrees(magh);
    bright[AnsiIndex::Cyan as usize]    =  RgbHue::from_degrees(bcyanh);
    bright[AnsiIndex::White as usize]   =  Srgb::from_hex("#ffffff")?.get_hue();

    Ok(Calculator::<RgbHue> { normal, bright })
}

// =============== SATURATIONS ===============

pub fn get_saturation(color: &Hsv, container: &Hsv, bright_blue: &Hsv) -> Result<Calculator<f32>> {
    let bases = container.saturation;
    const SAT_BOOST: f32 = 1.15;

    let mut normal = [color.saturation; 8];

    normal[AnsiIndex::Black as usize]   =  Srgb::from_hex("#1a1a1a")?.to_hsv().saturation;
    normal[AnsiIndex::Red as usize]     =  f32::min(0.65 * SAT_BOOST, 1.0);
    normal[AnsiIndex::Green as usize]   =  f32::min(0.42 * SAT_BOOST, 1.0);
    normal[AnsiIndex::Yellow as usize]  =  f32::min(0.38 * SAT_BOOST, 1.0);
    normal[AnsiIndex::Blue as usize]    =  f32::max(bases * 0.8, 0.6);
    normal[AnsiIndex::Magenta as usize] =  color.saturation * 0.8;
    normal[AnsiIndex::Cyan as usize]    =  color.saturation;
    normal[AnsiIndex::White as usize]   =  Srgb::from_hex("#abb2bf")?.to_hsv().saturation;

    let mut bright = normal;
    bright[AnsiIndex::Black as usize]   =  Srgb::from_hex("#5c6370")?.to_hsv().saturation;
    bright[AnsiIndex::Red as usize]     =  f32::min(0.5 * SAT_BOOST, 1.0);
    bright[AnsiIndex::Green as usize]   =  f32::min(0.35 * SAT_BOOST, 1.0);
    bright[AnsiIndex::Yellow as usize]  =  f32::min(0.3 * SAT_BOOST, 1.0);
    bright[AnsiIndex::Blue as usize]    =  bright_blue.saturation;
    bright[AnsiIndex::Magenta as usize] =  f32::max(bases * 0.7, 0.6);
    bright[AnsiIndex::Cyan as usize]    =  f32::max(bases * 0.6, 0.5);
    bright[AnsiIndex::White as usize]   =  Srgb::from_hex("#ffffff")?.to_hsv().saturation;

    Ok(Calculator::<f32> { normal, bright })
}

// =============== VALUES ===============

pub fn get_value(color: &Hsv, container: &Hsv, bright_blue: &Hsv) -> Result<Calculator<f32>> {
    let basev = container.value;

    let mut normal = [color.value; 8];

    normal[AnsiIndex::Black as usize]   =  Srgb::from_hex("#1a1a1a")?.to_hsv().value;
    normal[AnsiIndex::Red as usize]     =  0.8;
    normal[AnsiIndex::Green as usize]   =  0.84;
    normal[AnsiIndex::Yellow as usize]  =  0.86;
    normal[AnsiIndex::Blue as usize]    =  f32::min(basev * 1.6, 1.0);
    normal[AnsiIndex::Magenta as usize] =  color.value * 0.75;
    normal[AnsiIndex::Cyan as usize]    =  color.value;
    normal[AnsiIndex::White as usize]   =  Srgb::from_hex("#abb2bf")?.to_hsv().value;

    let mut bright = normal;

    bright[AnsiIndex::Black as usize]   =  Srgb::from_hex("#5c6370")?.to_hsv().value;
    bright[AnsiIndex::Red as usize]     =  0.88;
    bright[AnsiIndex::Green as usize]   =  0.88;
    bright[AnsiIndex::Yellow as usize]  =  0.91;
    bright[AnsiIndex::Blue as usize]    =  bright_blue.value;
    bright[AnsiIndex::Magenta as usize] =  f32::min(basev * 1.3, 0.9);
    bright[AnsiIndex::Cyan as usize]    =  f32::min(basev * 1.2, 0.85);
    bright[AnsiIndex::White as usize]   =  Srgb::from_hex("#ffffff")?.to_hsv().value;

    Ok(Calculator::<f32> { normal, bright })
}
