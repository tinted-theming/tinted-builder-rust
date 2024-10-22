use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt};

pub use crate::scheme::color::Color;

use crate::{utils::slugify, SchemeSystem, SchemeVariant};

pub(crate) const REQUIRED_BASE16_PALETTE_KEYS: [&str; 16] = [
    "base00", "base01", "base02", "base03", "base04", "base05", "base06", "base07", "base08",
    "base09", "base0A", "base0B", "base0C", "base0D", "base0E", "base0F",
];

pub(crate) const REQUIRED_BASE24_PALETTE_KEYS: [&str; 24] = [
    "base00", "base01", "base02", "base03", "base04", "base05", "base06", "base07", "base08",
    "base09", "base0A", "base0B", "base0C", "base0D", "base0E", "base0F", "base10", "base11",
    "base12", "base13", "base14", "base15", "base16", "base17",
];

#[derive(Deserialize, Serialize)]
struct SchemeWrapper {
    pub(crate) system: SchemeSystem,
    pub(crate) name: String,
    pub(crate) slug: Option<String>,
    pub(crate) author: String,
    pub(crate) description: Option<String>,
    pub(crate) variant: Option<SchemeVariant>,
    pub(crate) palette: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Base16Scheme {
    pub system: SchemeSystem,
    pub name: String,
    pub slug: String,
    pub author: String,
    pub description: Option<String>,
    pub variant: SchemeVariant,
    pub palette: HashMap<String, Color>,
}

impl fmt::Display for Base16Scheme {
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
            writeln!(f, "  {}: \"#{}\"", key, value)?;
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Base16Scheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = SchemeWrapper::deserialize(deserializer)?;
        let slug = wrapper
            .slug
            .map_or(slugify(&wrapper.name), |slug| slugify(&slug));
        let variant = wrapper.variant.unwrap_or(SchemeVariant::Dark);

        match wrapper.system {
            SchemeSystem::Base16 => {
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
            SchemeSystem::Base24 => {
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

        Ok(Base16Scheme {
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

impl Serialize for Base16Scheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Scheme", 7)?;
        state.serialize_field("system", &self.system)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("slug", &self.slug)?;
        state.serialize_field("author", &self.author)?;
        if let Some(description) = &self.description {
            state.serialize_field("description", description)?;
        }
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
            map.serialize_entry(key, format!("#{}", &value.to_hex()).as_str())?;
        }
        map.end()
    }
}
