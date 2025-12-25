use palette::Hsv;
use std::f32;

/// Hue Calculator
pub struct HueWheel {
    pub red: f32,
    pub green: f32,
    pub yellow: f32,
    pub blue: f32,
    pub magenta: f32,
    pub cyan: f32,
}
/// Hue Calculator
impl HueWheel {
    pub fn from_color(color: &Hsv, container: &Hsv) -> Self {
        let base = container.hue.into_positive_degrees();
        let shift = (base - 216.0) * 0.12;

        Self {
            red: (0.0 + shift + 360.0) % 360.0,
            green: (118.8 + shift + 360.0) % 360.0,
            yellow: (54.0 + shift + 360.0) % 360.0,
            blue: base,
            magenta: color.hue.into_positive_degrees(),
            cyan: color.hue.into_positive_degrees(),
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
