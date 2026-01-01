/*
 * # DMS backend
 *
 * This module is based on the **generator**
 * originally implemented by **DankMaterialShell**.
 *
 * Original algorithm
 * © 2025 Avenge Media LLC — MIT License
 *
 * This repository provides a **clean-room reimplementation**
 * written in Rust, with **significant structural and behavioral differences**
 * from the original implementation.
 *
 * All Rust source code in this repository:
 * © 2025 Maachue — MIT License
*/

/*
 * NOTE:
 * The author is **not a color scientist**.
 *
 * This software is provided **as-is** and should be evaluated carefully
 * before use.
*/

use color_eyre::eyre::Result;
use palette::Hsv;

use crate::{
    backends::{ResultBackEnd, dms::calculator::retone_to_l},
    colors::convert::{ColorExt, ToSrgb},
};

use super::Calculator;

mod calculator;

fn derive_container(color: &Hsv) -> Hsv {
    Hsv::new(
        color.hue,
        f32::min(color.saturation * 1.834, 1.0),
        color.value * 0.463,
    )
}

fn get(color: &Hsv, container: &Hsv) -> Result<Calculator<Hsv>> {
    let bright_blue = retone_to_l(color, 85.0);

    let hues     = calculator::get_hues(color, container, &bright_blue)?;
    let saturations = calculator::get_saturation(color, container, &bright_blue)?;
    let values      = calculator::get_value(color, container, &bright_blue)?;

    Ok(Calculator::<Hsv> {
        normal: std::array::from_fn(|i| {
            Hsv::new(hues.normal[i], saturations.normal[i], values.normal[i])
        }),
        bright: std::array::from_fn(|i| {
            Hsv::new(hues.bright[i], saturations.bright[i], values.bright[i])
        }),
    })
    }

pub struct Dms;

impl ResultBackEnd for Dms {
    fn generate(
        &self,
        color: &palette::Srgb<f32>,
        balance: &super::BalanceContrast,
    ) -> Result<super::BackEnds> {
        let color = color.to_hsv();
        let container = derive_container(&color);
        let ansi = get(&color, &container)?;

        let (normal_target, bright_target) = match balance {
            super::BalanceContrast::Dps => (Some(40.0), Some(35.0)),
            super::BalanceContrast::None => (None, None),
        };

        Ok(super::BackEnds {
            colors: super::AnsiPaletteSrgbf32::from_array(std::array::from_fn(|i| {
                if i < 8 {
                    ansi.normal[i].to_srgb()
                } else {
                    ansi.bright[i - 8].to_srgb()
                }
            })),
            score_normal: normal_target,
            score_bright: bright_target,
        })
    }
}

pub mod balance;
pub const DMS: Dms = Dms;
