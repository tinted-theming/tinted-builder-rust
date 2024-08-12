#[doc = include_str!("../README.md")]
mod constants;
mod scheme;
mod template;
mod utils;

pub use scheme::{Color, Scheme, SchemeSystem, SchemeVariant};
pub use template::{Template, TemplateContent};
