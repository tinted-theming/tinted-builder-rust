use palette::{rgb::Rgb, FromColor, GetHue, Hsl, IntoColor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

use crate::error::TintedBuilderError;

/// A normalized color with multiple representations used by templates.
///
/// Stores hex (lowercased, without the leading `#`), 8-bit RGB, and normalized decimal channels
/// in `[0.0, 1.0]`. The custom `Serialize` implementation exposes template-friendly fields like
/// `hex`, `hex-r/g/b`, `hex-bgr`, `rgb`, `rgb16`, and `dec` as documented in the spec.
#[derive(Debug, Clone, Deserialize)]
pub struct Color {
    pub hex: (String, String, String),
    pub rgb: (u8, u8, u8),
    pub dec: (f32, f32, f32),
    pub name: ColorName,
    pub variant: ColorVariant,
}

impl Color {
    /// Creates a `Color` from a hex string like `"ff00ff"` or `"#ffcc00"` along with the `ColorName`
    /// and `ColorVariant`
    ///
    /// # Errors
    ///
    /// Returns `Err(TintedBuilderError::HexInputFormat)` if `hex_color` is not a valid
    /// 6-digit hexadecimal color (optionally prefixed with `#`).
    /// Creates a `Color` from a hex string like `"ff00ff"` or `"#ffcc00"`.
    ///
    /// The color is associated with an optional `ColorName` and `ColorVariant` for downstream usage.
    ///
    /// # Errors
    ///
    /// Returns `Err(TintedBuilderError::HexInputFormat)` if `hex_color` is not a valid
    /// 3- or 6-digit hexadecimal color (optionally prefixed with `#`).
    pub fn new(
        hex_color: &str,
        name: Option<ColorName>,
        variant: Option<ColorVariant>,
    ) -> Result<Self, TintedBuilderError> {
        let hex_full = process_hex_input(hex_color).ok_or(TintedBuilderError::HexInputFormat)?;
        let hex: (String, String, String) = (
            hex_full[0..2].to_lowercase(),
            hex_full[2..4].to_lowercase(),
            hex_full[4..6].to_lowercase(),
        );
        let rgb = hex_to_rgb(&hex)?;
        // Store normalized decimal channels in [0.0, 1.0]
        let inv_255: f32 = 1.0 / 255.0;
        let dec: (f32, f32, f32) = (
            f32::from(rgb.0) * inv_255,
            f32::from(rgb.1) * inv_255,
            f32::from(rgb.2) * inv_255,
        );

        Ok(Self {
            hex,
            rgb,
            dec,
            name: name.unwrap_or(ColorName::Other),
            variant: variant.unwrap_or(ColorVariant::Normal),
        })
    }

    #[must_use]
    /// Returns the 6-digit hex string (lowercase) without the leading `#`.
    pub fn to_hex(&self) -> String {
        format!("{}{}{}", &self.hex.0, &self.hex.1, &self.hex.2)
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::missing_errors_doc
    )]
    /// Derives a `dim` or `bright` variant from a `normal` color according to the Tinted8 rules.
    ///
    /// # Errors
    ///
    /// Returns an error when the color cannot be converted.
    pub fn try_to_variant(&self, color_variant: &ColorVariant) -> Result<Self, TintedBuilderError> {
        let rgb = Rgb::new(self.rgb.0, self.rgb.1, self.rgb.2);
        let hsl: Hsl = Hsl::from_color(rgb.into_format::<f32>());
        let updated_hsl = adjust_normal_hsl_for_variant(hsl, color_variant);
        let updated_rgb: Rgb = updated_hsl.into_color();
        let updated_rgb_r: u8 = (updated_rgb.red.clamp(0.0, 1.0) * 255.0).round() as u8;
        let updated_rgb_g: u8 = (updated_rgb.green.clamp(0.0, 1.0) * 255.0).round() as u8;
        let updated_rgb_b: u8 = (updated_rgb.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
        let updated_hex = format!("{updated_rgb_r:02X}{updated_rgb_g:02X}{updated_rgb_b:02X}",);

        Self::new(
            &updated_hex,
            Some(self.name.clone()),
            Some(color_variant.clone()),
        )
    }

    #[allow(
        clippy::missing_errors_doc,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    /// Derives supplemental colors (e.g., `orange` or `brown`) from a base color as specified.
    ///
    /// # Errors
    ///
    /// Returns an error when the requested conversion is unsupported.
    pub fn try_to_color(&self, color_name: &ColorName) -> Result<Self, TintedBuilderError> {
        let from_color_name = &self.name.clone();
        let to_color_name = color_name.clone();
        let to_color_variant = &self.variant.clone();

        match (&from_color_name, &to_color_name) {
            (ColorName::Yellow, ColorName::Orange) => {
                let from_rgb = Rgb::new(self.rgb.0, self.rgb.1, self.rgb.2);
                let from_hsl: Hsl = Hsl::from_color(from_rgb.into_format::<f32>());
                let from_hsl_h = from_hsl.get_hue().into_degrees();
                let from_hsl_s = from_hsl.saturation;
                let from_hsl_l = from_hsl.lightness;
                // Wrap-aware hue rotation toward orange (−12°), keep S/L unchanged
                let h_prime = (from_hsl_h - 10.0 + 360.0) % 360.0;
                let to_hsl: Hsl = Hsl::new(h_prime, from_hsl_s, from_hsl_l);
                let to_rgb: Rgb = to_hsl.into_color();
                let [to_rgb_r, to_rgb_g, to_rgb_b]: [u8; 3] =
                    [to_rgb.red, to_rgb.green, to_rgb.blue]
                        .map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8);
                let to_hex = format!("{to_rgb_r:02X}{to_rgb_g:02X}{to_rgb_b:02X}",);

                Self::new(
                    &to_hex,
                    Some(to_color_name.clone()),
                    Some(to_color_variant.clone()),
                )
            }
            (ColorName::Yellow, ColorName::Brown) => {
                let from_rgb = Rgb::new(self.rgb.0, self.rgb.1, self.rgb.2);
                let from_hsl: Hsl = Hsl::from_color(from_rgb.into_format::<f32>());
                let from_hsl_h = from_hsl.get_hue().into_degrees();
                let from_hsl_s = from_hsl.saturation;
                let from_hsl_l = from_hsl.lightness;
                let h_difference = 15.0;
                let l_difference = 0.3;
                let s_perc_difference = 0.65;
                // Clamp S/L after adjustment as per spec
                let s_prime = (from_hsl_s * s_perc_difference).clamp(0.0, 1.0);
                let l_prime = (from_hsl_l - l_difference).clamp(0.0, 1.0);
                let to_hsl: Hsl = Hsl::new(from_hsl_h - h_difference, s_prime, l_prime);
                let to_rgb: Rgb = to_hsl.into_color();
                let [to_rgb_r, to_rgb_g, to_rgb_b]: [u8; 3] =
                    [to_rgb.red, to_rgb.green, to_rgb.blue]
                        .map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8);
                let to_hex = format!("{to_rgb_r:02X}{to_rgb_g:02X}{to_rgb_b:02X}",);

                Self::new(
                    &to_hex,
                    Some(to_color_name.clone()),
                    Some(to_color_variant.clone()),
                )
            }
            _ => Err(TintedBuilderError::ColorConversion(
                from_color_name.to_string(),
                to_color_name.to_string(),
            )),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", &self.to_hex())
    }
}

