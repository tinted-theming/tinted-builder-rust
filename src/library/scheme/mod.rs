// tinted-builder-rust is a Tinted Theming template builder which uses color
// schemes to generate theme files.
// Copyright (C) 2024  Tinted Theming

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod color;

use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

use crate::library::constants::{REQUIRED_BASE16_PALETTE_KEYS, REQUIRED_BASE24_PALETTE_KEYS};
use crate::library::scheme::color::Color;

#[derive(Deserialize)]
pub struct SchemeWrapper {
    pub system: String,
    pub name: String,
    pub slug: Option<String>,
    pub author: String,
    pub description: Option<String>,
    pub variant: Option<String>,
    pub palette: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Scheme {
    pub system: String,
    pub name: String,
    pub slug: String,
    pub author: String,
    pub description: Option<String>,
    pub variant: String,
    pub palette: HashMap<String, Color>,
}

fn slugify(input: &str) -> String {
    input
        .nfd() // Normalize the string to NFD form
        .filter(char::is_ascii) // Only keep ASCII characters
        .collect::<String>()
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-') // Only keep alphanumeric and hyphens
        .collect()
}

impl<'de> Deserialize<'de> for Scheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = SchemeWrapper::deserialize(deserializer)?;
        let slug = wrapper.slug.map_or(slugify(&wrapper.name), |slug| slugify(&slug));
        let variant = wrapper.variant.unwrap_or(String::from("dark"));

        match wrapper.system.as_str() {
            "base16" => {
                let contains_all_keys = REQUIRED_BASE16_PALETTE_KEYS
                    .iter()
                    .all(|&key| wrapper.palette.contains_key(key));

                if !contains_all_keys {
                    return Err(serde::de::Error::custom(format!(
                        "{} scheme does not contain the required palette properties",
                        wrapper.system
                    )));
                }
            }
            "base24" => {
                let contains_all_keys = REQUIRED_BASE24_PALETTE_KEYS
                    .iter()
                    .all(|&key| wrapper.palette.contains_key(key));

                if !contains_all_keys {
                    return Err(serde::de::Error::custom(format!(
                        "{} scheme does not contain the required palette properties",
                        wrapper.system
                    )));
                }
            }
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "Unknown system: {}",
                    wrapper.system
                )))
            }
        }

        let palette_result: Result<HashMap<String, Color>, _> = wrapper
            .palette
            .into_iter()
            .map(|(key, value)| {
                Color::new(value)
                    .map(|color| (key, color))
                    .map_err(|e| serde::de::Error::custom(e.to_string()))
            })
            .collect();

        Ok(Scheme {
            name: wrapper.name,
            slug,
            system: wrapper.system,
            author: wrapper.author,
            description: wrapper.description,
            variant,
            palette: palette_result?,
        })
    }
}
