#![doc = include_str!("../README.md")]
mod error;
mod scheme;
mod template;
mod utils;

pub use error::TintedBuilderError;
pub use scheme::{Base16Scheme, Color, Scheme, SchemeSystem, SchemeVariant, Tinted8Scheme};
pub use template::Template;