/// Variants for a color token.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ColorVariant {
    Dim,
    Normal,
    Bright,
}

impl fmt::Display for ColorVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dim => write!(f, "dim"),
            Self::Normal => write!(f, "normal"),
            Self::Bright => write!(f, "bright"),
        }
    }
}

impl ColorVariant {
    #[must_use]
    pub const fn get_list<'a>() -> &'a [Self] {
        &[Self::Dim, Self::Normal, Self::Bright]
    }
}

/// Canonical color names used by the palette and theming properties.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ColorName {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Orange,
    Gray,
    Brown,
    Other, // For backward compatibility
}

/// Converts a `(rr, gg, bb)` hex tuple to `rgb` bytes.
fn hex_to_rgb(hex: &(String, String, String)) -> Result<(u8, u8, u8), TintedBuilderError> {
    let r = u8::from_str_radix(hex.0.as_str(), 16)?;
    let g = u8::from_str_radix(hex.1.as_str(), 16)?;
    let b = u8::from_str_radix(hex.2.as_str(), 16)?;

    Ok((r, g, b))
}

/// Normalizes a hex string by removing an optional `#` and expanding 3-digit to 6-digit.
fn process_hex_input(input: &str) -> Option<String> {
    // Check and process the hash prefix
    let hex_str = input.strip_prefix('#').unwrap_or(input);

    match hex_str.len() {
        // Convert 3-length hex to 6-length by duplicating each character
        3 => {
            if hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
                Some(
                    hex_str
                        .chars()
                        .flat_map(|c| std::iter::repeat(c).take(2))
                        .collect(),
                )
            } else {
                None // Contains invalid characters
            }
        }
        // Validate the 6-length hex value
        6 => {
            if hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
                Some(hex_str.to_string())
            } else {
                None // Contains invalid characters
            }
        }
        // Invalid length
        _ => None,
    }
}

const DL: f32 = 0.12;

