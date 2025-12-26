use anyhow::Result;
use palette::{GetHue, Hsv, Lab, RgbHue, Srgb};

use crate::{colors::convert::{ColorExt, FromHexToSrgbf32}, dms::balance_contrast::balance_contrast_dps_l_star};

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

pub struct HueWheel {
    pub normal: [RgbHue; 8],
    pub bright: [RgbHue; 8],
}

impl HueWheel {
    pub fn from_color(color: &Hsv, container: &Hsv) -> Result<Self> {
        let base = container.hue.into_positive_degrees();
        let shift = (base - 216.0) * 0.12;

        let mut normal = [color.hue; 8];

        normal[AnsiIndex::Black as usize]   = Srgb::from_hex("#1a1a1a")?.get_hue();
        normal[AnsiIndex::Red as usize]     = RgbHue::from_degrees((0.0 + shift).rem_euclid(360.0));
        normal[AnsiIndex::Green as usize]   = RgbHue::from_degrees((118.8 + shift).rem_euclid(360.0));
        normal[AnsiIndex::Yellow as usize]  = RgbHue::from_degrees((54.0 + shift).rem_euclid(360.0));
        normal[AnsiIndex::Blue as usize]    = RgbHue::from_degrees(base);
        normal[AnsiIndex::White as usize]   = Srgb::from_hex("#abb2bf")?.get_hue();

        let magh = {
            let default = base - 10.8;
            if default < 0.0 {
                default + 360.0
            } else {
                default
            }
        };
        // let cyanh = {
        //     let default = color.hue.into_positive_degrees() + 0.08;
        //     if default > 1.0 {
        //         default - 1.0
        //     } else {
        //         default
        //     }
        // };
        let bcyanh = {
            let default = base + 7.2;
            if default > 360.0 {
                default - 360.0
            } else {
                default
            }
        };
        let mut bright = normal;
        bright[AnsiIndex::Black as usize]      = Srgb::from_hex("#5c6370")?.get_hue();
        bright[AnsiIndex::Blue as usize]       = retone_to_l(color, 85.0).hue; // wrong?
        bright[AnsiIndex::Magenta as usize]    = RgbHue::from_degrees(magh);
        bright[AnsiIndex::Cyan as usize]       = RgbHue::from_degrees(bcyanh);
        bright[AnsiIndex::White as usize]      = Srgb::from_hex("#ffffff")?.get_hue();

        Ok(Self { normal, bright })
    }
}

// ====================================================================================================================

// NOTE: Call this function too much and no archived as variable made the program
// so slow
fn retone_to_l(color: &Hsv, l_target: f32) -> Hsv {
    let (l, a, b) = color.to_lab().into_components();

    let scale = if l != 0.0 {
        l_target / l
    } else {
        1.0
    };

    let (mut a2, mut b2) = (a * scale, b * scale);

    const MAX_CHROMA: f32 = 0.4 * 128.0;
    let chroma = a2.hypot(b2);
    if chroma > MAX_CHROMA {
        let k = MAX_CHROMA / chroma;
        a2 *= k;
        b2 *= k;
    }

    Lab::new(l_target, a2, b2).to_hsv()
}

// ====================================================================================================================

pub struct AnsiSV {
    pub normal: [(f32, f32); 8],
    pub bright: [(f32, f32); 8],
}

impl AnsiSV {
    pub fn from_color(color: &Hsv, container: &Hsv) -> Result<Self> {
        let bases = container.saturation;
        let basev = container.value;
        const SAT_BOOST: f32 = 1.15;

        let mut normal = [(color.saturation, color.value); 8];

        normal[AnsiIndex::Black as usize]   = (Srgb::from_hex("#1a1a1a")?.to_hsv().saturation, Srgb::from_hex("#1a1a1a")?.to_hsv().value);
        normal[AnsiIndex::Red as usize]     = (f32::min(0.65 * SAT_BOOST, 1.0), 0.8);
        normal[AnsiIndex::Green as usize]   = (f32::min(0.42 * SAT_BOOST, 1.0), 0.84);
        normal[AnsiIndex::Yellow as usize]  = (f32::min(0.38 * SAT_BOOST, 1.0), 0.86);
        normal[AnsiIndex::Blue as usize]    = (f32::max(bases * 0.8, 0.6), f32::min(basev * 1.6, 1.0));
        normal[AnsiIndex::Magenta as usize] = (color.saturation * 0.8, color.value * 0.75);
        normal[AnsiIndex::Cyan as usize]    = (color.to_hsv().saturation, color.to_hsv().value);
        normal[AnsiIndex::White as usize]   = (Srgb::from_hex("#abb2bf")?.to_hsv().saturation, Srgb::from_hex("#abb2bf")?.to_hsv().value);

        let mut bright = normal;
        bright[AnsiIndex::Black as usize]      = (Srgb::from_hex("#5c6370")?.to_hsv().saturation, Srgb::from_hex("#5c6370")?.to_hsv().value);
        bright[AnsiIndex::Red as usize]        = (f32::min(0.5 * SAT_BOOST, 1.0), 0.88);
        bright[AnsiIndex::Green as usize]      = (f32::min(0.35 * SAT_BOOST, 1.0), 0.88);
        bright[AnsiIndex::Yellow as usize]     = (f32::min(0.3 * SAT_BOOST, 1.0), 0.91);
        bright[AnsiIndex::Blue as usize]       = (retone_to_l(color, 85.0).saturation, retone_to_l(color, 85.0).value);
        bright[AnsiIndex::Magenta as usize]    = (f32::max(bases * 0.7, 0.6), f32::min(basev * 1.3, 0.9));
        bright[AnsiIndex::Cyan as usize]       = (f32::max(bases * 0.6, 0.5), f32::min(basev * 1.2, 0.85));
        bright[AnsiIndex::White as usize]      = (Srgb::from_hex("#ffffff")?.to_hsv().saturation, Srgb::from_hex("#ffffff")?.to_hsv().value);

        Ok(Self { normal, bright })
    }
}

