use palette::{Hsv, RgbHue, Srgb};
use std::f32;
use crate::colors::convert::{ColorExt, FromHexToSrgbf32};
use anyhow::Result;

/// Hue Calculator
pub struct HueWheel {
    pub red: RgbHue,
    pub green: RgbHue,
    pub yellow: RgbHue,
    pub blue: RgbHue,
    pub magenta: RgbHue,
    pub cyan: RgbHue,
}
/// Hue Calculator
impl HueWheel {
    pub fn from_color(color: &Hsv, container: &Hsv) -> Self {
        let base = container.hue.into_positive_degrees();
        let shift = (base - 216.0) * 0.12;

        Self {
            red: RgbHue::from_degrees((0.0 + shift + 360.0).rem_euclid(360.0)),
            green: RgbHue::from_degrees((118.8 + shift + 360.0).rem_euclid(360.0)),
            yellow: RgbHue::from_degrees((54.0 + shift + 360.0).rem_euclid(360.0)),
            blue: RgbHue::from_degrees(base),
            magenta: color.hue,
            cyan: color.hue,
        }
    }
}

pub struct AnsiSV {
    pub red: (f32, f32),
    pub green: (f32, f32),
    pub yellow: (f32, f32),
    pub blue: (f32, f32),
    pub magenta: (f32, f32),
    pub cyan: (f32, f32),
}

impl AnsiSV {
    pub fn from_color(color: &Hsv, container: &Hsv) -> Self {
        const SAT_BOOST: f32 = 1.15;
        let bases = container.saturation;
        let basev = container.value;

        Self {
            red: (f32::min(0.65 * SAT_BOOST, 1.0), 0.8),
            green: (f32::min(0.42 * SAT_BOOST, 1.0), 0.84),
            yellow: (f32::min(0.38 * SAT_BOOST, 1.0), 0.86),
            blue: (f32::max(bases * 0.8, 0.6), f32::min(basev * 1.6, 1.0)),
            magenta: (color.saturation * 0.8, color.value * 0.75),
            cyan: (color.saturation, color.value),
        }
    }
}

pub struct AnsiNormalHSV {
    pub color: [Hsv; 7],
}

impl AnsiNormalHSV {
    pub fn get(color: &Hsv, container: &Hsv) -> Result<Self> {
        let sat_val = AnsiSV::from_color(color, container);
        let hues = HueWheel::from_color(color, container);

        let ansi = [
            Srgb::from_hex("#1a1a1a")?.to_hsv(),
            Hsv::new(hues.red, sat_val.red.0, sat_val.red.1),
            Hsv::new(hues.green, sat_val.green.0, sat_val.green.1),
            Hsv::new(hues.yellow, sat_val.yellow.0, sat_val.yellow.1),
            Hsv::new(hues.blue, sat_val.blue.0, sat_val.blue.1),
            Hsv::new(hues.magenta, sat_val.magenta.0, sat_val.magenta.1),
            Hsv::new(hues.cyan, sat_val.cyan.0, sat_val.cyan.1),
        ];

        Ok(Self {
            color: ansi,
        })
    }
}