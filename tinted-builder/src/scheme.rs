mod color;

use regex::Regex;
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt};

use crate::constants::{REQUIRED_BASE16_PALETTE_KEYS, REQUIRED_BASE24_PALETTE_KEYS};

pub use crate::scheme::color::Color;

#[derive(Deserialize, Serialize)]
pub struct SchemeWrapper {
    pub(crate) system: SchemeSystem,
    pub(crate) name: String,
    pub(crate) slug: Option<String>,
    pub(crate) author: String,
    pub(crate) description: Option<String>,
    pub(crate) variant: Option<SchemeVariant>,
    pub(crate) palette: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Scheme {
    pub system: SchemeSystem,
    pub name: String,
    pub slug: String,
    pub author: String,
    pub description: Option<String>,
    pub variant: SchemeVariant,
    pub palette: HashMap<String, Color>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemeSystem {
    #[default]
    Base16,
    Base24,
}

impl SchemeSystem {
    pub fn as_str(&self) -> &str {
        match self {
            SchemeSystem::Base16 => "base16",
            SchemeSystem::Base24 => "base24",
        }
    }
}

impl fmt::Display for SchemeSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "name: \"{}\"", self.as_str())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemeVariant {
    #[default]
    Dark,
    Light,
}

impl SchemeVariant {
    pub fn as_str(&self) -> &str {
        match self {
            SchemeVariant::Dark => "dark",
            SchemeVariant::Light => "light",
        }
    }
}

impl fmt::Display for SchemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "name: \"{}\"", self.as_str())?;
        Ok(())
    }
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "author: \"{}\"", self.author)?;
        if let Some(ref desc) = self.description {
            writeln!(f, "description: \"{}\"", desc)?;
        }
        writeln!(f, "name: \"{}\"", self.name)?;
        writeln!(f, "slug: \"{}\"", self.slug)?;
        writeln!(f, "system: \"{}\"", self.system)?;
        writeln!(f, "variant: \"{}\"", self.variant)?;
        writeln!(f, "palette:")?;

        let mut palette_vec: Vec<(String, Color)> = self
            .palette
            .clone()
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        palette_vec.sort_by_key(|k| k.0.clone());

        for (key, value) in palette_vec {
            writeln!(f, "  {}: \"{}\"", key, value)?;
        }
        Ok(())
    }
}

pub(crate) fn slugify(input: &str) -> String {
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

    let re = Regex::new(r"-+").expect("Unable to unwrap regex");
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
        let variant = wrapper.variant.unwrap_or(SchemeVariant::Dark);

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

impl Serialize for Scheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Scheme", 7)?;
        state.serialize_field("system", &self.system)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("slug", &self.slug)?;
        state.serialize_field("author", &self.author)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("variant", &self.variant)?;

        // Collect and sort the palette by key
        let mut sorted_palette: Vec<(&String, &Color)> = self.palette.iter().collect();
        sorted_palette.sort_by(|a, b| a.0.cmp(b.0));

        // Serialize the sorted palette as a map within the struct
        state.serialize_field("palette", &SortedPalette(sorted_palette))?;

        state.end()
    }
}

// Helper struct for serializing sorted palette
struct SortedPalette<'a>(Vec<(&'a String, &'a Color)>);

impl<'a> Serialize for SortedPalette<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            map.serialize_entry(key, &value.to_hex())?;
        }
        map.end()
    }
}