pub struct AnsiResult {
    pub normal: [Hsv; 8],
    pub bright: [Hsv; 8],
}

impl AnsiResult {
    pub fn get(color: &Hsv, container: &Hsv) -> Result<Self> {
        let sat_val = AnsiSV::from_color(color, container)?;
        let hues = HueWheel::from_color(color, container)?;

        let normal: [Hsv; 8] = {
            let mut temp: Vec<Hsv> = Vec::with_capacity(8);
            for (hue, sv) in hues.normal.iter().zip(sat_val.normal.iter()) {
                temp.push(Hsv::new(*hue, (*sv).0, (*sv).1));
            };
            temp.try_into().expect("Err!")
        };
        let bright: [Hsv; 8] = {
            let mut temp: Vec<Hsv> = Vec::with_capacity(8);
            for (hue, sv) in hues.bright.iter().zip(sat_val.bright.iter()) {
                temp.push(Hsv::new(*hue, (*sv).0, (*sv).1));
            };
            temp.try_into().expect("Err!")
        };

        Ok(
            Self { normal: normal, bright: bright }
        )
    }
    pub fn balance_dps_itself(&self, target_normal: f32, target_bright: f32) -> Self {
        let bg = self.normal[AnsiIndex::Black as usize];
        let new_normal: [Hsv; 8] = {
            let mut temp: Vec<Hsv> = Vec::with_capacity(8);
            temp.push(self.normal[0]);
            for color in self.normal.iter().skip(1) {
                temp.push(balance_contrast_dps_l_star(color, &bg, target_normal, false));
            }
            temp.try_into().expect("Err!a1")
        };
        let new_bright: [Hsv; 8] = {
            let mut temp: Vec<Hsv> = Vec::with_capacity(8);
            temp.push(self.bright[0]);
            for color in self.bright.iter().skip(1).take(8 - 1 - 5) {
                temp.push(balance_contrast_dps_l_star(color, &bg, target_bright, false));
            }
            temp.push(self.bright[11 - 8]);
            temp.push(self.bright[12 - 8]);
            temp.push(balance_contrast_dps_l_star(&self.bright[13 - 8], &bg, target_bright, false));
            temp.push(balance_contrast_dps_l_star(&self.bright[14 - 8], &bg, target_bright, false));
            temp.push(self.bright[15 - 8]);
            temp.try_into().expect("Err!a2")
        };

        Self {
            normal: new_normal,
            bright: new_bright,
        }
    }
    pub fn into_hex(&self) -> crate::colors::unified::AnsiPaletteHex {
        let normal: [String; 8] = {
            let mut temp = Vec::with_capacity(8);
            for color in self.normal.iter() {
                temp.push(color.to_hex());
            }
            temp.try_into().expect("Err!r")
        };
        let bright: [String; 8] = {
            let mut temp = Vec::with_capacity(8);
            for color in self.bright.iter() {
                temp.push((*color).to_hex());
            }
            temp.try_into().expect("Err!r")
        };

        // let chained: [String; 16] = std::array::from_fn(|i| {
        //     if i < 8 {
        //         normal[i].clone()
        //     } else {
        //         bright[i - 8].clone()
        //     }
        // });

        crate::colors::unified::AnsiPaletteHex::from_array(
            &std::array::from_fn(|i| {
            if i < 8 {
                normal[i].clone()
            } else {
                bright[i - 8].clone()
            }
        }))
    }
}