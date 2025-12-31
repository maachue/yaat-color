use std::ops::Index;

use crate::{
    backends::AnsiPaletteSrgbf32,
    colors::{convert::ToSrgb, unified::AnsiIndex},
};

use palette::{Clamp, Lab, Srgb};

use crate::colors::{convert::ColorExt, delta::delta_phi_star};

fn dps_ba(color1: &Lab, color2: &Lab, is_light: bool) -> f32 {
    if !is_light {
        delta_phi_star(color2.l, color1.l) + 5.0
    } else {
        delta_phi_star(color2.l, color1.l)
    }
}

/// Using 𝜟𝜱*
/// Algorithm: brute-force
fn balance_contrast_dps_l_star(color: &Lab, bg: &Lab, min_l: f32, is_light: bool) -> Lab {
    let score = dps_ba(color, bg, is_light);

    if score >= min_l {
        return *color;
    }

    let (mut lf, af, bf) = color.into_components();
    let dir = if is_light { -1.0 } else { 1.0 };

    const STEP: f32 = 0.5;

    for _ in 0..120 {
        lf = (lf + dir * STEP).clamp(0.0, 100.0);
        let cand = Lab::new(lf, af, bf);
        if dps_ba(&cand, &bg.to_lab(), is_light) >= min_l {
            return cand.clamp();
        }
    }

    *color
}

pub fn balance_dps(
    colors: AnsiPaletteSrgbf32,
    target: (f32, f32),
    is_light: bool,
) -> AnsiPaletteSrgbf32 {
    let bg = colors.normal.index(AnsiIndex::Black as usize).to_lab();

    let normal: [Srgb<f32>; 8] = std::array::from_fn(|i| {
        match i {
            0 /* AnsiIndex::Black */ => colors.normal[i],
            _ => balance_contrast_dps_l_star(&colors.normal[i].to_lab(), &bg, target.0, is_light).to_srgb(),
        }
    });

    let bright: [Srgb<f32>; 8] = std::array::from_fn(|i| {
        match i {
            0 | 3 | 4 | 7 /* AnsiIndex::Black | AnsiIndex::Yellow | AnsiIndex::Blue | AnsiIndex::White */ => colors.bright[i].to_srgb(),
            _ => balance_contrast_dps_l_star(&colors.bright[i].to_lab(), &bg, target.1, is_light).to_srgb(),
        }
    });

    AnsiPaletteSrgbf32::from_array(std::array::from_fn(|i| {
        if i < 8 { normal[i] } else { bright[i - 8] }
    }))
}
