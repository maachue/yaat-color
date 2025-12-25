use palette::{Clamp, Hsv, Lab};

use crate::colors::{convert::ColorExt, delta::delta_phi_star};

fn dps_ba(color1: &Lab, color2: &Lab, is_light: bool) -> f32 {
    if !is_light {
        delta_phi_star(color1.l, color2.to_lab().l) + 5.0
    } else {
        delta_phi_star(color1.l, color2.l)
    }
}

/// Using 𝜟𝜱*
pub fn balance_contrast_dps_l_star(color: &Hsv, bg: &Hsv, min_l: f32, is_light: bool) -> Hsv {
    let score = dps_ba(&color.to_lab(), &bg.to_lab(), is_light);

    if score >= min_l {
        return *color;
    }

    let (mut lf, af, bf) = color.to_lab().into_components();
    let dir = if is_light { -1.0 } else { 1.0 };

    const STEP: f32 = 1.5;

    for _ in 0..120 {
        lf = f32::max(0.0, f32::min(100.0, lf + dir * STEP));
        let mut cand = Lab::new(lf, af, bf);
        cand = cand.clamp();
        if dps_ba(&cand, &bg.to_lab(), is_light) >= min_l {
            return cand.to_hsv();
        }
    }

    *color
}
