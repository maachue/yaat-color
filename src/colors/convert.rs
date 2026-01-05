use color_eyre::eyre::{Context, Result};
use palette::{Clamp, Hsv, IntoColor, Lab, Lch, Oklch, Srgb};
use std::str::FromStr;

// NOTE: is it really need to refactor this using `thiserror`?
// NOTE: is `*self` safe?
pub trait ToSrgb {
    fn to_srgb(&self) -> Srgb<f32>;
}

pub trait ColorExt: ToSrgb {
    fn to_hex(&self) -> String {
        let rgb = self.to_srgb().clamp();
        format!(
            "#{:02x}{:02x}{:02x}",
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
        )
    }

    fn to_lab(&self) -> Lab {
        self.to_srgb().clamp().into_color()
    }

    fn to_hsv(&self) -> Hsv {
        self.to_srgb().clamp().into_color()
    }

    fn to_owo(&self) -> owo_colors::Rgb {
        let rgb: Srgb<u8> = self.to_srgb().into_format();
        owo_colors::Rgb(rgb.red, rgb.green, rgb.blue)
    }

    // fn to_oklch(&self) -> Oklch {
    //     self.to_srgb().clamp().into_color()
    // }
}

impl<T: ToSrgb> ColorExt for T {}

impl ToSrgb for Srgb<f32> {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).clamp()
    }
}

impl ToSrgb for Hsv {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).clamp().into_color()
    }
}

impl ToSrgb for Lab {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).clamp().into_color()
    }
}

impl ToSrgb for Lch {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).clamp().into_color()
    }
}

impl ToSrgb for Oklch {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).clamp().into_color()
    }
}

pub trait FromHexToSrgbf32 {
    fn from_hex(hex: &str) -> Result<Srgb<f32>>;
}

impl FromHexToSrgbf32 for Srgb {
    fn from_hex(hex: &str) -> Result<Srgb<f32>> {
        Ok(Srgb::from_str(hex)
            .with_context(|| {
                format!(
                    "input '{}' invalid! Please use a value between 0 and F and format below.",
                    hex
                )
            })?
            .into_format())
    }
}
