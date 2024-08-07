use serde::{Deserialize, Serialize};
use std::fmt;

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Color {
    pub hex: (String, String, String),
    pub rgb: (u8, u8, u8),
    pub dec: (f32, f32, f32),
}

impl Color {
    pub fn new(hex_color: String) -> Result<Color> {
        let hex_full = process_hex_input(&hex_color)
            .ok_or_else(|| anyhow::anyhow!("Provided hex value is not formatted correctly"))?;
        let hex: (String, String, String) = (
            hex_full[0..2].to_lowercase(),
            hex_full[2..4].to_lowercase(),
            hex_full[4..6].to_lowercase(),
        );
        let rgb = hex_to_rgb(&hex)
            .map_err(|_| anyhow!("Unable to convert hex value to rgb: {}", hex_full))?;
        let dec: (f32, f32, f32) = (
            rgb.0 as f32 / 255.0,
            rgb.1 as f32 / 255.0,
            rgb.2 as f32 / 255.0,
        );

        Ok(Color { hex, rgb, dec })
    }

    pub fn to_hex(&self) -> String {
        format!("{}{}{}", &self.hex.0, &self.hex.1, &self.hex.2)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.to_hex())
    }
}

fn hex_to_rgb(hex: &(String, String, String)) -> Result<(u8, u8, u8)> {
    let r = u8::from_str_radix(hex.0.as_str(), 16)
        .context("Invalid hex character for red component")?;
    let g = u8::from_str_radix(hex.1.as_str(), 16)
        .context("Invalid hex character for green component")?;
    let b = u8::from_str_radix(hex.2.as_str(), 16)
        .context("Invalid hex character for blue component")?;

    Ok((r, g, b))
}

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
