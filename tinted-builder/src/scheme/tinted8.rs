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
struct YamlScheme {
    pub system: SchemeSystem,
    #[serde(rename = "system-version")]
    pub system_version: Option<String>,
    #[serde(rename = "scheme-author")]
    pub scheme_author: String,
    pub palette: HashMap<String, String>,
    pub variant: SchemeVariant,

    pub name: Option<String>,
    #[serde(rename = "syntax")]
    pub syntax: Option<HashMap<String, String>>,
    pub ui: Option<HashMap<String, String>>,
    #[serde(rename = "theme-author")]
    pub theme_author: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub family: Option<String>,
    pub style: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Scheme {
    pub system: SchemeSystem,
    pub system_version: Option<String>,
    pub name: String,
    pub scheme_author: String,
    pub theme_author: String,
    pub slug: String,
    pub variant: SchemeVariant,
    // pub palette: Palette,
    pub syntax: Syntax,
    pub ui: Ui,

    pub description: Option<String>,
    pub family: Option<String>,
    pub style: Option<String>,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "system: \"{}\"", self.system)?;
        writeln!(f, "name: \"{}\"", self.name)?;
        writeln!(f, "scheme-author: \"{}\"", self.scheme_author)?;
        writeln!(f, "theme-author: \"{}\"", self.theme_author)?;
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

        // let palette_vec: Vec<(String, Color)> = self
        //     .palette
        //     .clone()
        //     .iter()
        //     .map(|(k, v)| (k.clone(), v.clone()))
        //     .collect();

        // writeln!(f, "palette:")?;
        // for (key, value) in palette_vec {
        //     writeln!(f, "  {key}: \"{value}\"")?;
        // }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Scheme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wrapper = YamlScheme::deserialize(deserializer)?;

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

        // let syntax = Syntax

