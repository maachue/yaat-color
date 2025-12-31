use anyhow::Result;
use palette::Hsv;

use crate::{
    backends::{ResultBackEnd, dms::calculator::retone_to_l},
    colors::convert::{ColorExt, ToSrgb},
};

use super::Calculator;

mod calculator;

pub type CalculatorDms = Calculator<Hsv>;

fn derive_container(color: &Hsv) -> Hsv {
    Hsv::new(
        color.hue,
        f32::min(color.saturation * 1.834, 1.0),
        color.value * 0.463,
    )
}

impl CalculatorDms {
    fn get(color: &Hsv, container: &Hsv) -> Result<Self> {
        let bright_blue = retone_to_l(color, 85.0);

        let hues = calculator::get_hues(color, container, &bright_blue)?;
        let saturations = calculator::get_saturation(color, container, &bright_blue)?;
        let values = calculator::get_value(color, container, &bright_blue)?;

        Ok(Self {
            normal: std::array::from_fn(|i| {
                Hsv::new(hues.normal[i], saturations.normal[i], values.normal[i])
            }),
            bright: std::array::from_fn(|i| {
                Hsv::new(hues.bright[i], saturations.bright[i], values.bright[i])
            }),
        })
    }
}

impl ResultBackEnd for CalculatorDms {
    fn generate(
        color: &palette::Srgb<f32>,
        balance: &super::BalanceContrast,
    ) -> Result<super::BackEnds> {
        let color = color.to_hsv();
        let container = derive_container(&color);
        let ansi = Self::get(&color, &container)?;

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
