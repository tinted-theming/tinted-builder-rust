use crate::scheme::tinted8::yaml::BasicPalette;
use crate::Color;
use crate::ColorName;
use crate::ColorVariant;
use crate::TintedBuilderError;
use palette::rgb::Rgb;
use palette::GetHue;
use palette::IntoColor;
use palette::{FromColor, Hsl, RgbHue};
use serde::ser::SerializeMap;
use serde::Serialize;
use std::fmt;
use thiserror::Error;

impl fmt::Display for ColorName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, "black"),
            Self::Red => write!(f, "red"),
            Self::Green => write!(f, "green"),
            Self::Yellow => write!(f, "yellow"),
            Self::Blue => write!(f, "blue"),
            Self::Magenta => write!(f, "magenta"),
            Self::Cyan => write!(f, "cyan"),
            Self::White => write!(f, "white"),
            Self::Orange => write!(f, "orange"),
            Self::Gray => write!(f, "gray"),
            Self::Brown => write!(f, "brown"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl ColorName {
    #[must_use]
    pub const fn get_list<'a>() -> &'a [Self] {
        &[
            Self::Black,
            Self::Red,
            Self::Green,
            Self::Yellow,
            Self::Blue,
            Self::Magenta,
            Self::Cyan,
            Self::White,
            Self::Orange,
            Self::Gray,
            Self::Brown,
        ]
    }
}
#[derive(Debug, Clone)]
pub struct Palette {
    pub black_normal: Color,
    pub black_bright: Color,
    pub black_dim: Color,

    pub red_normal: Color,
    pub red_bright: Color,
    pub red_dim: Color,

    pub green_normal: Color,
    pub green_bright: Color,
    pub green_dim: Color,

    pub yellow_normal: Color,
    pub yellow_bright: Color,
    pub yellow_dim: Color,

    pub blue_normal: Color,
    pub blue_bright: Color,
    pub blue_dim: Color,

    pub magenta_normal: Color,
    pub magenta_bright: Color,
    pub magenta_dim: Color,

    pub cyan_normal: Color,
    pub cyan_bright: Color,
    pub cyan_dim: Color,

    pub white_normal: Color,
    pub white_bright: Color,
    pub white_dim: Color,

    pub orange_normal: Color,
    pub orange_bright: Color,
    pub orange_dim: Color,

    pub gray_normal: Color,
    pub gray_bright: Color,
    pub gray_dim: Color,

    pub brown_normal: Color,
    pub brown_bright: Color,
    pub brown_dim: Color,
}

impl Palette {
    /// Converts a basic YAML palette into a fully expanded palette with `normal`, `dim`, and `bright` variants.
    ///
    /// Derives missing variants and supplemental colors according to Tinted8 builder rules.
    #[allow(clippy::too_many_lines)]
    pub fn try_from_basic(basic_palette: &BasicPalette) -> Result<Self, PaletteError> {
        let variant_bright = &ColorVariant::Bright;
        let variant_dim = &ColorVariant::Dim;
        let generated_gray = color_black_and_white_to_gray(
            &Color::new(
                &basic_palette.black,
                Some(ColorName::Black),
                Some(ColorVariant::Normal),
            )
            .map_err(|_| PaletteError::UnableToConvertFrom("Color".to_string()))?,
            &Color::new(
                &basic_palette.white,
                Some(ColorName::White),
                Some(ColorVariant::Normal),
            )
            .map_err(|_| PaletteError::UnableToConvertFrom("Color".to_string()))?,
        )
        .map_err(|_| PaletteError::UnableToConvertFrom("Generated Gray".to_string()))?;
        let generated_gray_hex = generated_gray.to_hex();
        let black_normal = Color::new(
            &basic_palette.black,
            Some(ColorName::Black),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let red_normal = Color::new(
            &basic_palette.red,
            Some(ColorName::Red),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let green_normal = Color::new(
            &basic_palette.green,
            Some(ColorName::Green),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let yellow_normal = Color::new(
            &basic_palette.yellow,
            Some(ColorName::Yellow),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let blue_normal = Color::new(
            &basic_palette.blue,
            Some(ColorName::Blue),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let magenta_normal = Color::new(
            &basic_palette.magenta,
            Some(ColorName::Magenta),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let cyan_normal = Color::new(
            &basic_palette.cyan,
            Some(ColorName::Cyan),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let white_normal = Color::new(
            &basic_palette.white,
            Some(ColorName::White),
            Some(ColorVariant::Normal),
        )
        .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let orange_normal = basic_palette
            .orange
            .as_ref()
            .map_or_else(
                || yellow_normal.clone().try_to_color(&ColorName::Orange),
                |orange_hex| {
                    Color::new(
                        orange_hex,
                        Some(ColorName::Orange),
                        Some(ColorVariant::Normal),
                    )
                },
            )
            .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;
        let brown_normal = basic_palette
            .brown
            .as_ref()
            .map_or_else(
                || yellow_normal.clone().try_to_color(&ColorName::Brown),
                |brown_hex| {
                    Color::new(
                        brown_hex,
                        Some(ColorName::Brown),
                        Some(ColorVariant::Normal),
                    )
                },
            )
            .map_err(|err| PaletteError::UnableToCreateColor(err.to_string()))?;

        let palette = Self {
            black_normal: Color::new(
                &basic_palette.black,
                Some(ColorName::Black),
                Some(ColorVariant::Normal),
            )
            .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            black_dim: basic_palette
                .black_dim
                .as_ref()
                .map_or_else(
                    || black_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::Black), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            black_bright: basic_palette
                .black_bright
                .as_ref()
                .map_or_else(
                    || black_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::Black), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,

            red_dim: basic_palette
                .red_dim
                .as_ref()
                .map_or_else(
                    || red_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::Red), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            red_bright: basic_palette
                .red_bright
                .as_ref()
                .map_or_else(
                    || red_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::Red), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            red_normal,

            green_dim: basic_palette
                .green_dim
                .as_ref()
                .map_or_else(
                    || green_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::Green), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            green_bright: basic_palette
                .green_bright
                .as_ref()
                .map_or_else(
                    || green_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::Green), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,

            green_normal,

            yellow_dim: basic_palette
                .yellow_dim
                .as_ref()
                .map_or_else(
                    || yellow_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::Yellow), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            yellow_bright: basic_palette
                .yellow_bright
                .as_ref()
                .map_or_else(
                    || yellow_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::Yellow), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            yellow_normal,

            blue_dim: basic_palette
                .blue_dim
                .as_ref()
                .map_or_else(
                    || blue_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::Blue), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            blue_bright: basic_palette
                .blue_bright
                .as_ref()
                .map_or_else(
                    || blue_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::Blue), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            blue_normal,

            magenta_dim: basic_palette
                .magenta_dim
                .as_ref()
                .map_or_else(
                    || magenta_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::Magenta), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            magenta_bright: basic_palette
                .magenta_bright
                .as_ref()
                .map_or_else(
                    || magenta_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::Magenta), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            magenta_normal,

            cyan_dim: basic_palette
                .cyan_dim
                .as_ref()
                .map_or_else(
                    || cyan_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::Cyan), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            cyan_bright: basic_palette
                .cyan_bright
                .as_ref()
                .map_or_else(
                    || cyan_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::Cyan), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            cyan_normal,

            white_dim: basic_palette
                .white_dim
                .as_ref()
                .map_or_else(
                    || white_normal.clone().try_to_variant(&ColorVariant::Dim),
                    |hex| Color::new(hex, Some(ColorName::White), Some(ColorVariant::Dim)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            white_bright: basic_palette
                .white_bright
                .as_ref()
                .map_or_else(
                    || white_normal.clone().try_to_variant(&ColorVariant::Bright),
                    |hex| Color::new(hex, Some(ColorName::White), Some(ColorVariant::Bright)),
                )
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            white_normal,

            orange_normal: orange_normal.clone(),
            orange_bright: orange_normal
                .clone()
                .try_to_variant(&ColorVariant::Bright)
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            orange_dim: orange_normal
                .try_to_variant(&ColorVariant::Dim)
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,

            gray_normal: Color::new(
                basic_palette.gray.as_ref().unwrap_or(&generated_gray_hex),
                Some(ColorName::Gray),
                Some(ColorVariant::Normal),
            )
            .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            gray_bright: Color::new(
                basic_palette.gray.as_ref().unwrap_or(&generated_gray_hex),
                Some(ColorName::Gray),
                Some(ColorVariant::Bright),
            )
            .and_then(|c| c.try_to_variant(variant_bright))
            .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            gray_dim: Color::new(
                basic_palette.gray.as_ref().unwrap_or(&generated_gray_hex),
                Some(ColorName::Gray),
                Some(ColorVariant::Dim),
            )
            .and_then(|c| c.try_to_variant(variant_dim))
            .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,

            brown_normal: brown_normal.clone(),
            brown_bright: brown_normal
                .clone()
                .try_to_variant(&ColorVariant::Bright)
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
            brown_dim: brown_normal
                .try_to_variant(&ColorVariant::Dim)
                .map_err(|err| PaletteError::UnableToConvertFrom(err.to_string()))?,
        };

        Ok(palette)
    }

    /// Returns all `(ColorName, ColorVariant)` tuples in canonical order.
    pub fn get_color_list() -> Vec<(ColorName, ColorVariant)> {
        let mut keys = vec![];

        for color_name in ColorName::get_list() {
            for color_variant in ColorVariant::get_list() {
                keys.push((color_name.clone(), color_variant.clone()));
            }
        }

        keys
    }

    /// Returns a reference to a color by its name and variant, if present.
    pub const fn get_color(
        &self,
        color_name: &ColorName,
        color_variant: &ColorVariant,
    ) -> Option<&Color> {
        match (color_name, color_variant) {
            (ColorName::Black, ColorVariant::Dim) => Some(&self.black_dim),
            (ColorName::Black, ColorVariant::Normal) => Some(&self.black_normal),
            (ColorName::Black, ColorVariant::Bright) => Some(&self.black_bright),

            (ColorName::Red, ColorVariant::Dim) => Some(&self.red_dim),
            (ColorName::Red, ColorVariant::Normal) => Some(&self.red_normal),
            (ColorName::Red, ColorVariant::Bright) => Some(&self.red_bright),

            (ColorName::Green, ColorVariant::Dim) => Some(&self.green_dim),
            (ColorName::Green, ColorVariant::Normal) => Some(&self.green_normal),
            (ColorName::Green, ColorVariant::Bright) => Some(&self.green_bright),

            (ColorName::Yellow, ColorVariant::Dim) => Some(&self.yellow_dim),
            (ColorName::Yellow, ColorVariant::Normal) => Some(&self.yellow_normal),
            (ColorName::Yellow, ColorVariant::Bright) => Some(&self.yellow_bright),

            (ColorName::Blue, ColorVariant::Dim) => Some(&self.blue_dim),
            (ColorName::Blue, ColorVariant::Normal) => Some(&self.blue_normal),
            (ColorName::Blue, ColorVariant::Bright) => Some(&self.blue_bright),

            (ColorName::Magenta, ColorVariant::Dim) => Some(&self.magenta_dim),
            (ColorName::Magenta, ColorVariant::Normal) => Some(&self.magenta_normal),
            (ColorName::Magenta, ColorVariant::Bright) => Some(&self.magenta_bright),

            (ColorName::Cyan, ColorVariant::Dim) => Some(&self.cyan_dim),
            (ColorName::Cyan, ColorVariant::Normal) => Some(&self.cyan_normal),
            (ColorName::Cyan, ColorVariant::Bright) => Some(&self.cyan_bright),

            (ColorName::White, ColorVariant::Dim) => Some(&self.white_dim),
            (ColorName::White, ColorVariant::Normal) => Some(&self.white_normal),
            (ColorName::White, ColorVariant::Bright) => Some(&self.white_bright),

            (ColorName::Orange, ColorVariant::Dim) => Some(&self.orange_dim),
            (ColorName::Orange, ColorVariant::Normal) => Some(&self.orange_normal),
            (ColorName::Orange, ColorVariant::Bright) => Some(&self.orange_bright),

            (ColorName::Gray, ColorVariant::Dim) => Some(&self.gray_dim),
            (ColorName::Gray, ColorVariant::Normal) => Some(&self.gray_normal),
            (ColorName::Gray, ColorVariant::Bright) => Some(&self.gray_bright),

            (ColorName::Brown, ColorVariant::Dim) => Some(&self.brown_dim),
            (ColorName::Brown, ColorVariant::Normal) => Some(&self.brown_normal),
            (ColorName::Brown, ColorVariant::Bright) => Some(&self.brown_bright),
            _ => None,
        }
    }

    pub fn to_basic_palette(&self) -> BasicPalette {
        BasicPalette {
            black: self.black_normal.to_hex(),
            black_dim: Some(self.black_dim.to_hex()),
            black_bright: Some(self.black_bright.to_hex()),

            red: self.red_normal.to_hex(),
            red_dim: Some(self.red_dim.to_hex()),
            red_bright: Some(self.red_bright.to_hex()),

            green: self.green_normal.to_hex(),
            green_dim: Some(self.green_dim.to_hex()),
            green_bright: Some(self.green_bright.to_hex()),

            yellow: self.yellow_normal.to_hex(),
            yellow_dim: Some(self.yellow_dim.to_hex()),
            yellow_bright: Some(self.yellow_bright.to_hex()),

            blue: self.blue_normal.to_hex(),
            blue_dim: Some(self.blue_dim.to_hex()),
            blue_bright: Some(self.blue_bright.to_hex()),

            magenta: self.magenta_normal.to_hex(),
            magenta_dim: Some(self.magenta_dim.to_hex()),
            magenta_bright: Some(self.magenta_bright.to_hex()),

            cyan: self.cyan_normal.to_hex(),
            cyan_dim: Some(self.cyan_dim.to_hex()),
            cyan_bright: Some(self.cyan_bright.to_hex()),

            white: self.white_normal.to_hex(),
            white_dim: Some(self.white_dim.to_hex()),
            white_bright: Some(self.white_bright.to_hex()),

            orange: Some(self.orange_normal.to_hex()),
            orange_dim: Some(self.orange_dim.to_hex()),
            orange_bright: Some(self.orange_bright.to_hex()),

            gray: Some(self.gray_normal.to_hex()),
            gray_dim: Some(self.gray_dim.to_hex()),
            gray_bright: Some(self.gray_bright.to_hex()),

            brown: Some(self.brown_normal.to_hex()),
            brown_dim: Some(self.brown_dim.to_hex()),
            brown_bright: Some(self.brown_bright.to_hex()),
        }
    }
}

#[derive(Serialize)]
struct Variants<'a> {
    normal: &'a Color,
    bright: &'a Color,
    dim: &'a Color,
}

impl Serialize for Palette {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(11))?;

        map.serialize_entry(
            "black",
            &Variants {
                normal: &self.black_normal,
                bright: &self.black_bright,
                dim: &self.black_dim,
            },
        )?;
        map.serialize_entry(
            "red",
            &Variants {
                normal: &self.red_normal,
                bright: &self.red_bright,
                dim: &self.red_dim,
            },
        )?;
        map.serialize_entry(
            "green",
            &Variants {
                normal: &self.green_normal,
                bright: &self.green_bright,
                dim: &self.green_dim,
            },
        )?;
        map.serialize_entry(
            "yellow",
            &Variants {
                normal: &self.yellow_normal,
                bright: &self.yellow_bright,
                dim: &self.yellow_dim,
            },
        )?;
        map.serialize_entry(
            "blue",
            &Variants {
                normal: &self.blue_normal,
                bright: &self.blue_bright,
                dim: &self.blue_dim,
            },
        )?;
        map.serialize_entry(
            "magenta",
            &Variants {
                normal: &self.magenta_normal,
                bright: &self.magenta_bright,
                dim: &self.magenta_dim,
            },
        )?;
        map.serialize_entry(
            "cyan",
            &Variants {
                normal: &self.cyan_normal,
                bright: &self.cyan_bright,
                dim: &self.cyan_dim,
            },
        )?;
        map.serialize_entry(
            "white",
            &Variants {
                normal: &self.white_normal,
                bright: &self.white_bright,
                dim: &self.white_dim,
            },
        )?;
        map.serialize_entry(
            "orange",
            &Variants {
                normal: &self.orange_normal,
                bright: &self.orange_bright,
                dim: &self.orange_dim,
            },
        )?;
        map.serialize_entry(
            "gray",
            &Variants {
                normal: &self.gray_normal,
                bright: &self.gray_bright,
                dim: &self.gray_dim,
            },
        )?;
        map.serialize_entry(
            "brown",
            &Variants {
                normal: &self.brown_normal,
                bright: &self.brown_bright,
                dim: &self.brown_dim,
            },
        )?;

        map.end()
    }
}

impl fmt::Display for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_list = Self::get_color_list();
        for (name, variant) in color_list {
            let unknown = "unknown".to_string();
            let hex = self
                .clone()
                .get_color(&name, &variant)
                .map_or(unknown, Color::to_hex);

            writeln!(f, "  {name}-{variant}: \"#{hex}\"")?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PaletteError {
    #[error("missing palette key {0}")]
    MissingKey(&'static str),
    #[error("unable to convert from type: {0}")]
    UnableToConvertFrom(String),
    #[error("unable to create color: {0}")]
    UnableToCreateColor(String),
    #[error("invalid palette yaml")]
    InvalidYaml,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
/// Derives a neutral gray by averaging black and white in HSL and forcing `S=0`.
fn color_black_and_white_to_gray(
    black_color: &Color,
    white_color: &Color,
) -> Result<Color, TintedBuilderError> {
    let black_rgb = Rgb::new(black_color.rgb.0, black_color.rgb.1, black_color.rgb.2);
    let black_hsl: Hsl = Hsl::from_color(black_rgb.into_format::<f32>());
    let white_rgb = Rgb::new(white_color.rgb.0, white_color.rgb.1, white_color.rgb.2);
    let white_hsl: Hsl = Hsl::from_color(white_rgb.into_format::<f32>());

    // Compute midpoint hue (wrap-aware); hue is irrelevant for gray but keep it stable.
    let h1 = white_hsl.get_hue().into_degrees();
    let h2 = black_hsl.get_hue().into_degrees();
    let d = ((h2 - h1 + 540.0) % 360.0) - 180.0;
    let gray_hsl_h = (0.5_f32.mul_add(d, h1) + 360.0) % 360.0;
    // For a neutral gray, force saturation to 0 and average the lightness.
    let gray_hsl_s = 0.0;
    let gray_hsl_l = 0.5 * (white_hsl.lightness + black_hsl.lightness);

    let gray_hsl = Hsl::new(RgbHue::from_degrees(gray_hsl_h), gray_hsl_s, gray_hsl_l);
    let gray_rgb: Rgb = gray_hsl.into_color();
    let gray_rgb_r: u8 = (gray_rgb.red.clamp(0.0, 1.0) * 255.0).round() as u8;
    let gray_rgb_g: u8 = (gray_rgb.green.clamp(0.0, 1.0) * 255.0).round() as u8;
    let gray_rgb_b: u8 = (gray_rgb.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
    let gray_hex = format!("{gray_rgb_r:02X}{gray_rgb_g:02X}{gray_rgb_b:02X}",);

    Color::new(&gray_hex, Some(ColorName::Gray), Some(ColorVariant::Normal))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheme::tinted8::yaml::BasicPalette;

    #[test]
    fn serializes_nested_color_variants() {
        let basic = BasicPalette {
            black: "#000000".into(),
            red: "#ff0000".into(),
            green: "#00ff00".into(),
            yellow: "#ffff00".into(),
            blue: "#0000ff".into(),
            magenta: "#ff00ff".into(),
            cyan: "#00ffff".into(),
            white: "#ffffff".into(),
            orange: None,
            brown: None,
            black_bright: None,
            black_dim: None,
            red_bright: None,
            red_dim: None,
            green_bright: None,
            green_dim: None,
            yellow_bright: None,
            yellow_dim: None,
            blue_bright: None,
            blue_dim: None,
            magenta_bright: None,
            magenta_dim: None,
            cyan_bright: None,
            cyan_dim: None,
            white_bright: None,
            white_dim: None,
            orange_bright: None,
            orange_dim: None,
            gray: None,
            gray_bright: None,
            gray_dim: None,
            brown_bright: None,
            brown_dim: None,
        };

        let palette = Palette::try_from_basic(&basic).expect("palette");
        let yaml = serde_yaml::to_string(&palette).expect("yaml");

        assert!(yaml.contains("black:\n  normal:"));
        assert!(yaml.contains("red:\n  normal:"));
        assert!(yaml.contains("blue:\n  normal:"));
        assert!(yaml.contains("hex:"));
        assert!(yaml.contains("rgb:"));
        assert!(yaml.contains("dec:"));
        assert!(yaml.contains("rgb16:"));
    }
}
