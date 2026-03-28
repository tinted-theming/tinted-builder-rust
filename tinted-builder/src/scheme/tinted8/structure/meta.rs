use crate::{SchemeSupports, SchemeVariant};
use serde::Serialize;

use crate::SchemeSystem;

#[derive(Debug, Clone, Serialize)]
pub struct SchemeMeta {
    pub system: SchemeSystem,
    pub name: String,
    pub author: String,
    pub theme_author: String,
    pub slug: String,
    pub supports: SchemeSupports,
    pub variant: SchemeVariant,
    pub family: Option<String>,
    pub style: Option<String>,
    pub description: Option<String>,
}
