#![doc = include_str!("../README.md")]
mod error;
mod scheme;
mod template;
mod utils;

pub use error::TintedBuilderError;
pub use scheme::{
    Color, ColorName, ColorType, ColorVariant, Scheme, SchemeSupports, SchemeSystem, SchemeVariant,
};
pub use template::Template;

pub mod base16 {
    /// Base16 support for the library.
    ///
    /// - `Scheme`: deserialize Base16 YAML into this type and wrap in `Scheme::Base16` to render
    ///   templates.
    pub use crate::scheme::base16::Scheme;
}

pub mod base24 {
    /// Base24 support for the library.
    ///
    /// - `Scheme`: deserialize Base24 YAML into this type and wrap in `Scheme::Base24` to render
    ///   templates.
    pub use crate::scheme::base24::Scheme;
}

pub mod tinted8 {
    /// Tinted8 support for the library.
    ///
    /// - `Scheme`: deserialize Tinted8 YAML into this type and wrap in `Scheme::Tinted8` to render
    ///   templates with nested variables (`palette`, `ui`, `syntax`).
    /// - `SUPPORTED_STYLING_SPEC_VERSION` / `SUPPORTED_BUILDER_SPEC_VERSION`: version strings the
    ///   library targets; useful for compatibility checks.
    pub use crate::scheme::tinted8::{
        Palette, Scheme, SyntaxKey, UiKey, SUPPORTED_BUILDER_SPEC_VERSION,
        SUPPORTED_STYLING_SPEC_VERSION,
    };
}
