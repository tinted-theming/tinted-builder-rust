use serde::Serialize;

use crate::SchemeSystem;

#[derive(Debug, Clone, Serialize)]
pub struct SchemeMeta {
    pub system: SchemeSystem,
    pub name: String,
    pub author: String,
    pub theme_author: String,
    pub slug: String,
    pub supported_styling_version: String,
    pub supported_builder_version: String,
    pub description: Option<String>,
}
