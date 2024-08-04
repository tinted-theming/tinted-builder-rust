#[doc = include_str!("../README.md")]
mod constants;
mod scheme;
mod template;

pub use scheme::{Color, Scheme, SchemeType, TmScheme};
pub use template::{Template, TemplateContent};
