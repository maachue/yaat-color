// /// return CIE Delta H*
// #[cfg(test)]
// #[allow(dead_code)]
// pub fn cie_delta_hue_star(color1: &Lch, color2: &Lch) -> f32 {
//     use std::f32::consts::PI;

//     let c1 = color1.chroma;
//     let c2 = color2.chroma;
//     let h1 = color1.hue.into_radians();
//     let h2 = color2.hue.into_radians();

//     let mut delta_h = h2 - h1;
//     if delta_h > PI {
//         delta_h -= 2.0 * PI;
//     } else if delta_h < -PI {
//         delta_h += 2.0 * PI;
//     }

//     2.0 * (c1 * c2).sqrt() * (delta_h / 2.0).sin().abs()
// }

/// return Delta Phi Star (𝜟𝜱*)
/// Formula: | lf^𝜱 - bf^𝜱 |^(1/𝜱) * √(2) - 40
pub fn delta_phi_star(l1: f32, l2: f32) -> f32 {
    const PHI: f32 = 1.618;
    const INV: f32 = 0.618;
    const SQRT2: f32 = 1.414;
    // 𝜱 = 5**0.5 * 0.5 + 0.5

    // delta phi star
    (l1.powf(PHI) - l2.powf(PHI)).abs().powf(INV) * SQRT2 - 40.0
}
