use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

use crate::Scheme;

pub struct Template {
    content: String,
}

pub(crate) fn write_to_file(path: &Path, contents: &str) -> Result<()> {
    if path.exists() {
        remove_file(path).with_context(|| format!("Unable to remove file: {}", path.display()))?;
    }

    let mut file =
        File::create(path).with_context(|| format!("Unable to create file: {}", path.display()))?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}

impl Template {
    pub fn new(content: String) -> Result<Template> {
        Ok(Template { content })
    }

    fn to_template_context(scheme: &Scheme) -> HashMap<String, String> {
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
        context.insert(
            "scheme-system".to_string(),
            scheme.system.as_str().to_string(),
        );
        context.insert(
            "scheme-variant".to_string(),
            scheme.variant.as_str().to_string(),
        );
        context.insert(
            format!("scheme-is-{}-variant", scheme.variant),
            "true".to_string(),
        );

        for (name, color) in scheme.palette.iter() {
            let hex = color.hex.clone();
            let rgb = color.rgb;

            context.insert(
                format!("{}-hex", name),
                format!("{}{}{}", color.hex.0, color.hex.1, color.hex.2),
            );
            context.insert(
                format!("{}-hex-bgr", name),
                format!("{}{}{}", color.hex.2, color.hex.1, color.hex.0),
            );
            context.insert(format!("{}-hex-r", name), hex.0);
            context.insert(format!("{}-hex-g", name), hex.1);
            context.insert(format!("{}-hex-b", name), hex.2);
            context.insert(format!("{}-rgb-r", name), rgb.0.to_string());
            context.insert(format!("{}-rgb-g", name), rgb.1.to_string());
            context.insert(format!("{}-rgb-b", name), rgb.2.to_string());
            context.insert(
                format!("{}-dec-r", name),
                format!("{:.8}", rgb.0 as f64 / 255.),
            );
            context.insert(
                format!("{}-dec-g", name),
                format!("{:.8}", rgb.1 as f64 / 255.),
            );
            context.insert(
                format!("{}-dec-b", name),
                format!("{:.8}", rgb.2 as f64 / 255.),
            );
        }

        context
    }

    pub fn render(&self, scheme: &Scheme) -> Result<String> {
        let context = serde_yaml::to_string(&Self::to_template_context(scheme))?;
        let rendered = ribboncurls::render(&self.content, &context, None)?;

        Ok(rendered)
    }

    #[deprecated(
        since = "0.4.0",
        note = "Please use the `render` method instead and write the output to a file yourself."
    )]
    pub fn render_to_file(&self, output_path: &Path, scheme: &Scheme) -> Result<&Self> {
        let context = serde_yaml::to_string(&Self::to_template_context(scheme))?;
        let rendered = ribboncurls::render(&self.content, &context, None)?;

        write_to_file(output_path, &rendered)?;

        Ok(self)
    }
}
