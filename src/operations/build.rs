use anyhow::{anyhow, Result};
use base16_color_scheme::{Scheme, Template};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, read_to_string, create_dir, create_dir_all};
use std::path::{PathBuf, Path};

use crate::constants::REPO_NAME;

const SCHEME_EXTENSION: &str = "yaml";

fn is_output_dir_as_expected(path: &Path, extension: &str) -> Result<bool> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            return Ok(false);
        }

        if entry.path().extension().unwrap_or_default() != extension {
            return Ok(false)
        }
    }

    Ok(true)
}

fn generate_theme(
    template: &Template,
    output_dir: &PathBuf,
    scheme_path: &PathBuf,
    extension: &str,
) -> Result<()> {
    let scheme_file_extension: &str = &scheme_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    if scheme_file_extension == SCHEME_EXTENSION {
        let slug = scheme_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let scheme_str = read_to_string(&scheme_path).unwrap();
        let scheme: Scheme = serde_yaml::from_str(&scheme_str).unwrap();
        let output_path = output_dir.join(format!("{}{}", slug, extension));

        if !output_path.exists() {
            fs::create_dir_all(&output_dir)?;
        }

        template.render_to_file(&output_path, &scheme).unwrap();
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct TemplateConfig {
    extension: String,
    output: String,
}

/// Build theme template using provided schemes
pub fn build(template_path: &PathBuf, user_schemes_path: &PathBuf) -> Result<()> {
    // todo: Add base24 support and supported_systems property
    if !user_schemes_path.exists() {
        return Err(anyhow!(
            "Schemes don't exist locally. First run `{} sync` and try again",
            REPO_NAME
        ));
    }

    let template_config_path = template_path.join("templates/config.yaml");
    // todo: Make sure file exists

    let template_config_content = read_to_string(&template_config_path)?;
    let template_config: HashMap<String, TemplateConfig> =
        serde_yaml::from_str(&template_config_content)?;

    for (key, value) in template_config.iter() {
        // todo: fix all unwraps
        let extension = value.extension.as_str().strip_prefix(".").unwrap_or(value.extension.as_str());
        let template_content =
            read_to_string(template_path.join(format!("templates/{}.mustache", key))).unwrap();
        let template = Template::new(template_content).unwrap();
        let output_str = &value.output;
        let output_path = PathBuf::from(output_str);

        if output_str.starts_with("/") {
            return Err(anyhow!("`output` value in config.yaml only accepts relative paths: {}", output_str));
        }

        if output_path.exists() && !is_output_dir_as_expected(&output_path, extension)? {
            return Err(anyhow!("Output directory contains directories or non-theme files: {}", &output_path.display()));
        }

        if !output_path.exists() {
            create_dir_all(&output_path)?
        }

        for item_result in fs::read_dir(user_schemes_path)? {
            let scheme_direntry = item_result?;
            let scheme_path = scheme_direntry.path();

            generate_theme(&template, &output_path, &scheme_path, extension)?;
        }

        println!("Themes generated for \"{}\" at \"{}\"", key, output_path.display());
    }

    Ok(())
}
