mod color;

use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

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
    let char_map: HashMap<char, &str> = [
        ('á', "a"),
        ('à', "a"),
        ('â', "a"),
        ('ä', "a"),
        ('ã', "a"),
        ('å', "a"),
        ('æ', "ae"),
        ('ç', "c"),
        ('é', "e"),
        ('è', "e"),
        ('ê', "e"),
        ('ë', "e"),
        ('í', "i"),
        ('ì', "i"),
        ('î', "i"),
        ('ï', "i"),
        ('ł', "l"),
        ('ñ', "n"),
        ('ń', "n"),
        ('ó', "o"),
        ('ò', "o"),
        ('ô', "o"),
        ('ö', "o"),
        ('õ', "o"),
        ('ø', "o"),
        ('œ', "oe"),
        ('ś', "s"),
        ('ú', "u"),
        ('ù', "u"),
        ('û', "u"),
        ('ü', "u"),
        ('ý', "y"),
        ('ÿ', "y"),
        ('ż', "z"),
        ('ź', "z"),
        ('š', "s"),
        ('č', "c"),
        ('ř', "r"),
        ('đ', "d"),
        ('ß', "ss"),
        ('þ', "th"),
        ('ħ', "h"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut slug = String::new();
    for c in input.to_lowercase().chars() {
        match c {
            'a'..='z' | '0'..='9' => slug.push(c),
            ' ' | '-' | '_' => slug.push('-'),
            _ => {
                if let Some(replacement) = char_map.get(&c) {
                    slug.push_str(replacement);
                }
            }
        }
    }

    let re = Regex::new(r"-+").unwrap();
    let cleaned_slug = re.replace_all(&slug, "-").to_string();

    cleaned_slug.trim_matches('-').to_string()
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
