use std::collections::HashMap;

use crate::error::TintedBuilderError;
use crate::scheme::{Color, SchemeSystem, SchemeVariant};

/// Renders a mustache template using a flat string context (Base16/Base24 variables).
pub fn render(content: &str, ctx: &HashMap<String, String>) -> Result<String, TintedBuilderError> {
    let ctx = serde_yaml::to_string(&ctx)?;
    let rendered = ribboncurls::render(content, &ctx, None)?;

    Ok(rendered)
}

/// Common fields shared by Base16 and Base24 schemes, used to build template context.
pub(crate) struct SchemeContext<'a> {
    pub name: &'a str,
    pub author: &'a str,
    pub description: Option<&'a str>,
    pub slug: &'a str,
    pub system: &'a SchemeSystem,
    pub variant: &'a SchemeVariant,
    pub palette: &'a HashMap<String, Color>,
}

impl<'a> From<&'a crate::scheme::base16::Scheme> for SchemeContext<'a> {
    fn from(s: &'a crate::scheme::base16::Scheme) -> Self {
        Self {
            name: &s.name,
            author: &s.author,
            description: s.description.as_deref(),
            slug: &s.slug,
            system: &s.system,
            variant: &s.variant,
            palette: &s.palette,
        }
    }
}

impl<'a> From<&'a crate::scheme::base24::Scheme> for SchemeContext<'a> {
    fn from(s: &'a crate::scheme::base24::Scheme) -> Self {
        Self {
            name: &s.name,
            author: &s.author,
            description: s.description.as_deref(),
            slug: &s.slug,
            system: &s.system,
            variant: &s.variant,
            palette: &s.palette,
        }
    }
}

/// Builds the flat Base16/Base24 variable context expected by templates.
///
/// Provides keys like `scheme-name`, `base0A-hex`, `base0A-hex-bgr`, `base0A-rgb-r`, etc.
pub fn to_template_context(scheme: &SchemeContext<'_>) -> HashMap<String, String> {
    let mut context = HashMap::new();

    context.insert("scheme-name".to_string(), scheme.name.to_string());
    context.insert("scheme-author".to_string(), scheme.author.to_string());
    context.insert(
        "scheme-description".to_string(),
        scheme.description.unwrap_or_default().to_string(),
    );
    context.insert("scheme-slug".to_string(), scheme.slug.to_string());
    context.insert(
        "scheme-slug-underscored".to_string(),
        scheme.slug.replace('-', "_"),
    );
    context.insert("scheme-system".to_string(), scheme.system.to_string());
    context.insert(
        "scheme-variant".to_string(),
        scheme.variant.as_str().to_string(),
    );
    context.insert(
        format!("scheme-is-{}-variant", scheme.variant),
        "true".to_string(),
    );

    for (name, color) in scheme.palette {
        let hex = color.hex.clone();
        let rgb = color.rgb;

        context.insert(
            format!("{name}-hex"),
            format!("{}{}{}", color.hex.0, color.hex.1, color.hex.2),
        );
        context.insert(
            format!("{name}-hex-bgr"),
            format!("{}{}{}", color.hex.2, color.hex.1, color.hex.0),
        );
        context.insert(format!("{name}-hex-r"), hex.0);
        context.insert(format!("{name}-hex-g"), hex.1);
        context.insert(format!("{name}-hex-b"), hex.2);
        context.insert(format!("{name}-rgb-r"), rgb.0.to_string());
        context.insert(format!("{name}-rgb-g"), rgb.1.to_string());
        context.insert(format!("{name}-rgb-b"), rgb.2.to_string());
        context.insert(
            format!("{name}-rgb16-r"),
            (u16::from(rgb.0) * 257_u16).to_string(),
        );
        context.insert(
            format!("{name}-rgb16-g"),
            (u16::from(rgb.1) * 257_u16).to_string(),
        );
        context.insert(
            format!("{name}-rgb16-b"),
            (u16::from(rgb.2) * 257_u16).to_string(),
        );
        context.insert(
            format!("{name}-dec-r"),
            format!("{:.8}", f64::from(rgb.0) / 255.),
        );
        context.insert(
            format!("{name}-dec-g"),
            format!("{:.8}", f64::from(rgb.1) / 255.),
        );
        context.insert(
            format!("{name}-dec-b"),
            format!("{:.8}", f64::from(rgb.2) / 255.),
        );
    }

    context
}
