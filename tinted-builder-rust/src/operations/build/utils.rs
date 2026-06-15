use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use tinted_builder::{Scheme, SchemeSystem};
use wax::{Glob, Program};

/// Represents a path to a scheme file with a supported extension.
#[derive(Debug, Clone)]
pub enum SchemeFile {
    Yaml(PathBuf),
    Yml(PathBuf),
}

impl SchemeFile {
    /// Creates a new [`SchemeFile`] from the given path.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided file does not have a supported extension (`.yaml`/`.yml`).
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
            _ => Err(anyhow!(
                "E111: Invalid scheme file extension: {}",
                path.as_ref().display()
            )),
        }
    }

    /// Reads and parses the YAML scheme file into a [`Scheme`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read from disk
    /// - The contents are not valid YAML
    /// - The YAML structure does not match a supported scheme system
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
                        Some(_) => {
                            let scheme_inner =
                                serde_yaml::from_value(serde_yaml::Value::Mapping(map))?;
                            let scheme = Scheme::Base16(scheme_inner);

                            Ok(scheme)
                        }
                        None => {
                            if let Some(scheme_meta) = map.get("scheme") {
                                if let Some(system) = scheme_meta.get("system") {
                                    if system == &SchemeSystem::Tinted8.to_string() {
                                        let scheme_inner = serde_yaml::from_value(
                                            serde_yaml::Value::Mapping(map),
                                        )?;
                                        let scheme = Scheme::Tinted8(scheme_inner);

                                        Ok(scheme)
                                    } else {
                                        Err(anyhow!("E110: Unknown or unsupported scheme system"))
                                    }
                                } else {
                                    Err(anyhow!("E111: Missing required field `scheme.system`"))
                                }
                            } else {
                                Err(anyhow!("E111: Missing required field `system`"))
                            }
                        }
                    }
                } else {
                    Err(anyhow!("E112: Unable to parse scheme file"))
                }
            }
        }
    }

    /// Returns the underlying path to the scheme file.
    #[must_use]
    pub fn get_path(&self) -> PathBuf {
        match self {
            Self::Yaml(path) | Self::Yml(path) => path.clone(),
        }
    }
}

/// Template configuration for a single output target.
#[derive(Debug, Deserialize)]
pub struct TemplateConfig {
    pub filename: Option<String>,

    #[serde(rename = "supported-systems")]
    pub supported_systems: Option<Vec<SchemeSystem>>,

    pub supports: Option<HashMap<String, String>>,

    pub options: Option<HashMap<String, String>>,

    #[deprecated]
    pub extension: Option<String>,

    #[deprecated]
    pub output: Option<String>,
}

/// Parsed components of a generated output filename.
#[derive(Debug)]
pub struct ParsedFilename {
    pub directory: PathBuf,
    pub filestem: String,
    pub file_extension: Option<String>,
}

impl ParsedFilename {
    /// Returns the full path for this parsed filename.
    #[must_use]
    pub fn get_path(&self) -> PathBuf {
        let directory = &self.directory;
        let filestem = &self.filestem;
        let file_extension = &self
            .file_extension
            .as_ref()
            .map(|ext| format!(".{ext}"))
            .unwrap_or_default();

        directory.join(format!("{filestem}{file_extension}"))
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
/// Strictness depends on directory layout: a file directly inside a scheme-system directory
/// (`base16`/`base24`/`tinted8`) must be a valid scheme file, while unrecognized files elsewhere
/// (e.g. at the root) are treated as non-schemes and skipped.
///
/// # Errors
///
/// This function can return an error in the following scenarios:
///
/// * If the directory cannot be read.
/// * If there is an issue accessing the contents of the directory.
/// * If an unrecognized file is found inside a scheme-system directory.
pub fn get_scheme_files(
    dirpath: impl AsRef<Path>,
    ignores: &[String],
    is_recursive: bool,
) -> Result<Vec<SchemeFile>> {
    let glob_ignores: Vec<Glob> = ignores
        .iter()
        .map(|s| Glob::new(s))
        .collect::<Result<_, _>>()?;

    // Strictness is keyed off directory layout: files directly inside a scheme-system
    // directory (`base16`/`base24`/`tinted8`) must be valid scheme files, while at the
    // root (or any non-system directory) unrecognized files are simply not schemes and
    // are skipped. This lets a schemes repo carry `LICENSE`, `README.md`, etc. without
    // breaking discovery, while still surfacing genuinely misplaced files.
    let strict = dir_is_scheme_system(dirpath.as_ref());

    collect_scheme_files(dirpath, &glob_ignores, is_recursive, strict)
}

/// Returns `true` when the directory's own name is a scheme system (`base16`/`base24`/`tinted8`).
fn dir_is_scheme_system(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.parse::<SchemeSystem>().is_ok())
}

/// Recursively collects scheme files. When `strict` is set (i.e. we are inside a scheme-system
/// directory) an unrecognized file extension surfaces as an error; otherwise it is skipped.
fn collect_scheme_files(
    dirpath: impl AsRef<Path>,
    glob_ignores: &[Glob],
    is_recursive: bool,
    strict: bool,
) -> Result<Vec<SchemeFile>> {
    let mut scheme_paths: Vec<SchemeFile> = vec![];

    for item in dirpath.as_ref().read_dir()? {
        let file_path = item?.path();

        // Skip hidden files and directories (e.g. `.git`, `.github`, `.yamllint.yml`)
        // as well as any caller-provided ignore globs.
        let is_hidden = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with('.'));
        if is_hidden || glob_ignores.iter().any(|g| g.is_match(file_path.as_path())) {
            continue;
        }

        if file_path.is_dir() && is_recursive {
            let child_strict = strict || dir_is_scheme_system(&file_path);
            scheme_paths.extend(collect_scheme_files(
                &file_path,
                glob_ignores,
                true,
                child_strict,
            )?);

            continue;
        }

        // Only attempt to create a SchemeFile for regular files
        if file_path.is_file() {
            match SchemeFile::new(&file_path) {
                Ok(scheme_file_type) => scheme_paths.push(scheme_file_type),
                // Inside a scheme-system directory, be strict: surface invalid files as
                // intake errors. Elsewhere, the file simply isn't a scheme - skip it.
                Err(err) if strict => return Err(err),
                Err(_) => {}
            }
        }
    }

