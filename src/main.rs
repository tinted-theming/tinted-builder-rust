// tinted-builder-rust is a Tinted Theming template builder which uses color
// schemes to generate theme files.
// Copyright (C) 2024  Tinted Theming

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod cli;
mod operations;
mod utils;

use anyhow::{anyhow, Result};
use std::path::PathBuf;

use crate::cli::get_matches;

const REPO_NAME: &str = env!("CARGO_PKG_NAME");

pub fn replace_tilde_slash_with_home(path_str: &str) -> Result<PathBuf> {
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
            if let Some(template_dir) = sub_matches.get_one::<String>("template-dir") {
                let template_path = PathBuf::from(template_dir);

                operations::build::build(&template_path, &schemes_path)?;
            } else {
                return Err(anyhow!("scheme_name is required for apply command"));
            }
        }
        Some(("sync", _)) => {
            operations::sync::sync(&schemes_path)?;
        }
        _ => {
            println!("Basic usage: {} apply <SCHEME_NAME>", REPO_NAME);
            println!("For more information try --help");
        }
    };

    Ok(())
}
