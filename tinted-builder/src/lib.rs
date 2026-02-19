#![doc = include_str!("../README.md")]
mod error;
mod scheme;
mod template;
mod utils;

pub use error::TintedBuilderError;
pub use scheme::{
    Base16Scheme, Color, ColorName, ColorVariant, Scheme, SchemeSystem, SchemeVariant,
};
pub use template::Template;

pub mod tinted8 {
    /// Tinted8 support for the library.
    ///
    /// - `Scheme`: deserialize Tinted8 YAML into this type and wrap in `Scheme::Tinted8` to render
    ///   templates with nested variables (`palette`, `ui`, `syntax`).
    /// - `SUPPORTED_STYLING_SPEC_VERSION` / `SUPPORTED_BUILDER_SPEC_VERSION`: version strings the
    ///   library targets; useful for compatibility checks.
    pub use crate::scheme::tinted8::{
        Scheme, SUPPORTED_BUILDER_SPEC_VERSION, SUPPORTED_STYLING_SPEC_VERSION,
    };
}
