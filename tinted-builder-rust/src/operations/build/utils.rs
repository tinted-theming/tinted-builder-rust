use anyhow::{anyhow, Result};
use regex::Regex;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use tinted_builder::{Scheme, SchemeSystem};

#[derive(Debug, Clone)]
pub enum SchemeFile {
    Yaml(PathBuf),
    Yml(PathBuf),
}

impl SchemeFile {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let extension = path
            .as_ref()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        match extension {
            "yaml" => Ok(Self::Yaml(path.as_ref().to_path_buf())),
            "yml" => Ok(Self::Yml(path.as_ref().to_path_buf())),
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
                    Err(anyhow!("Unable to get scheme from SchemeFile"))
                }
            }
        }
    }

    pub fn get_path(&self) -> Option<PathBuf> {
        match self {
            Self::Yaml(path) | Self::Yml(path) => Some(path.to_path_buf()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct TemplateConfig {
    pub filename: Option<String>,

    #[serde(rename = "supported-systems")]
    pub supported_systems: Option<Vec<SchemeSystem>>,

    #[deprecated]
    pub extension: Option<String>,

    #[deprecated]
    pub output: Option<String>,
}

#[derive(Debug)]
pub(crate) struct ParsedFilename {
    pub directory: PathBuf,
    pub filestem: String,
    pub file_extension: Option<String>,
}

impl ParsedFilename {
    pub fn get_path(&self) -> PathBuf {
        let directory = &self.directory;
        let filestem = &self.filestem;
        let file_extension = &self
            .file_extension
            .as_ref()
            .map(|ext| format!(".{}", ext))
            .unwrap_or_default();

        directory.join(format!("{}{}", filestem, file_extension))
    }
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
/// Returns a `Result` containing a `Vec<SchemeFile>` if successful, where `SchemeFile`
/// represents a valid scheme file. If any error occurs during directory traversal or file handling,
/// an `Err` with the relevant error information is returned.
///
/// # Errors
///
/// This function can return an error in the following scenarios:
///
/// * If the directory cannot be read.
/// * If there is an issue accessing the contents of the directory.
/// * If there is an issue creating a `SchemeFile` from a file path.
pub fn get_scheme_files(dirpath: impl AsRef<Path>, is_recursive: bool) -> Result<Vec<SchemeFile>> {
    let mut scheme_paths: Vec<SchemeFile> = vec![];

    for item in dirpath.as_ref().read_dir()? {
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

        if file_path.is_dir() && is_recursive {
            let inner_scheme_paths_result = get_scheme_files(&file_path, true);

            if let Ok(inner_scheme_paths) = inner_scheme_paths_result {
                scheme_paths.extend(inner_scheme_paths);
            }

            continue;
        }

        let scheme_file_type_result = SchemeFile::new(&file_path);

        match scheme_file_type_result {
            Ok(scheme_file_type) => {
                scheme_paths.push(scheme_file_type);
            }
            Err(_) => continue,
        }
    }

    scheme_paths.sort_by_key(|k| k.get_path());

    Ok(scheme_paths)
}

/// Parses a given file path into its directory, filestem, and optional extension.
///
/// This function takes a `template_path` (which is used as the base path for relative directories)
/// and a `filepath` (the path to parse). It returns a `ParsedFilename` struct, which contains:
/// - `directory`: the directory of the file (relative to `template_path` or `.` if not present)
/// - `filestem`: the filename without the extension
/// - `file_extension`: the optional file extension
pub(crate) fn parse_filename(
    template_path: impl AsRef<Path>,
    filepath: &str,
) -> Result<ParsedFilename> {
    let re = Regex::new(r"^(?P<directory>.*/)?(?P<filestem>[^/\.]+)(?:\.(?P<extension>[^/]+))?$")
        .unwrap();

    if let Some(captures) = re.captures(filepath) {
        // Extract the directory (if present), or use "." if there's no directory
        let directory = captures
            .name("directory")
            .map(|d| template_path.as_ref().join(d.as_str()))
            .unwrap_or_else(|| template_path.as_ref().to_path_buf());
        let filestem = captures.name("filestem").unwrap().as_str().to_string();
        let file_extension = captures
            .name("extension")
            .map(|ext| ext.as_str().to_string());

        if filestem.is_empty() {
            Err(anyhow!(
                "Config property \"filename\" requires a filestem: {}",
                &filepath
            ))
        } else {
            // Return the parsed path
            Ok(ParsedFilename {
                directory,
                filestem,
                file_extension,
            })
        }
    } else {
        Err(anyhow!("Unable to parse template: {}", &filepath))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_filename_with_directory_and_extension() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "some-directory/name/file.txt").unwrap();

        assert_eq!(result.directory, template_path.join("some-directory/name"));
        assert_eq!(result.filestem, "file");
        assert_eq!(result.file_extension, Some("txt".to_string()));
    }

    #[test]
    fn test_parse_filename_with_filename_and_extension() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "filename.ext").unwrap();

        assert_eq!(result.directory, template_path);
        assert_eq!(result.filestem, "filename");
        assert_eq!(result.file_extension, Some("ext".to_string()));
    }

    #[test]
    fn test_parse_filename_with_only_filename() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "file").unwrap();

        assert_eq!(result.directory, template_path);
        assert_eq!(result.filestem, "file");
        assert_eq!(result.file_extension, None);
    }

    #[test]
    fn test_parse_filename_with_directory_and_no_extension() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "some-directory/file").unwrap();

        assert_eq!(result.directory, template_path.join("some-directory"));
        assert_eq!(result.filestem, "file");
        assert_eq!(result.file_extension, None);
    }

    #[test]
    fn test_parse_filename_invalid_filestem() {
        let template_path = Path::new("/home/user/templates");
        let filename = "/invalid/path/";
        let err_message = parse_filename(template_path, filename)
            .unwrap_err()
            .to_string();

        assert!(
            err_message.contains(format!("Unable to parse template: {}", &filename).as_str()),
            "Unexpected error message: {}",
            err_message
        );
    }
}