/// Adjusts HSL channels to derive `dim`/`bright` from `normal` according to Tinted8 rules.
fn adjust_normal_hsl_for_variant(hsl: Hsl, color_variant: &ColorVariant) -> Hsl {
    let mut updated_s = hsl.saturation;
    let mut updated_l = hsl.lightness;

    match color_variant {
        ColorVariant::Dim => {
            let k: f32 = if hsl.lightness < 0.4 {
                1.04
            } else if hsl.lightness < 0.7 {
                1.07
            } else {
                1.1
            };
            let delta_l = DL.min(hsl.lightness);

            updated_l = (hsl.lightness - delta_l).clamp(0.0, 1.0);
            updated_s = (hsl.saturation * k).clamp(0.0, 1.0);
        }
        ColorVariant::Bright => {
            let k: f32 = if hsl.lightness < 0.5 {
                1.08
            } else if hsl.lightness < 0.8 {
                1.00
            } else {
                0.9
            };
            let delta_l = DL.min(1.0 - hsl.lightness);

            updated_l = (hsl.lightness + delta_l).clamp(0.0, 1.0);
            updated_s = (hsl.saturation * k).clamp(0.0, 1.0);
        }
        _ => {}
    }

    Hsl::new(hsl.hue, updated_s, updated_l)
}

#[derive(Serialize)]
struct RgbSer {
    r: u8,
    g: u8,
    b: u8,
}
#[derive(Serialize)]
struct Rgb16Ser {
    r: u16,
    g: u16,
    b: u16,
}
#[derive(Serialize)]
struct DecSer {
    r: String,
    g: String,
    b: String,
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("hex", &self.to_hex())?;
        map.serialize_entry("hex-r", &self.hex.0)?;
        map.serialize_entry("hex-g", &self.hex.1)?;
        map.serialize_entry("hex-b", &self.hex.2)?;
        let hex_bgr = format!("{}{}{}", self.hex.2, self.hex.1, self.hex.0);
        map.serialize_entry("hex-bgr", &hex_bgr)?;

        let rgb = RgbSer {
            r: self.rgb.0,
            g: self.rgb.1,
            b: self.rgb.2,
        };
        map.serialize_entry("rgb", &rgb)?;

        let rgb16 = Rgb16Ser {
            r: u16::from(self.rgb.0) * 257,
            g: u16::from(self.rgb.1) * 257,
            b: u16::from(self.rgb.2) * 257,
        };
        map.serialize_entry("rgb16", &rgb16)?;

        let dec = DecSer {
            r: format!("{:.8}", f64::from(self.dec.0)),
            g: format!("{:.8}", f64::from(self.dec.1)),
            b: format!("{:.8}", f64::from(self.dec.2)),
        };
        map.serialize_entry("dec", &dec)?;

        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_to_color_object() {
        let color = Color::new("#AABBCC", Some(ColorName::Blue), Some(ColorVariant::Normal))
            .expect("unable to create new color");
        let yaml = serde_yaml::to_string(&color).expect("unable to serialize color");
        assert!(yaml.contains("hex: aabbcc"));
        assert!(yaml.contains("hex-r: aa"));
        assert!(yaml.contains("hex-g: bb"));
        assert!(yaml.contains("hex-b: cc"));
        assert!(yaml.contains("hex-bgr: ccbbaa"));
        assert!(yaml.contains(
            "rgb:
  r: 170
  g: 187
  b: 204"
        ));
        assert!(yaml.contains(
            "rgb16:
  r: 43690
  g: 48059
  b: 52428"
        ));
        assert!(yaml.contains(
            "dec:
  r: '0.66666669'
  g: '0.73333335'
  b: '0.80000007'"
        ));
    }

    #[test]
    fn color_object_field_types() {
        let color = Color::new("#112233", Some(ColorName::Blue), Some(ColorVariant::Normal))
            .expect("unable to create color");
        let val = serde_yaml::to_value(&color).expect("unable to deserialize color");
        let map = val.as_mapping().expect("unable to create mapping");
        // hex is string
        assert!(map
            .get(serde_yaml::Value::String("hex".into()))
            .expect("unable to get 'hex' property")
            .as_str()
            .is_some());
        // rgb is mapping with numeric values
        let rgb = map
            .get(serde_yaml::Value::String("rgb".into()))
            .expect("unable to get 'rgb' property")
            .as_mapping()
            .expect("unable to create mapping");
        assert!(rgb
            .get(serde_yaml::Value::String("r".into()))
            .expect("unable to get 'rgb.r' property")
            .as_i64()
            .is_some());
        // rgb16 numeric
        let rgb16 = map
            .get(serde_yaml::Value::String("rgb16".into()))
            .expect("unable to get 'rgb16' property")
            .as_mapping()
            .expect("unable to create mapping");
        assert!(rgb16
            .get(serde_yaml::Value::String("r".into()))
            .expect("unable to get 'rgb16.r' property")
            .as_i64()
            .is_some());
        // dec strings
        let dec = map
            .get(serde_yaml::Value::String("dec".into()))
            .expect("unable to get 'dec' property")
            .as_mapping()
            .expect("unable to create mapping");
        assert!(dec
            .get(serde_yaml::Value::String("r".into()))
            .expect("unable to get 'dec.r' property")
            .as_str()
            .is_some());
    }
}
