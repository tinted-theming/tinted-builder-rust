use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use tinted_builder::Scheme;
use tinted_builder::Template;

const REPO_NAME: &str = env!("CARGO_PKG_NAME");
const DEFAULT_SYSTEM: &str = "base16";

fn match_scheme_file_extension(extension: &str) -> bool {
    extension == "yaml" || extension == "yml"
}

fn is_output_dir_as_expected(path: &Path, extension: &str) -> Result<bool> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            return Ok(false);
        }

        if entry.path().ends_with(extension) {
            return Ok(false);
        }
    }

    Ok(true)
}

// Allow for the use of `.yaml` and `.yml` extensions
fn get_theme_template_path(theme_template_path: &Path) -> Result<PathBuf> {
    if theme_template_path.join("templates/config.yml").is_file() {
        return Ok(theme_template_path.join("templates/config.yml"));
    }

    Ok(theme_template_path.join("templates/config.yaml"))
}

fn generate_theme(
    template: &Template,
    output_dir: &PathBuf,
    scheme_path: &PathBuf,
    system: &str,
    extension: &str,
) -> Result<()> {
    let scheme_file_extension: &str = scheme_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    if match_scheme_file_extension(scheme_file_extension) {
        let slug = scheme_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let scheme_str = read_to_string(scheme_path)?;
        let scheme: Scheme = serde_yaml::from_str(&scheme_str)?;
        let output_path = output_dir.join(format!("{}-{}.{}", &scheme.system, slug, extension));

        if scheme.system != system {
            return Ok(());
        }

        if !output_path.exists() {
            fs::create_dir_all(output_dir)?;
        }

        template.render_to_file(&output_path, &scheme)?;
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct TemplateConfig {
    extension: String,
    output: String,
    #[serde(rename = "supported-systems")]
    supported_systems: Option<Vec<String>>,
}

/// Build theme template using provided schemes
pub fn build(theme_template_path: &Path, user_schemes_path: &Path) -> Result<()> {
    if !user_schemes_path.exists() {
        return Err(anyhow!(
            "Schemes don't exist locally. First run `{} sync` and try again",
            REPO_NAME
        ));
    }

    let template_config_path = get_theme_template_path(theme_template_path)?;
    if !template_config_path.exists() || !template_config_path.is_file() {
        return Err(anyhow!(
            "The theme template config file is missing or not a valid yaml file: {}",
            template_config_path.display()
        ));
    }

    let template_config_content = read_to_string(template_config_path)?;
    let template_config: HashMap<String, TemplateConfig> =
        serde_yaml::from_str(&template_config_content)?;

    for (key, value) in template_config.iter() {
        let extension = value
            .extension
            .as_str()
            .strip_prefix('.')
            .unwrap_or(value.extension.as_str());
        let template_path = theme_template_path.join(format!("templates/{}.mustache", key));
        let template_content = read_to_string(&template_path).context(format!(
            "Mustache template missing: {}",
            template_path.display()
        ))?;
        let supported_systems = &value
            .supported_systems
            .clone()
            .unwrap_or(vec![DEFAULT_SYSTEM.to_string()]);
        let template = Template::new(template_content)?;
        let output_str = &value.output;
        let output_path = theme_template_path.join(output_str);

        if output_str.starts_with('/') {
            return Err(anyhow!(
                "`output` value in config.yaml only accepts relative paths: {}",
                output_str
            ));
        }

        if output_path.exists() && !is_output_dir_as_expected(&output_path, extension)? {
            let abs_path = output_path.canonicalize()?;

            return Err(anyhow!(
                "Output directory contains directories or non-theme files.\nManually remove the directory and try again: {}", abs_path.display()
            ));
        }

        if !output_path.exists() {
            create_dir_all(&output_path)?
        }

        let mut scheme_path_vec: Vec<(&str, PathBuf)> = Vec::new();

        for system in supported_systems {
            if user_schemes_path.join(system).is_dir() {
                let path = user_schemes_path.join(system);

                scheme_path_vec.push((system, path));
            } else {
                let path = user_schemes_path.to_path_buf();

                scheme_path_vec.push((system, path));
            }
        }

        for (system, schemes_path) in scheme_path_vec {
            for item_result in fs::read_dir(schemes_path)? {
                let scheme_direntry = item_result?;
                let scheme_file_path = scheme_direntry.path();

                generate_theme(
                    &template,
                    &output_path,
                    &scheme_file_path,
                    system,
                    extension,
                )?;
            }

            println!(
                "{} themes generated for \"{}\" at \"{}/{}-*.{}\"",
                system,
                key,
                output_path.display(),
                system,
                extension,
            );
        }
    }

    Ok(())
}
