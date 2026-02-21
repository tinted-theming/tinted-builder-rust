use crate::{error::TintedBuilderError, tinted8::Scheme as Tinted8Scheme};
use serde::Serialize;

/// Render a template with any serializable context.
///
/// This allows passing structured (YAML) objects so templates can
/// access nested fields like `blue-bright.hex`.
pub fn render<T: serde::Serialize>(content: &str, ctx: &T) -> Result<String, TintedBuilderError> {
    let ctx = serde_yaml::to_string(&ctx)?;
    let rendered = ribboncurls::render(content, &ctx, None)?;

    Ok(rendered)
}

/// Build a structured context for templates.
///
/// Returns a nested YAML value while also preserving the legacy
/// flat string keys for backward compatibility.
#[derive(Serialize)]
struct SchemeMetaCtx {
    name: String,
    author: String,
    description: String,
    slug: String,
    #[serde(rename = "slug-underscored")]
    slug_underscored: String,
    system: String,
    variant: String,
    #[serde(rename = "builder-version")]
    builder_version: String,
    #[serde(rename = "system-version")]
    system_version: String,
    family: String,
    style: String,
}

#[derive(Serialize)]
struct TemplateCtx<'a> {
    scheme: SchemeMetaCtx,
    palette: &'a crate::scheme::tinted8::structure::Palette,
    syntax: &'a crate::scheme::tinted8::structure::Syntax,
    ui: &'a crate::scheme::tinted8::structure::Ui,
}

/// Builds a structured YAML context for Tinted8 templates.
///
/// The context exposes nested objects under `scheme`, `palette`, `ui`, and `syntax` matching
/// the Tinted8 builder specification. Color objects include `hex`, `hex-r/g/b`, `hex-bgr`,
/// `rgb`, `rgb16`, and `dec` fields.
pub fn to_template_context(
    scheme: &Tinted8Scheme,
) -> Result<serde_yaml::Value, TintedBuilderError> {
    let meta = &scheme.scheme;
    let scheme_ctx = SchemeMetaCtx {
        name: meta.name.clone(),
        author: meta.author.clone(),
        description: meta.description.clone().unwrap_or_default(),
        variant: meta.variant.to_string(),
        slug: meta.slug.clone(),
        slug_underscored: meta.slug.replace('-', "_"),
        system: meta.system.to_string(),
        builder_version: meta.supported_builder_version.clone(),
        system_version: meta.supported_styling_version.clone(),
        family: meta.family.clone().unwrap_or_default(),
        style: meta.style.clone().unwrap_or_default(),
    };

    let ctx = TemplateCtx {
        scheme: scheme_ctx,
        palette: &scheme.palette,
        syntax: &scheme.syntax,
        ui: &scheme.ui,
    };

    Ok(serde_yaml::to_value(&ctx)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_includes_hex_bgr() {
        let yml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  author: "User <user@example.com>"
  name: "Test"
  slug: "test"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
        let s: Tinted8Scheme = serde_yaml::from_str(yml).expect("unable to deserialize");
        let val = to_template_context(&s).expect("Unable to get template context");
        let root = val.as_mapping().expect("unable to get mapping");
        let palette = root
            .get(serde_yaml::Value::String("palette".into()))
            .expect("'palette' does not exist")
            .as_mapping()
            .expect("unable to get mapping");
        let blue = palette
            .get(serde_yaml::Value::String("blue".into()))
            .expect("'blue' does not exist")
            .as_mapping()
            .expect("unable to get mapping");
        let normal = blue
            .get(serde_yaml::Value::String("normal".into()))
            .expect("'normal' does not exist")
            .as_mapping()
            .expect("unable to get mapping");
        assert!(normal.contains_key(serde_yaml::Value::String("hex-bgr".into())));
    }
}
