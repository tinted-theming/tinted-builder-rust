#[doc = include_str!("../README.md")]
mod constants;
mod scheme;
mod template;

pub use scheme::{Color, Scheme};
pub use template::Template;
