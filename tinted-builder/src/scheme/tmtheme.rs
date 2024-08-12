use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::fmt;
use tmtheme_deserialize::Settings;

use crate::{SchemeSystem, SchemeVariant};

pub(crate) const REQUIRED_META_KEYS: &[&str] = &["system", "variant", "name", "author"];

#[derive(Clone, Debug, PartialEq)]
pub enum SettingValue {
    String(String),
    HashMap(Settings),
}

impl Serialize for SettingValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SettingValue::String(value) => serializer.serialize_str(value),
            SettingValue::HashMap(value) => {
                let mut map = serializer.serialize_map(Some(value.len()))?;
                for (k, v) in value {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

impl fmt::Display for SettingValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingValue::String(value) => writeln!(f, "{}", value),
            SettingValue::HashMap(value) => {
                for (key, value) in value {
                    writeln!(f, "  {}: \"{}\"", key, value)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TmThemeScheme {
    pub system: SchemeSystem,
    pub name: String,
    pub slug: String,
    pub author: String,
    pub description: Option<String>,
    pub variant: SchemeVariant,
    pub settings: Settings,
}
