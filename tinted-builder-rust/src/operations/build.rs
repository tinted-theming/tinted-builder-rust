use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use tinted_builder::{Scheme, SchemeSystem, Template};

use crate::utils::write_to_file;

const REPO_NAME: &str = env!("CARGO_PKG_NAME");

/// Builds themes using the provided template and user schemes.
///
/// This function is typically invoked as part of a CLI operation, such as `tinted-builder-rust
/// build`. It reads a theme template configuration file, processes user-defined color schemes, and
/// generates the appropriate themes based on the configuration. The function assumes that the
/// necessary scheme files have been synchronized locally. If they are not present, it prompts the
/// user to sync them first.
///
/// # Arguments
///
/// * `theme_template_path` - A reference to a `Path` representing the path to the theme template
///   directory or file. * `user_schemes_path` - A reference to a `Path` representing the directory
///   where user schemes are stored. * `is_quiet` - A boolean flag that, when set to `true`,
///   suppresses most of the output, making the build process quieter.
///
/// # Returns
///
/// Returns a `Result<()>` indicating success (`Ok(())`) or an error (`Err`) if any issues are
/// encountered during the build process.
///
/// # Errors
///
/// This function can return an error in several scenarios:
///
/// * If the user schemes directory does not exist locally, it suggests running the `sync` command
///   first. * If the theme template configuration file is missing or invalid (e.g., not a valid YAML
///   file). * If there are issues reading the template configuration or parsing it as a YAML file. *
///   If there are errors during the theme generation process for any configuration.
///
/// # Usage
///
/// This function is intended to be called from a CLI context, as in:
///
/// ```sh
/// tinted-builder-rust build /path/to/theme-template
/// ```
///
/// The function will read the configuration from the specified paths and generate the
/// corresponding themes.
pub fn build(theme_template_path: &Path, user_schemes_path: &Path, is_quiet: bool) -> Result<()> {
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
    let schemes_filetypes = get_recursive_scheme_paths_from_dir(user_schemes_path)?;

    // For each template definition in the templates/config.yaml file
    for (config_name, config_value) in template_config.iter() {
        generate_themes_for_config(
            config_name,
            config_value,
            theme_template_path,
            &schemes_filetypes,
            is_quiet,
        )?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
enum SchemeFileType {
    Yaml(PathBuf),
    Yml(PathBuf),
}

impl SchemeFileType {
    pub fn new(path: &Path) -> Result<Self> {
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        match extension {
            "yaml" => Ok(Self::Yaml(path.to_path_buf())),
            "yml" => Ok(Self::Yml(path.to_path_buf())),
            _ => Err(anyhow!("Invalid file extension: {}", extension.to_string())),
        }
    }

    pub fn get_scheme(&self) -> Result<Scheme> {
        match self {
            Self::Yaml(path) | Self::Yml(path) => {
                let scheme_str = read_to_string(path)?;
                let scheme: serde_yaml::Value = serde_yaml::from_str(&scheme_str)?;

                if let serde_yaml::Value::Mapping(map) = scheme {
                    match map.get("system") {
                        Some(serde_yaml::Value::String(system_str))
                            if system_str == &SchemeSystem::Base24.to_string() =>
                        {
                            let scheme_inner =
                                serde_yaml::from_value(serde_yaml::Value::Mapping(map))?;
                            let scheme = Scheme::Base24(scheme_inner);

                            Ok(scheme)
                        }
                        None | Some(_) => {
                            let scheme_inner =
                                serde_yaml::from_value(serde_yaml::Value::Mapping(map))?;
                            let scheme = Scheme::Base16(scheme_inner);

                            Ok(scheme)
                        }
                    }
                } else {
                    Err(anyhow!("Unable to get scheme from SchemeFileType"))
                }
            }
        }
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        match self {
            Self::Yaml(path) | Self::Yml(path) => Some(path.to_path_buf()),
        }
    }

    pub fn get_file_stem(&self) -> Result<String> {
        match self {
            Self::Yaml(path) | Self::Yml(path) => {
                let file_stem = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();

                if file_stem.is_empty() {
                    Err(anyhow!(
                        "Unable to extract file_stem from path: {}",
                        path.display()
                    ))
                } else {
                    Ok(file_stem.to_string())
                }
            }
        }
    }
}

// Allow for the use of `.yaml` and `.yml` extensions
fn get_theme_template_path(theme_template_path: &Path) -> Result<PathBuf> {
    if theme_template_path.join("templates/config.yml").is_file() {
        return Ok(theme_template_path.join("templates/config.yml"));
    }

    Ok(theme_template_path.join("templates/config.yaml"))
}

/// Generates a theme file based on a given template and scheme.
///
/// This function processes a scheme file and generates a themed output file
/// in the specified directory. It reads the scheme data, applies it to the template,
/// and writes the output to a file with the appropriate extension.
///
/// The function also filters out hidden files (those whose names start with a `.`)
/// and ensures that the scheme system matches the provided `SchemeSystem`.
///
/// # Arguments
///
/// * `template_content` - A reference to a string slice containing the template's content.
/// * `output_dir` - A reference to a `PathBuf` representing the directory where the output file will be written.
/// * `scheme_path` - A reference to a `Path` representing the file path to the scheme file.
/// * `system` - The `SchemeSystem` that the scheme file should match.
/// * `explicit_extension` - A string slice representing the file extension for the generated theme
///   file. The parameter is named "explict" extension because it includes the "dot" or lack thereof
///
/// # Returns
///
/// Returns `Result<()>` indicating success (`Ok(())`) or an error (`Err`) if any of the following conditions are met:
///
/// * The scheme file cannot be read or parsed.
/// * The output directory cannot be created.
/// * There is an issue with writing the output file.
/// * The scheme file's system does not match the provided `SchemeSystem`.
///
/// # Errors
///
/// This function can return an error in several scenarios:
///
/// * If the scheme file cannot be read from the specified path.
/// * If the scheme file content cannot be parsed into a `Base16Scheme`.
/// * If the output directory cannot be created.
/// * If the template cannot be rendered with the provided scheme.
/// * If there is an issue writing the generated output to the file.
/// * If the scheme file's system does not match the provided `SchemeSystem`.
///
/// Note: This function skips processing hidden files (files whose names start with a `.`).
fn generate_theme(
    template_content: &str,
    output_dir: &PathBuf,
    scheme_path: &Path,
    system: SchemeSystem,
    explicit_extension: &str,
) -> Result<()> {
    let scheme_file_type = SchemeFileType::new(scheme_path)?;
    let scheme_path = scheme_file_type
        .get_path()
        .ok_or(anyhow!("Unable to get path from FileType"))?;
    let scheme_file_stem = scheme_path
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    // Ignore hidden files
    if scheme_file_stem.starts_with('.') {
        return Ok(());
    }

    let scheme = scheme_file_type.get_scheme()?;

    match &scheme {
        Scheme::Base16(scheme_inner) | Scheme::Base24(scheme_inner) => {
            if scheme_inner.system != system {
                return Err(anyhow!(
                    "Scheme enum variant is mismatched with the provided scheme (\"{}\")",
                    system
                ));
            }

            let template = Template::new(template_content.to_string(), scheme.clone());
            let output = template.render()?;
            let output_path = output_dir.join(format!(
                "{}-{}{}",
                &scheme_inner.system,
                scheme_file_type.get_file_stem().unwrap_or_default(),
                explicit_extension
            ));

            if !output_path.exists() {
                fs::create_dir_all(output_dir)?;
            }

            write_to_file(&output_path, &output)?;
        }
        _ => {
            return Err(anyhow!("Unknown Scheme enum variant"));
        }
    }

    Ok(())
}

fn generate_themes_for_config(
    config_name: &str,
    config_value: &TemplateConfig,
    theme_template_path: &Path,
    schemes_filetypes: &[SchemeFileType],
    is_quiet: bool,
) -> Result<()> {
    // "explicit" extension because it contains the entire extension including (or excluding) the
    // period
    let explicit_extension = config_value.extension.as_str();
    let template_path = theme_template_path.join(format!("templates/{}.mustache", config_name));
    let template_content = read_to_string(&template_path).context(format!(
        "Mustache template missing: {}",
        template_path.display()
    ))?;
    let supported_systems = &config_value
        .supported_systems
        .clone()
        .unwrap_or(vec![SchemeSystem::default()]);
    let output_str = &config_value.output;
    let output_path = if output_str.is_empty() {
        PathBuf::from(theme_template_path)
    } else {
        theme_template_path.join(output_str)
    };

    if output_str.starts_with('/') {
        return Err(anyhow!(
            "`output` value in config.yaml only accepts relative paths: {}",
            output_str
        ));
    }

    if !output_path.exists() {
        create_dir_all(&output_path)?
    }

    let schemes_result_vec: Vec<(PathBuf, Result<Scheme>)> = schemes_filetypes
        .iter()
        .map(|item| (item.get_path().unwrap_or_default(), item.get_scheme()))
        .collect();
    let scheme_err_path_option = schemes_result_vec.iter().find_map(|(path, scheme)| {
        if let Err(err_message) = scheme {
            Some((path, err_message))
        } else {
            None
        }
    });

    if let Some((path, err_message)) = scheme_err_path_option {
        return Err(anyhow!(
            "Unable to deserialize scheme \"{}\": {}",
            path.display(),
            err_message
        ));
    }

    let schemes_vec = schemes_result_vec
        .into_iter()
        .filter_map(|(path, scheme_result)| match &scheme_result {
            Ok(Scheme::Base16(scheme)) | Ok(Scheme::Base24(scheme)) => {
                if supported_systems.contains(&scheme.system) {
                    // This should always unwrap since `Err` variant checking happens in the step
                    // before this
                    Some((path, scheme_result.unwrap()))
                } else {
                    None
                }
            }
            _ => None,
        });

    for (scheme_path, scheme) in schemes_vec {
        let scheme_system = scheme.get_scheme_system();

        generate_theme(
            &template_content,
            &output_path,
            &scheme_path,
            scheme_system,
            explicit_extension,
        )?;
    }

    if !is_quiet {
        println!(
            "Successfully generated \"{}\" themes for \"{}\" at \"{}/*{}\"",
            supported_systems
                .iter()
                .map(|item| item.as_str().to_string())
                .collect::<Vec<String>>()
                .join(", "),
            config_name,
            output_path.display(),
            explicit_extension,
        );
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct TemplateConfig {
    extension: String,
    output: String,
    #[serde(rename = "supported-systems")]
    supported_systems: Option<Vec<SchemeSystem>>,
}

/// Recursively retrieves scheme file paths from a directory.
///
/// This function traverses the given directory recursively, gathering all valid scheme files.
/// It skips hidden files and directories (those whose names start with a `.`).
///
/// # Arguments
///
/// * `dirpath` - A reference to a `Path` representing the directory to start the search from.
///
/// # Returns
///
/// Returns a `Result` containing a `Vec<SchemeFileType>` if successful, where `SchemeFileType`
/// represents a valid scheme file. If any error occurs during directory traversal or file handling,
/// an `Err` with the relevant error information is returned.
///
/// # Errors
///
/// This function can return an error in the following scenarios:
///
/// * If the directory cannot be read.
/// * If there is an issue accessing the contents of the directory.
/// * If there is an issue creating a `SchemeFileType` from a file path.
fn get_recursive_scheme_paths_from_dir(dirpath: &Path) -> Result<Vec<SchemeFileType>> {
    let mut scheme_paths: Vec<SchemeFileType> = vec![];

    for item in dirpath.read_dir()? {
        let file_path = item?.path();
        let file_stem = file_path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        // Skip hidden files and directories
        if file_stem.starts_with('.') {
            continue;
        }

        if file_path.is_dir() {
            let inner_scheme_paths_result = get_recursive_scheme_paths_from_dir(&file_path);

            if let Ok(inner_scheme_paths) = inner_scheme_paths_result {
                scheme_paths.extend(inner_scheme_paths);
            }

            continue;
        }

        let scheme_file_type_result = SchemeFileType::new(&file_path);

        match scheme_file_type_result {
            Ok(scheme_file_type) => {
                scheme_paths.push(scheme_file_type);
            }
            Err(_) => continue,
        }
    }

    Ok(scheme_paths)
}
