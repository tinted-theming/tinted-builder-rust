mod base16;
mod color;

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub use crate::scheme::base16::Base16Scheme;
pub use crate::scheme::color::Color;
use crate::TintedBuilderError;

/// Enum representing schemes for different scheme systems. This enum is non-exhaustive, meaning
/// additional variants may be added in future versions without it being considered a breaking
/// change.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Scheme {
    /// Base16 variant with Base16Scheme deserialized content.
    Base16(Base16Scheme),
    /// Base24 variant with Base16Scheme deserialized content. Base16Scheme is built to support
    /// basic supersets of Base16 schemes.
    Base24(Base16Scheme),
}

impl Scheme {
    pub fn get_scheme_system(&self) -> SchemeSystem {
        match self {
            Scheme::Base16(_) => SchemeSystem::Base16,
            Scheme::Base24(_) => SchemeSystem::Base24,
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
}

impl SchemeSystem {
    /// Returns the string representation of the `SchemeSystem`.
    pub fn as_str(&self) -> &str {
        match self {
            SchemeSystem::Base16 => "base16",
            SchemeSystem::Base24 => "base24",
        }
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

/// Enum representing variants of a color scheme, such as Dark or Light. This enum is
/// non-exhaustive, meaning additional variants may be added in future versions without it being
/// considered a breaking change.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
    pub fn as_str(&self) -> &str {
        match self {
            SchemeVariant::Dark => "dark",
            SchemeVariant::Light => "light",
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
