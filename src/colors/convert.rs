use anyhow::Result;
use palette::{Clamp, Hsl, Hsv, IntoColor, Lab, Lch, Oklch, Srgb};
use std::str::FromStr;

pub trait ToSrgb {
    fn to_srgb(&self) -> Srgb<f32>;
}

pub trait ColorExt: ToSrgb {
    #[allow(unused)]
    fn to_hex(&self) -> String {
        let rgb = self.to_srgb().clamp();
        format!(
            "#{:02x}{:02x}{:02x}",
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
        )
    }

    #[allow(unused)]
    fn to_lab(&self) -> Lab {
        self.to_srgb().into_color()
    }

    #[allow(unused)]
    fn to_hsv(&self) -> Hsv {
        self.to_srgb().into_color()
    }

    #[allow(unused)]
    fn to_lch(&self) -> Lch {
        self.to_srgb().into_color()
    }

    #[allow(unused)]
    fn to_oklch(&self) -> Oklch {
        self.to_srgb().into_color()
    }
}

impl<T: ToSrgb> ColorExt for T {}

impl ToSrgb for Srgb<f32> {
    fn to_srgb(&self) -> Srgb<f32> {
        *self
    }
}

impl ToSrgb for Hsv {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).into_color()
    }
}

impl ToSrgb for Lab {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).into_color()
    }
}

impl ToSrgb for Lch {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).into_color()
    }
}

impl ToSrgb for Oklch {
    fn to_srgb(&self) -> Srgb<f32> {
        (*self).into_color()
    }
}

pub trait FromHexToSrgbf32 {
    fn from_hex(hex: &str) -> Result<Srgb<f32>>;
}

impl FromHexToSrgbf32 for Srgb {
    fn from_hex(hex: &str) -> Result<Srgb<f32>> {
        Ok(Srgb::from_str(hex)?.into_format())
    }
}
