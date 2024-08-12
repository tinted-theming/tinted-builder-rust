pub mod tmtheme;
mod yaml;

use anyhow::{anyhow, Context, Result};
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

use crate::scheme::SchemeType;
use crate::{Scheme, SchemeSystem};

pub struct Template {
    content: String,
    scheme: SchemeType,
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
    pub fn new(content: String, scheme: SchemeType) -> Template {
        Template { content, scheme }
    }

    pub fn render(&self, scheme: &Scheme) -> Result<String> {
        match scheme.system {
            SchemeSystem::Base16 | SchemeSystem::Base24 => {
                let ctx = yaml::to_template_context(scheme);
                let rendered = yaml::render(&self.content, &ctx)?;

                Ok(rendered)
            }
            SchemeSystem::TmTheme => {
                let ctx = tmtheme::to_template_context(scheme)?;
                let rendered = tmtheme::render(content, &ctx)?;

                Ok(rendered)
            }
            _ => Err(anyhow!("Mismatch between template type and scheme type")),
        }
    }

    #[deprecated(
        since = "0.4.0",
        note = "Please use the `render` method instead and write the output to a file yourself."
    )]
    pub fn render_to_file(&self, output_path: &Path, scheme: &Scheme) -> Result<&Self> {
        match scheme.system {
            SchemeSystem::Base16 | SchemeSystem::Base24 => {
                let ctx = yaml::to_template_context(scheme);
                let context = serde_yaml::to_string(&ctx)?;
                let rendered = ribboncurls::render(&self.content, &context, None)?;

                write_to_file(output_path, &rendered)?;

                Ok(self)
            }
            // (TemplateContent::TmTheme(content), SchemeType::TmTheme(scheme)) => Ok(self),
            _ => Err(anyhow!("Mismatch between template type and scheme type")),
        }
    }
}
