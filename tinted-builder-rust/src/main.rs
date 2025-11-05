mod cli;
mod operations {
    pub mod build;
    pub mod sync;
}
mod helpers;

use crate::cli::get_matches;
use anyhow::{anyhow, Result};
use std::{borrow, path::PathBuf};

const REPO_NAME: &str = env!("CARGO_PKG_NAME");

/// Expands a leading `~/` to the current user's home directory.
///
/// Returns the original input as a `PathBuf` if not prefixed with `~/`.
fn replace_tilde_slash_with_home(path_str: &str) -> Result<PathBuf> {
    let trimmed_path_str = path_str.trim();
    if trimmed_path_str.starts_with("~/") {
        dirs::home_dir().map_or_else(
            || Err(anyhow!("Unable to determine a home directory for \"{trimmed_path_str}\", please use an absolute path instead")),
            |home_dir|
                Ok(PathBuf::from(trimmed_path_str.replacen(
                    "~/",
                    format!("{}/", home_dir.display()).as_str(),
                    1,
                )))
        )
    } else {
        Ok(PathBuf::from(trimmed_path_str))
    }
}

fn main() -> Result<()> {
    let matches = get_matches();
    let data_path_result: Result<PathBuf> =
        if let Some(data_dir_path) = matches.get_one::<String>("data-dir") {
            replace_tilde_slash_with_home(data_dir_path)
        } else {
            Ok(dirs::data_dir()
                .ok_or_else(|| anyhow!("Error getting data directory"))?
                .join(format!("tinted-theming/{REPO_NAME}")))
        };
    let data_path = data_path_result?;
    let data_schemes_path = data_path.join("schemes");
    let mut is_custom_scheme_path = false;
    let scheme_paths_result: Result<Vec<PathBuf>> =
        if let Some(schemes_dirs) = matches.get_many::<String>("schemes-dir") {
            let mut paths = vec![];

            for dir in schemes_dirs {
                let schemes_path = PathBuf::from(dir);
                if !schemes_path.exists() {
                    return Err(anyhow!("The provided schemes path does not exist: {dir}"));
                }

                paths.push(replace_tilde_slash_with_home(dir)?);
            }

            is_custom_scheme_path = true;

            Ok(paths)
        } else {
            let paths = [
                data_schemes_path.join("base16"),
                data_schemes_path.join("base24"),
                data_schemes_path.join("tinted8"),
            ]
            .iter()
            .filter(|p| p.exists())
            .cloned()
            .collect::<Vec<PathBuf>>();

            Ok(paths)
        };
    let scheme_paths = scheme_paths_result?;

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            if let (Some(template_dir), Some(is_quiet), Some(sync)) = (
                sub_matches.get_one::<String>("template-dir"),
                sub_matches.get_one::<bool>("quiet"),
                sub_matches.get_one::<bool>("sync"),
            ) {
                let template_path = PathBuf::from(template_dir);

                if *sync {
                    if is_custom_scheme_path {
                        return Err(anyhow!("Unable to sync with a custom '--schemes-dir'"));
                    }
                    operations::sync::sync(&data_schemes_path, *is_quiet)?;
                }

                operations::build::build(&template_path, &scheme_paths, *is_quiet)?;
            }
        }
        Some(("sync", sub_matches)) => {
            let is_quiet: bool = sub_matches
                .get_one::<bool>("quiet")
                .is_some_and(borrow::ToOwned::to_owned);

            if is_custom_scheme_path {
                return Err(anyhow!("Unable to sync with a custom '--schemes-dir'"));
            }

            operations::sync::sync(&data_schemes_path, is_quiet)?;
        }
        _ => {
            println!("Basic usage: {REPO_NAME} apply <SCHEME_NAME>");
            println!("For more information try --help");
        }
    }

    Ok(())
}
