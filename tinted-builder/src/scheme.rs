pub mod base16;
pub mod base24;
mod color;
pub mod tinted8;

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub use crate::scheme::color::Color;
pub use crate::scheme::color::{ColorName, ColorType, ColorVariant};
use crate::TintedBuilderError;

/// Enum representing schemes for different scheme systems. This enum is non-exhaustive, meaning
/// additional variants may be added in future versions without it being considered a breaking
/// change.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Scheme {
    /// Base16 variant with `base16::Scheme` deserialized content.
    Base16(base16::Scheme),
    /// Base24 variant with `base24::Scheme` deserialized content.
    Base24(base24::Scheme),
    /// Tinted8 scheme system with `tinted8::Scheme` deserialized content.
    Tinted8(Box<tinted8::Scheme>),
}

impl Scheme {
    /// Parse a YAML string into a `Scheme`, auto-detecting the system.
    ///
    /// Inspects the `system` field (top-level or nested under `scheme`) to
    /// determine which variant to deserialize. Defaults to Base16 if no
    /// `system` field is found.
    ///
    /// # Errors
    ///
    /// Returns a `TintedBuilderError` if the YAML is malformed, the system
    /// is unrecognised, or the scheme fails validation.
    pub fn from_yaml(yaml: &str) -> Result<Self, TintedBuilderError> {
        let raw: serde_yaml::Value =
            serde_yaml::from_str(yaml).map_err(TintedBuilderError::YamlDeserialize)?;

        let system = raw
            .get("system")
            .or_else(|| raw.get("scheme").and_then(|s| s.get("system")))
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| TintedBuilderError::SchemeMissingProperty("system".to_string()))?
            .parse::<SchemeSystem>()?;

        match system {
            SchemeSystem::Base16 => {
                let scheme: base16::Scheme =
                    serde_yaml::from_str(yaml).map_err(TintedBuilderError::YamlDeserialize)?;
                Ok(Self::Base16(scheme))
            }
            SchemeSystem::Base24 => {
                let scheme: base24::Scheme =
                    serde_yaml::from_str(yaml).map_err(TintedBuilderError::YamlDeserialize)?;
                Ok(Self::Base24(scheme))
            }
            SchemeSystem::Tinted8 => {
                let scheme: tinted8::Scheme =
                    serde_yaml::from_str(yaml).map_err(TintedBuilderError::YamlDeserialize)?;
                Ok(Self::Tinted8(Box::new(scheme)))
            }
        }
    }

    /// Returns the author of the scheme.
    #[must_use]
    pub fn get_scheme_author(&self) -> String {
        match self {
            Self::Base16(scheme) => scheme.author.clone(),
            Self::Base24(scheme) => scheme.author.clone(),
            Self::Tinted8(scheme) => scheme.scheme.author.clone(),
        }
    }
    /// Returns the optional description (empty string when missing).
    #[must_use]
    pub fn get_scheme_description(&self) -> String {
        match self {
            Self::Base16(scheme) => scheme.description.clone().unwrap_or_default(),
            Self::Base24(scheme) => scheme.description.clone().unwrap_or_default(),
            Self::Tinted8(scheme) => scheme.scheme.description.clone().unwrap_or_default(),
        }
    }
    /// Returns the human-readable name of the scheme.
    #[must_use]
    pub fn get_scheme_name(&self) -> String {
        match self {
            Self::Base16(scheme) => scheme.name.clone(),
            Self::Base24(scheme) => scheme.name.clone(),
            Self::Tinted8(scheme) => scheme.scheme.name.clone(),
        }
    }
    /// Returns the scheme slug.
    #[must_use]
    pub fn get_scheme_slug(&self) -> String {
        match self {
            Self::Base16(scheme) => scheme.slug.clone(),
            Self::Base24(scheme) => scheme.slug.clone(),
            Self::Tinted8(scheme) => scheme.scheme.slug.clone(),
        }
    }
    /// Returns the scheme system for this variant.
    #[must_use]
    pub const fn get_scheme_system(&self) -> SchemeSystem {
        match self {
            Self::Base16(_) => SchemeSystem::Base16,
            Self::Base24(_) => SchemeSystem::Base24,
            Self::Tinted8(_) => SchemeSystem::Tinted8,
        }
    }
    /// Returns the scheme variant (light or dark).
    #[must_use]
    pub fn get_scheme_variant(&self) -> SchemeVariant {
        match self {
            Self::Base16(scheme) => scheme.variant.clone(),
            Self::Base24(scheme) => scheme.variant.clone(),
            Self::Tinted8(scheme) => scheme.variant.clone(),
        }
    }
}

/// Enum representing the scheme system. This enum is non-exhaustive, meaning additional variants
/// may be added in future versions without it being considered a breaking change.
#[non_exhaustive]
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemeSystem {
    /// Base16 scheme system, the default.
    #[default]
    Base16,
    /// Base24 scheme system.
    Base24,
    /// Tinted8 scheme system.
    Tinted8,
}
#[derive(Debug, Clone, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct SchemeSupports {
    #[serde(rename = "styling-spec")]
    pub styling_spec: String,
}

impl SchemeSystem {
    /// Returns the string representation of the `SchemeSystem`.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Base16 => "base16",
            Self::Base24 => "base24",
            Self::Tinted8 => "tinted8",
        }
    }
    #[must_use]
    pub const fn variants() -> &'static [Self] {
        &[Self::Base16, Self::Base24, Self::Tinted8]
    }
}

impl fmt::Display for SchemeSystem {
    /// Formats the `SchemeSystem` for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())?;
        Ok(())
    }
}

impl FromStr for SchemeSystem {
    type Err = TintedBuilderError;

    /// Parses a string to create a `SchemeSystem`.
    ///
    /// # Errors
    ///
    /// Returns a `TintedBuilderError` if the input string does not match
    /// any valid scheme variant.
    fn from_str(system_str: &str) -> Result<Self, Self::Err> {
        match system_str {
            "base16" => Ok(Self::Base16),
            "base24" => Ok(Self::Base24),
            "tinted8" => Ok(Self::Tinted8),
            _ => Err(TintedBuilderError::InvalidSchemeSystem(
                system_str.to_string(),
            )),
        }
    }
}

/// Enum representing variants of a color scheme (Dark or Light). This enum is non-exhaustive,
/// meaning additional variants may be added in future versions without it being considered a
/// breaking change.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemeVariant {
    /// Dark variant of the color scheme, the default.
    #[default]
    Dark,
    /// Light variant of the color scheme.
    Light,
}

impl FromStr for SchemeVariant {
    type Err = TintedBuilderError;

    /// Parses a string to create a `SchemeVariant`.
    ///
    /// # Errors
    ///
    /// Returns a `TintedBuilderError` if the input string does not match
    /// any valid scheme variant.
    fn from_str(variant_str: &str) -> Result<Self, Self::Err> {
        match variant_str {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err(TintedBuilderError::InvalidSchemeVariant(
                variant_str.to_string(),
            )),
        }
    }
}

impl SchemeVariant {
    /// Returns the string representation of the `SchemeVariant`.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }
}

impl fmt::Display for SchemeVariant {
    /// Formats the `SchemeVariant` for display purposes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())?;
        Ok(())
    }
}
