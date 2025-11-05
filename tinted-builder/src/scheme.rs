mod base16;
mod color;
pub mod tinted8;

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub use crate::scheme::base16::Base16Scheme;
pub use crate::scheme::color::Color;
pub use crate::scheme::color::{ColorName, ColorVariant};
pub use crate::scheme::tinted8::Scheme as Tinted8Scheme;
use crate::TintedBuilderError;

/// Enum representing schemes for different scheme systems. This enum is non-exhaustive, meaning
/// additional variants may be added in future versions without it being considered a breaking
/// change.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Scheme {
    /// Base16 variant with `Base16Scheme` deserialized content.
    Base16(Base16Scheme),
    /// Base24 variant with `Base16Scheme` deserialized content. `Base16Scheme` is built to support
    /// basic supersets of Base16 schemes.
    Base24(Base16Scheme),
    /// Tinted8 scheme system with `Tinted8Scheme` deserialized content.
    Tinted8(Box<Tinted8Scheme>),
}

impl Scheme {
    /// Returns the author of the scheme.
    #[must_use]
    pub fn get_scheme_author(&self) -> String {
        match self {
            Self::Base16(scheme) | Self::Base24(scheme) => scheme.author.clone(),
            Self::Tinted8(scheme) => scheme.scheme.author.clone(),
        }
    }
    /// Returns the optional description (empty string when missing).
    #[must_use]
    pub fn get_scheme_description(&self) -> String {
        match self {
            Self::Base16(scheme) | Self::Base24(scheme) => {
                scheme.description.clone().unwrap_or_default()
            }
            Self::Tinted8(scheme) => scheme.scheme.description.clone().unwrap_or_default(),
        }
    }
    /// Returns the human-readable name of the scheme.
    #[must_use]
    pub fn get_scheme_name(&self) -> String {
        match self {
            Self::Base16(scheme) | Self::Base24(scheme) => scheme.name.clone(),
            Self::Tinted8(scheme) => scheme.scheme.name.clone(),
        }
    }
    /// Returns the scheme slug.
    #[must_use]
    pub fn get_scheme_slug(&self) -> String {
        match self {
            Self::Base16(scheme) | Self::Base24(scheme) => scheme.slug.clone(),
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
            Self::Base16(scheme) | Self::Base24(scheme) => scheme.variant.clone(),
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
