use std::collections::HashMap;

use crate::{error::TintedBuilderError, Base16Scheme};

pub fn render(content: &str, ctx: &HashMap<String, String>) -> Result<String, TintedBuilderError> {
    let ctx = serde_yaml::to_string(&ctx)?;
    let rendered = ribboncurls::render(content, &ctx, None)?;

    Ok(rendered)
}

pub fn to_template_context(scheme: &Base16Scheme) -> HashMap<String, String> {
    let mut context = HashMap::new();

    context.insert("scheme-name".to_string(), scheme.name.clone());
    context.insert("scheme-author".to_string(), scheme.author.clone());
    context.insert(
        "scheme-description".to_string(),
        scheme.description.clone().unwrap_or_default(),
    );
    context.insert("scheme-slug".to_string(), scheme.slug.clone());
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

    for (name, color) in &scheme.palette {
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
