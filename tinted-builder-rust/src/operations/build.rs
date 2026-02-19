pub mod utils;

use crate::helpers::write_to_file;
use anyhow::{anyhow, Result};
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::fs::{self, create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use tinted_builder::tinted8::{
    Scheme as Tinted8Scheme, SUPPORTED_BUILDER_SPEC_VERSION, SUPPORTED_STYLING_SPEC_VERSION,
};
use tinted_builder::{Base16Scheme, Scheme, SchemeSystem, Template};
use utils::{get_scheme_files, parse_filename, ParsedFilename, TemplateConfig};

pub use utils::SchemeFile;

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
/// * `theme_template_path` - A `impl AsRef<Path>` representing the path to the theme template
///   directory or file.
/// * `user_schemes_path` - A `impl AsRef<Path>` representing the directory where user schemes are
///   stored.
/// * `is_quiet` - A boolean flag that, when set to `true`, suppresses most of the output,
///   making the build process quieter.
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
#[allow(clippy::too_many_lines)]
pub fn build(
    theme_template_path: impl AsRef<Path>,
    user_scheme_paths: &[impl AsRef<Path>],
    is_quiet: bool,
) -> Result<()> {
    for path in user_scheme_paths {
        if !path.as_ref().exists() {
            return Err(anyhow!(
                "Schemes don't exist locally. First run `{REPO_NAME} sync` and try again",
            ));
        }
    }

    let template_config_path = {
        if theme_template_path
            .as_ref()
            .join("templates/config.yml")
            .is_file()
        {
            theme_template_path.as_ref().join("templates/config.yml")
        } else {
            theme_template_path.as_ref().join("templates/config.yaml")
        }
    };

    if !template_config_path.exists() || !template_config_path.is_file() {
        return Err(anyhow!(
            "E305: Template config missing or invalid: {}",
            template_config_path.display()
        ));
    }

    let template_config_content = read_to_string(&template_config_path).map_err(|_| {
        anyhow!(
            "E305: Template config missing or invalid: {}",
            template_config_path.display()
        )
    })?;
    let template_config: HashMap<String, TemplateConfig> =
        serde_yaml::from_str(&template_config_content).map_err(|_| {
            anyhow!(
                "E305: Template config missing or invalid: {}",
                template_config_path.display()
            )
        })?;

    let scheme_files: Vec<(PathBuf, Result<Scheme>)> = get_scheme_files(user_scheme_paths, true)?
        .iter()
        .map(|item| (item.get_path(), item.get_scheme()))
        .collect();

    let all_scheme_files: Vec<(PathBuf, Scheme)> = scheme_files
        .iter()
        .map(|(path, scheme)| match scheme {
            Ok(scheme) => Ok((path.clone(), scheme.clone())),
            Err(err_message) => Err(anyhow!(
                "Unable to deserialize scheme \"{}\": {}",
                path.display(),
                err_message
            )),
        })
        .collect::<Result<Vec<(PathBuf, Scheme)>>>()?;

    // For each template definition in the templates/config.yaml file
    for (template_item_config_name, template_item_config_value) in &template_config {
        let supported_systems = template_item_config_value
            .supported_systems
            .clone()
            .unwrap_or_else(|| vec![SchemeSystem::default()]);

        if supported_systems.contains(&SchemeSystem::Tinted8) {
            let supports = template_item_config_value.supports.clone().ok_or_else(|| {
                anyhow!("E300: \"tinted8\" scheme system requires config property \"supports\"")
            })?;

            {
                let builder_req_str = supports.get("tinted8-builder").ok_or_else(|| {
                    anyhow!(
                        "E302: \"tinted8\" scheme system requires config property supports.tinted8-builder"
                    )
                })?;
                let builder_req = VersionReq::parse(builder_req_str)?;
                let builder_ver = Version::parse(SUPPORTED_BUILDER_SPEC_VERSION)?;
                if !builder_req.matches(&builder_ver) {
                    return Err(anyhow!(
                        "E003: Tinted8 Builder Spec Incompatible (requires {builder_req}, self v{builder_ver})"
                    ));
                }
                if !is_quiet {
                    println!(
                        "→ tinted8-builder: v{builder_ver} (self-compatible with {builder_req})",
                    );
                }
            }

            {
                let styling_req_str = supports.get("tinted8-styling").ok_or_else(|| {
                    anyhow!(
                        "E301: \"tinted8\" scheme system requires config property supports.tinted8-styling"
                    )
                })?;
                let styling_req = VersionReq::parse(styling_req_str)?;
                let styling_ver = Version::parse(SUPPORTED_STYLING_SPEC_VERSION)?;
                if !styling_req.matches(&styling_ver) {
                    return Err(anyhow!(
                        "E002: Unsupported Tinted8 Styling Spec (requires {styling_req}, supported v{styling_ver})"
                    ));
                }
                if !is_quiet {
                    println!("→ tinted8-styling: v{styling_ver} (supported range {styling_req})",);
                }
            }
        }

        // Render list
        for (template_item_config_name, template_item_config_value) in &template_config {
            if let Some(options) = &template_item_config_value.options {
                if options.get("list").is_some() {
                    render_list(
                        &theme_template_path,
                        &supported_systems,
                        (template_item_config_name, template_item_config_value),
                        &all_scheme_files,
                        is_quiet,
                    )?;

                    return Ok(());
                }
            }
        }

        // If no list exists generate
        let template_item_scheme_files: Vec<(PathBuf, Scheme)> = all_scheme_files
            .iter()
            .filter_map(|(path, scheme)| {
                if supported_systems.contains(&scheme.get_scheme_system()) {
                    Some((path.clone(), scheme.clone()))
                } else {
                    None
                }
            })
            .collect();

        generate_themes_for_config(
            template_item_config_name,
            template_item_config_value,
            &theme_template_path,
            &template_item_scheme_files,
            is_quiet,
        )?;
    }

    Ok(())
}

fn render_list(
    template_path: impl AsRef<Path>,
    supported_systems: &[SchemeSystem],
    (config_name, config_value): (&str, &TemplateConfig),
    all_scheme_files: &[(PathBuf, Scheme)],
    is_quiet: bool,
) -> Result<()> {
    let filename = get_filename(config_value, is_quiet)?;
    let mustache_template_path = template_path
        .as_ref()
        .join(format!("templates/{config_name}.mustache"));
    let template_content = read_to_string(&mustache_template_path).map_err(|_| {
        anyhow!(
            "E303: Mustache template missing: {}",
            mustache_template_path.display()
        )
    })?;

    let data_yaml: &mut String = &mut String::new();

    if supported_systems.contains(&SchemeSystem::Tinted8)
        && (supported_systems.contains(&SchemeSystem::Base16)
            || supported_systems.contains(&SchemeSystem::Base24))
    {
        return Err(anyhow!("Unable to list tinted8 along with base16 or base24 since their structures are different"));
    }

    for scheme_system in supported_systems {
        match &scheme_system {
            SchemeSystem::Base16 | SchemeSystem::Base24 => {
                let mut data: HashMap<&str, Vec<Base16Scheme>> = HashMap::new();

                data.insert(
                    "schemes",
                    all_scheme_files
                        .iter()
                        .cloned()
                        .filter_map(|(_, scheme)| match scheme {
                            Scheme::Base16(scheme) | Scheme::Base24(scheme) => Some(scheme),
                            _ => None,
                        })
                        .collect::<Vec<Base16Scheme>>(),
                );

                *data_yaml = serde_yaml::to_string(&data).unwrap_or_default();
            }
            SchemeSystem::Tinted8 => {
                let mut data: HashMap<&str, Vec<Box<Tinted8Scheme>>> = HashMap::new();

                data.insert(
                    "schemes",
                    all_scheme_files
                        .iter()
                        .cloned()
                        .filter_map(|(_, scheme)| match scheme {
                            Scheme::Tinted8(scheme) => Some(scheme),
                            _ => None,
                        })
                        .collect::<Vec<Box<Tinted8Scheme>>>(),
                );

                *data_yaml = serde_yaml::to_string(&data).unwrap_or_default();
            }

            _ => return Err(anyhow!("E110: Unknown or unsupported scheme system")),
        }
    }

    let supported_systems_str = &supported_systems
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ");
    let output = ribboncurls::render(&template_content, data_yaml, None)?;
    let filepath = filename
        .replace("{{ scheme-system }}", supported_systems_str)
        .replace("{{scheme-system}}", supported_systems_str);

    let parsed_filename = parse_filename(&template_path, &filepath);
    let output_path = parsed_filename.get_path();

    if !parsed_filename.directory.exists() {
        create_dir_all(&parsed_filename.directory)?;
    }

    write_to_file(&output_path, &output)?;

    if !is_quiet {
        println!(
            "✔ Successfully generated \"{}\" list with filename \"{}\"",
            supported_systems_str,
            template_path.as_ref().join(filename).display(),
        );
    }

    Ok(())
}

fn get_filename(config_value: &TemplateConfig, is_quiet: bool) -> Result<String> {
    match (
        &config_value.filename,
        #[allow(deprecated)]
        &config_value.extension,
        #[allow(deprecated)]
        &config_value.output,
    ) {
        (Some(filename), _, _) => Ok(filename.clone()),
        (None, Some(extension), Some(output)) => {
            if !is_quiet {
                println!("Warning: \"extension\" is a deprecated config property, use \"filename\" instead.");
                println!("Warning: \"output\" is a deprecated config property, use \"filename\" instead.");
            }

            Ok(format!(
                "{output}/{{{{ scheme-system }}}}-{{{{ scheme-slug }}}}{extension}",
            ))
        }
        (None, None, Some(output)) => {
            if !is_quiet {
                println!("Warning: \"output\" is a deprecated config property, use \"filename\" instead.");
            }

            Ok(format!(
                "{output}/{{{{ scheme-system }}}}-{{{{ scheme-slug }}}}",
            ))
        }
        (None, Some(extension), None) => {
            if !is_quiet {
                println!("Warning: \"extension\" is a deprecated config property, use \"filename\" instead.");
            }

            Ok(format!(
                "{{{{ scheme-system }}}}-{{{{ scheme-slug }}}}{extension}",
            ))
        }
        _ => Err(anyhow!(
            "E304: Invalid filename configuration: provide \"filename\" or use deprecated \"extension\"/\"output\" combination"
        )),
    }
}

fn generate_themes_for_config(
    config_name: &str,
    config_value: &TemplateConfig,
    theme_template_path: impl AsRef<Path>,
    scheme_files: &Vec<(PathBuf, Scheme)>,
    is_quiet: bool,
) -> Result<()> {
    if scheme_files.is_empty() {
        return Err(anyhow!(
            "E400: No schemes found for a template config entry \"{config_name}\"",
        ));
    }

    let filename = get_filename(config_value, is_quiet)?;
    let mustache_template_path = theme_template_path
        .as_ref()
        .join(format!("templates/{config_name}.mustache"));
    let supported_systems = &config_value
        .supported_systems
        .clone()
        .unwrap_or_else(|| vec![SchemeSystem::default()]);
    let template_content = read_to_string(&mustache_template_path).map_err(|_| {
        anyhow!(
            "E303: Mustache template missing: {}",
            mustache_template_path.display()
        )
    })?;

    // If this config targets tinted8, prepare the styling VersionReq for validation
    let tinted8_styling_req: Option<VersionReq> = config_value
        .supports
        .as_ref()
        .and_then(|m| m.get("tinted8-styling"))
        .and_then(|s| VersionReq::parse(s).ok());

    for (scheme_path, scheme) in scheme_files {
        let (scheme_slug, scheme_system) = match scheme {
            Scheme::Base16(scheme) | Scheme::Base24(scheme) => Ok((&scheme.slug, &scheme.system)),
            Scheme::Tinted8(scheme) => Ok((&scheme.scheme.slug, &scheme.scheme.system)),
            scheme => Err(anyhow!(
                "E110: Unknown or unsupported scheme system: {}",
                scheme.get_scheme_system()
            )),
        }?;

        // Enforce tinted8 styling version compliance if requested by config
        if let (Scheme::Tinted8(s), Some(req)) = (scheme, tinted8_styling_req.clone()) {
            // Print system line (per example output)
            if !is_quiet {
                println!("→ system: {}", s.scheme.system);
            }

            let scheme_styling_version = Version::parse(&s.scheme.supported_styling_version)?;
            if !req.matches(&scheme_styling_version) {
                return Err(anyhow!(
                    "E002: Scheme requires Styling v{scheme_styling_version} but tinted8-builder supports only {req}",
                ));
            }
            if !is_quiet {
                println!("→ tinted8-styling: v{scheme_styling_version} (supported range {req})",);
            }
        }

        // Early system validation (defensive): ensure scheme matches supported systems
        if !supported_systems.contains(scheme_system) {
            return Err(anyhow!("E001: Invalid system"));
        }

        // Replace string variables. Use lazy replace instead of running through mustache template
        // rendering engine for performace
        let filepath = filename
            .replace("{{ scheme-slug }}", &scheme_slug.clone())
            .replace("{{scheme-slug}}", &scheme_slug.clone())
            .replace("{{ scheme-system }}", &scheme_system.to_string())
            .replace("{{scheme-system}}", &scheme_system.to_string());

        let parsed_filename = parse_filename(&theme_template_path, &filepath);
        if !parsed_filename.directory.exists() {
            create_dir_all(&parsed_filename.directory)?;
        }

        generate_theme(
            &template_content,
            parsed_filename,
            scheme_path,
            &scheme_system.clone(),
        )?;
    }

    if !is_quiet {
        println!(
            "✔ Successfully generated \"{}\" themes for \"{}\"",
            supported_systems
                .iter()
                .map(|item| item.as_str().to_string())
                .collect::<Vec<String>>()
                .join(", "),
            config_name,
        );
    }

    Ok(())
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
/// * `scheme_path` - A `impl AsRef<Path>` representing the file path to the scheme file.
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
    parsed_filename: ParsedFilename,
    scheme_path: impl AsRef<Path>,
    system: &SchemeSystem,
) -> Result<()> {
    let scheme_file_type = SchemeFile::new(scheme_path)?;
    let scheme_path = scheme_file_type.get_path();
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
            if scheme_inner.system != *system {
                return Err(anyhow!("E001: Invalid system"));
            }

            let template = Template::new(template_content.to_string(), scheme.clone());
            let output = template.render()?;
            let output_path = parsed_filename.get_path();

            if !parsed_filename.directory.exists() {
                fs::create_dir_all(parsed_filename.directory)?;
            }

            write_to_file(&output_path, &output)?;
        }
        Scheme::Tinted8(scheme_inner) => {
            if scheme_inner.scheme.system != *system {
                return Err(anyhow!("E001: Invalid system"));
            }

            let template = Template::new(template_content.to_string(), scheme.clone());
            let output = template.render()?;
            let output_path = parsed_filename.get_path();

            if !parsed_filename.directory.exists() {
                fs::create_dir_all(parsed_filename.directory)?;
            }

            write_to_file(&output_path, &output)?;
        }
        _ => {
            return Err(anyhow!("Unknown Scheme enum variant"));
        }
    }

    Ok(())
}
