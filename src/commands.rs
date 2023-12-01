use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str as yaml_from_str;
use std::fs;
use std::path::PathBuf;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Scheme {
    scheme: String,
    author: String,
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    base0A: String,
    base0B: String,
    base0C: String,
    base0D: String,
    base0E: String,
    base0F: String,
}

/// Loads the file paths of YAML schemes from a specified directory.
///
/// This function reads all files in the given directory and filters out
/// those that have a `.yaml` extension. It is used to collect a list of
/// YAML scheme files for further processing.
///
/// # Arguments
///
/// * `schemes_path` - A `String` specifying the path to the directory containing scheme files.
///
/// # Returns
///
/// A `Result` wrapping a vector of `PathBuf`, each representing a path to a `.yaml` file.
/// Returns an error if the directory cannot be read or if other IO errors occur.
///
/// # Examples
///
/// ```
/// // Assuming there are .yaml files in "./schemes" directory
/// let scheme_paths = load_schemes("./schemes".to_string())?;
/// for path in scheme_paths {
///     println!("Scheme file path: {:?}", path);
/// }
/// ```
pub fn load_schemes(schemes_path: String) -> Result<Vec<PathBuf>> {
    let paths = fs::read_dir(&schemes_path)
        .with_context(|| format!("Failed to read directory: {:?}", &schemes_path))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            // Only collect yaml files
            if let Some(ext) = path.extension() {
                if ext == "yaml" {
                    return Some(path);
                }
            }

            return None;
        })
        .collect();

    Ok(paths)
}

/// Validates if a file at a given path is a correctly formatted YAML scheme.
///
/// Reads the contents of the file at the given path and attempts to deserialize
/// it into a `Scheme` struct. It is used to validate the format and fields of a
/// scheme file.
///
/// # Arguments
///
/// * `scheme_path` - A `PathBuf` pointing to the YAML file to be validated.
///
/// # Returns
///
/// A `Result` wrapping a boolean value. Returns `true` if the file is a valid YAML
/// scheme, `false` otherwise. Returns an error if the file cannot be read or
/// deserialized properly.
///
/// # Examples
///
/// ```
/// let is_valid = validate_scheme_from_path(PathBuf::from("./schemes/example.yaml"))?;
/// assert!(is_valid, "The scheme should be valid");
/// ```
pub fn validate_scheme_from_path(scheme_path: PathBuf) -> Result<bool> {
    let content: String = fs::read_to_string(&scheme_path).unwrap();
    let is_ok = yaml_from_str::<Scheme>(&content)
        .map_err(|err| {
            anyhow::Error::new(err).context(format!(
                "Failed to deserialize YAML from {:?}",
                scheme_path.display()
            ))
        })
        .is_ok();

    Ok(is_ok)
}
