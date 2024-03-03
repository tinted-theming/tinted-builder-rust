use std::path::PathBuf;

use anyhow::{anyhow, Result};

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
