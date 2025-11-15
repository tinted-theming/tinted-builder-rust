use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, fmt};

pub use crate::scheme::color::Color;

use crate::utils::titlecasify;
use crate::{utils::slugify, SchemeSystem, SchemeVariant};

pub const REQUIRED_TINTED8_PALETTE_KEYS: [&str; 8] = [
    "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
];

#[derive(Deserialize, Serialize)]
struct YamlTinted8Scheme {
    pub system: SchemeSystem,
    #[serde(rename = "scheme-author")]
    pub scheme_author: String,
    pub palette: HashMap<String, String>,
    pub variant: SchemeVariant,

    pub name: Option<String>,
    pub theme: Option<HashMap<String, String>>,
    pub ui: Option<HashMap<String, String>>,
    #[serde(rename = "theme-author")]
    pub theme_author: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub family: Option<String>,
    pub style: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Tinted8Scheme {
    pub system: SchemeSystem,
    pub name: String,
    pub scheme_author: String,
    pub slug: String,
    pub variant: SchemeVariant,
    pub palette: HashMap<String, Color>,
    pub theme: HashMap<String, String>,
    pub ui: HashMap<String, String>,

    pub theme_author: Option<String>,
    pub description: Option<String>,
    pub family: Option<String>,
    pub style: Option<String>,
}

impl fmt::Display for Tinted8Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "system: \"{}\"", self.system)?;
        writeln!(f, "name: \"{}\"", self.name)?;
        writeln!(f, "scheme-author: \"{}\"", self.scheme_author)?;
        if let Some(ref theme_author) = self.theme_author {
            writeln!(f, "theme_author: \"{theme_author}\"")?;
        }
        writeln!(f, "slug: \"{}\"", self.slug)?;
        writeln!(f, "variant: \"{}\"", self.variant)?;
        if let Some(ref family) = self.family {
            writeln!(f, "family: \"{family}\"")?;
        }
        if let Some(ref style) = self.style {
            writeln!(f, "style: \"{style}\"")?;
        }
        if let Some(ref desc) = self.description {
            writeln!(f, "description: \"{desc}\"")?;
        }

        let mut palette_vec: Vec<(String, Color)> = self
            .palette
            .clone()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        writeln!(f, "palette:")?;
        for (key, value) in palette_vec {
            writeln!(f, "  {key}: \"{value}\"")?;
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Tinted8Scheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = YamlTinted8Scheme::deserialize(deserializer)?;

        let (name, slug): (String, String) = match (
            &wrapper.name,
            &wrapper.slug,
            &wrapper.family,
            &wrapper.style,
        ) {
            (Some(name), None, _, _) => (name.to_owned(), slugify(name)),
            (None, Some(slug), _, _) => (titlecasify(slug), slug.to_owned()),
            (None, None, Some(family), Some(style)) => {
                let name = format!("{family}-{style}");

                (name.clone(), slugify(&name))
            }
            (None, None, Some(family), None) => {
                let name = family.clone();

                (name.clone(), slugify(&name))
            }
            _ => {
                return Err(serde::de::Error::custom(
                    "Either 'name', 'slug' or 'family' must exist in yaml scheme",
                ))
            }
        };

        match wrapper.system {
            SchemeSystem::Tinted8 => {
                let contains_all_keys = REQUIRED_TINTED8_PALETTE_KEYS
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
                    "Tinted8 does not support \"{}\" scheme system",
                    wrapper.system
                )));
            }
        }

        let palette_result: Result<HashMap<String, Color>, _> = wrapper
            .palette
            .into_iter()
            .map(|(key, value)| {
                Color::new(&value)
                    .map(|color| (key, color))
                    .map_err(|e| serde::de::Error::custom(e.to_string()))
            })
            .collect();

        Ok(Self {
            name,
            slug,
            system: wrapper.system,
            scheme_author: wrapper.scheme_author,
            description: wrapper.description,
            family: wrapper.family,
            style: wrapper.style,
            variant: wrapper.variant,
            theme: wrapper.theme.expect("Unable to extract theme"),
            theme_author: wrapper.theme_author,
            ui: wrapper.ui.expect("Unable to extract ui"),
            palette: palette_result?,
        })
    }
}

impl Serialize for Tinted8Scheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Scheme", 7)?;
        state.serialize_field("system", &self.system)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("slug", &self.slug)?;
        state.serialize_field("scheme-author", &self.scheme_author)?;
        if let Some(description) = &self.description {
            state.serialize_field("description", description)?;
        }
        if let Some(theme_author) = &self.theme_author {
            state.serialize_field("theme-author", theme_author)?;
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

#[allow(clippy::elidable_lifetime_names)]
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
