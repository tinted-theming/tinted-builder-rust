mod color;

use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;
use unidecode::unidecode;

use crate::constants::{REQUIRED_BASE16_PALETTE_KEYS, REQUIRED_BASE24_PALETTE_KEYS};
use crate::scheme::color::Color;

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

pub fn slugify(input: &str) -> String {
    unidecode(input) // Convert to ASCII approximations
        .nfd() // Normalize the string to NFD form
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace() || *c == '-') // Keep ASCII alphanumerics, whitespace, and hyphens
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
        let slug = wrapper
            .slug
            .map_or(slugify(&wrapper.name), |slug| slugify(&slug));
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
