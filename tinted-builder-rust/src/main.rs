mod cli;
mod operations {
    pub(crate) mod build;
    pub(crate) mod sync;
}
mod helpers;

use anyhow::{anyhow, Result};
use std::path::PathBuf;

use crate::cli::get_matches;

const REPO_NAME: &str = env!("CARGO_PKG_NAME");

fn replace_tilde_slash_with_home(path_str: &str) -> Result<PathBuf> {
    let trimmed_path_str = path_str.trim();
    if trimmed_path_str.starts_with("~/") {
        match dirs::home_dir() {
            Some(home_dir) => Ok(PathBuf::from(trimmed_path_str.replacen(
                "~/",
                format!("{}/", home_dir.display()).as_str(),
                1,
            ))),
            None => Err(anyhow!("Unable to determine a home directory for \"{}\", please use an absolute path instead", trimmed_path_str))
        }
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
                .join(format!("tinted-theming/{}", REPO_NAME)))
        };
    let data_path = data_path_result?;
    let schemes_path_result: Result<PathBuf> =
        if let Some(schemes_dir) = matches.get_one::<String>("schemes-dir") {
            let schemes_path = PathBuf::from(schemes_dir);
            if !schemes_path.exists() {
                anyhow::bail!("The provided schemes path does not exist: {}", schemes_dir);
            }

            replace_tilde_slash_with_home(schemes_dir)
        } else {
            Ok(data_path.join("schemes"))
        };
    let schemes_path = schemes_path_result?;

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            if let (Some(template_dir), Some(is_quiet), Some(sync)) = (
                sub_matches.get_one::<String>("template-dir"),
                sub_matches.get_one::<bool>("quiet"),
                sub_matches.get_one::<bool>("sync"),
            ) {
                let template_path = PathBuf::from(template_dir);

                if *sync {
                    operations::sync::sync(&schemes_path, *is_quiet)?;
                }

                operations::build::build(&template_path, &schemes_path, *is_quiet)?;
            }
        }
        Some(("sync", sub_matches)) => {
            let is_quiet: bool = sub_matches
                .get_one::<bool>("quiet")
                .map(|b| b.to_owned())
                .unwrap_or(false);
            operations::sync::sync(&schemes_path, is_quiet)?;
        }
        _ => {
            println!("Basic usage: {} apply <SCHEME_NAME>", REPO_NAME);
            println!("For more information try --help");
        }
    };

    Ok(())
}