    scheme_paths.sort_by_key(SchemeFile::get_path);

    Ok(scheme_paths)
}

/// Collects scheme files from a `<system>/`-organized schemes directory.
///
/// Walks the `base16`/`base24`/`tinted8` subdirectories (the layout of the
/// `tinted-theming/schemes` repository) and returns a map keyed by `<system>-<file-stem>`
/// (e.g. `base16-github`). Discovery is strict within each scheme-system directory, so an
/// unrecognized file there surfaces an error; files at the schemes-directory root are ignored.
///
/// When `scheme_system` is `Some`, only that system's subdirectory is walked. Missing
/// subdirectories are skipped rather than treated as errors.
///
/// # Errors
///
/// Returns an error if a scheme-system subdirectory cannot be read, or if it contains a file
/// with an unrecognized extension.
// Public library API consumed by downstreams (e.g. tinty); unused by this crate's own binary.
#[allow(dead_code)]
pub fn get_scheme_files_by_name(
    schemes_path: impl AsRef<Path>,
    scheme_system: Option<SchemeSystem>,
) -> Result<HashMap<String, SchemeFile>> {
    let scheme_systems =
        scheme_system.map_or_else(|| SchemeSystem::variants().to_vec(), |system| vec![system]);

    let mut scheme_files: HashMap<String, SchemeFile> = HashMap::new();
    for scheme_system in scheme_systems {
        let scheme_system_dir = schemes_path.as_ref().join(scheme_system.as_str());
        if !scheme_system_dir.is_dir() {
            continue;
        }

        // The directory name is a scheme system, so `get_scheme_files` walks it strictly.
        for scheme_file in get_scheme_files(&scheme_system_dir, &[], false)? {
            if let Some(stem) = scheme_file.get_path().file_stem().and_then(|s| s.to_str()) {
                scheme_files.insert(format!("{scheme_system}-{stem}"), scheme_file);
            }
        }
    }

    Ok(scheme_files)
}

/// Parses a given file path into its directory, filestem, and optional extension.
///
/// This function takes a `template_path` (which is used as the base path for relative directories)
/// and a `filepath` (the path to parse). It returns a `ParsedFilename` struct, which contains:
/// - `directory`: the directory of the file (relative to `template_path` or `.` if not present)
/// - `filestem`: the filename without the extension
/// - `file_extension`: the optional file extension
pub fn parse_filename(template_path: impl AsRef<Path>, filepath: &str) -> ParsedFilename {
    let p = Path::new(filepath);

    let directory: PathBuf = p.parent().map_or_else(
        || template_path.as_ref().to_path_buf(),
        |dir| template_path.as_ref().join(dir),
    );

    // A filestem must exist and be non-empty.
    let filestem = p
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .unwrap_or_default();

    let file_extension = p.extension().and_then(|e| e.to_str()).map(String::from);

    ParsedFilename {
        directory,
        filestem,
        file_extension,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_parse_filename_with_directory_and_extension() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "some-directory/name/file.txt");

        assert_eq!(result.directory, template_path.join("some-directory/name"));
        assert_eq!(result.filestem, "file");
        assert_eq!(result.file_extension, Some("txt".to_string()));
    }

    #[test]
    fn test_parse_filename_with_filename_and_extension() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "filename.ext");

        assert_eq!(result.directory, template_path);
        assert_eq!(result.filestem, "filename");
        assert_eq!(result.file_extension, Some("ext".to_string()));
    }

    #[test]
    fn test_parse_filename_with_only_filename() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "file");

        assert_eq!(result.directory, template_path);
        assert_eq!(result.filestem, "file");
        assert_eq!(result.file_extension, None);
    }

    #[test]
    fn test_parse_filename_with_directory_and_no_extension() {
        let template_path = Path::new("/home/user/templates");
        let result = parse_filename(template_path, "some-directory/file");

        assert_eq!(result.directory, template_path.join("some-directory"));
        assert_eq!(result.filestem, "file");
        assert_eq!(result.file_extension, None);
    }
}
