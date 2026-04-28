pub mod structure;
mod yaml;

pub use crate::scheme::tinted8::structure::{Palette, Scheme, SyntaxKey, UiKey};
use crate::SchemeSystem;

pub const SUPPORTED_BUILDER_SPEC_VERSION: &str = "0.2.0";
pub const SUPPORTED_STYLING_SPEC_VERSION: &str = "0.2.0";
