mod base;
mod color;
mod tmtheme;

use crate::scheme::base::BaseScheme;
pub use crate::scheme::color::Color;
use crate::scheme::tmtheme::TmThemeScheme;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

pub enum SchemeType {
    Base(BaseScheme),
    TmTheme(TmThemeScheme),
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
    TmTheme,
}

impl SchemeSystem {
    pub fn as_str(&self) -> &str {
        match self {
            SchemeSystem::Base16 => "base16",
            SchemeSystem::Base24 => "base24",
            SchemeSystem::TmTheme => "tmtheme",
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