        Ok(Self {
            name,
            slug,
            system: wrapper.system,
            system_version: wrapper.system_version,
            scheme_author: wrapper.scheme_author.clone(),
            description: wrapper.description,
            family: wrapper.family,
            style: wrapper.style,
            variant: wrapper.variant,
            syntax: wrapper.syntax.unwrap_or_default(),
            theme_author: wrapper.theme_author.unwrap_or(wrapper.scheme_author),
            ui: wrapper.ui.unwrap_or_default(),
            // palette: palette_result?,
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
        state.serialize_field("scheme-author", &self.scheme_author)?;
        state.serialize_field("theme-author", &self.theme_author)?;
        if let Some(description) = &self.description {
            state.serialize_field("description", description)?;
        }
        state.serialize_field("variant", &self.variant)?;

        // Collect and sort the palette by key
        // let mut sorted_palette: Vec<(&String, &Color)> = self.palette.iter().collect();
        // sorted_palette.sort_by(|a, b| a.0.cmp(b.0));

        // Serialize the sorted palette as a map within the struct
        // state.serialize_field("palette", &SortedPalette(sorted_palette))?;

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

// #[derive(Debug, Clone)]
// struct Palette([[String; 3]; 11]);
//
// impl Palette {
//     fn get(&self, h: Hue, t: Tone) -> &str {
//         &self.0[h as usize][t as usize]
//     }
//     fn iter(&self) -> impl Iterator<Item = ((Hue, Tone), &str)> {
//         const HUES: [Hue; 11] = [
//             Hue::Black,
//             Hue::Red,
//             Hue::Green,
//             Hue::Yellow,
//             Hue::Blue,
//             Hue::Cyan,
//             Hue::Magenta,
//             Hue::White,
//             Hue::Orange,
//             Hue::Gray,
//             Hue::Brown,
//         ];
//         const TONES: [Tone; 3] = [Tone::Default, Tone::Bright, Tone::Dim];
//         HUES.into_iter()
//             .flat_map(move |h| TONES.into_iter().map(move |t| ((h, t), self.get(h, t))))
//     }
//     fin to_vec() -> &[]
// }
//
// struct Palette {
//     black_default: String,
//     black_bright: String,
//     black_dim: String,
//     red_default: String,
//     red_bright: String,
//     red_dim: String,
//     green_default: String,
//     green_bright: String,
//     green_dim: String,
//     yellow_default: String,
//     yellow_bright: String,
//     yellow_dim: String,
//     blue_default: String,
//     blue_bright: String,
//     blue_dim: String,
//     cyan_default: String,
//     cyan_bright: String,
//     cyan_dim: String,
//     magenta_default: String,
//     magenta_bright: String,
//     magenta_dim: String,
//     white_default: String,
//     white_bright: String,
//     white_dim: String,
//     orange_default: String,
//     orange_bright: String,
//     orange_dim: String,
//     gray_default: String,
//     gray_bright: String,
//     gray_dim: String,
//     brown_default: String,
//     brown_bright: String,
//     brown_dim: String,
// }
//
// #[repr(u8)]
// #[derive(Copy, Clone, Debug)]
// enum Hue {
//     Black,
//     Red,
//     Green,
//     Yellow,
//     Blue,
//     Cyan,
//     Magenta,
//     White,
//     Orange,
//     Gray,
//     Brown,
// }
// #[repr(u8)]
// #[derive(Copy, Clone, Debug)]
// enum Tone {
//     Default,
//     Bright,
//     Dim,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Ui {
    background: Option<String>,
    #[serde(rename = "background-dark")]
    background_dark: Option<String>,
    #[serde(rename = "background-light")]
    background_light: Option<String>,
    deprecated: Option<String>,
    foreground: Option<String>,
    #[serde(rename = "foreground-dark")]
    foreground_dark: Option<String>,
    #[serde(rename = "foreground-light")]
    foreground_light: Option<String>,
    #[serde(rename = "line-background")]
    line_background: Option<String>,
    #[serde(rename = "search-text")]
    search_text: Option<String>,
    #[serde(rename = "selection-background")]
    selection_background: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Syntax {
    comment: Option<String>,
    string: Option<SyntaxString>,
    constant: Option<SyntaxConstant>,
    entity: Option<SyntaxEntity>,
    keyword: Option<SyntaxKeyword>,
    markup: Option<SyntaxMarkup>,
    diff: Option<SyntaxDiff>,
}

impl Default for Syntax {
    fn default() -> Self {
        Self {
            comment: None,
            string: None,
            constant: None,
            entity: None,
            keyword: None,
            markup: None,
            diff: None,
        }
    }
}

impl TryFrom<HashMap<String, String>> for Syntax {
    type Error = serde_yaml::Error;
    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut s = Syntax::default();

        s.comment = map.remove("comment");
        s.string = map
            .remove("string")
            .and_then(|item| SyntaxString::try_from(item).ok());
        s.constant = map
            .remove("constant")
            .map(|v| v.parse())
            .transpose()
            .map_err(|e| format!("invalid constant: {e}"))?;
        s.entity = map
            .remove("entity")
            .map(|v| v.parse())
            .transpose()
            .map_err(|e| format!("invalid entity: {e}"))?;
        s.keyword = map
            .remove("keyword")
            .map(|v| v.parse())
            .transpose()
            .map_err(|e| format!("invalid keyword: {e}"))?;
        s.markup = map
            .remove("markup")
            .map(|v| v.parse())
            .transpose()
            .map_err(|e| format!("invalid markup: {e}"))?;
        s.diff = map
            .remove("diff")
            .map(|v| v.parse())
            .transpose()
            .map_err(|e| format!("invalid diff: {e}"))?;

        // Optional: error on unknown keys
        if !map.is_empty() {
            return Err(format!(
                "unknown keys: {:?}",
                map.keys().collect::<Vec<_>>()
            ));
        }

        Ok(s)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxEntity {
    name: Option<SyntaxEntityName>,
    other: Option<SyntaxEntityOther>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxEntityName {
    class: Option<String>,
    function: Option<String>,
    tag: Option<String>,
    variable: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxEntityOther {
    #[serde(rename = "attribute-name")]
    attribute_name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxKeyword {
    control: Option<String>,
    declaration: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxMarkup {
    bold: Option<String>,
    code: Option<String>,
    italic: Option<String>,
    quote: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxDiff {
    added: Option<String>,
    changed: Option<String>,
    deleted: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxConstant {
    numeric: Option<SyntaxConstantNumeric>,
    language: Option<SyntaxConstantLanguage>,
    character: Option<SyntaxConstantCharacter>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxConstantNumeric {
    integer: Option<String>,
    float: Option<String>,
    hex: Option<String>,
    exponential: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxConstantLanguage {
    boolean: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxConstantCharacter {
    escape: Option<String>,
    entity: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SyntaxString {
    quoted: Option<String>,
    regexp: Option<String>,
    template: Option<String>,
}

macro_rules! impl_tryfrom_yaml_map {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl<TV> ::std::convert::TryFrom<::std::collections::HashMap<::std::string::String, TV>> for $ty
            where
                $ty: ::serde::de::DeserializeOwned,
                ::std::collections::HashMap<::std::string::String, TV>: ::serde::Serialize,
            {
                type Error = ::serde_yaml::Error;
                fn try_from(map: ::std::collections::HashMap<::std::string::String, TV>) -> ::std::result::Result<Self, Self::Error> {
                    ::serde_yaml::from_value(::serde_yaml::to_value(map)?)
                }
            }
        )+
    };
}

impl_tryfrom_yaml_map!(
    SyntaxString,
    SyntaxConstant,
    SyntaxEntity,
    SyntaxKeyword,
    SyntaxMarkup,
    SyntaxDiff
);
