use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;

const REPO_NAME: &str = env!("CARGO_PKG_NAME");
const SCHEMES_REPO_NAME: &str = "schemes";
const SCHEMES_URL: &str = "https://github.com/tinted-theming/schemes";

fn git_clone(repo_url: &str, target_dir: &Path, is_quiet: bool) -> Result<()> {
    if target_dir.exists() {
        return Err(anyhow!(
            "Error cloning {}. Target directory '{}' already exists",
            repo_url,
            target_dir.display()
        ));
    }

    let mut cmd = Command::new("git");

    cmd.arg("clone").arg(repo_url).arg(target_dir);

    if is_quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    };

    cmd.status()
        .with_context(|| format!("Failed to clone repository from {}", repo_url))?;

    Ok(())
}

fn git_pull(repo_path: &Path, is_quiet: bool) -> Result<()> {
    if !repo_path.is_dir() {
        return Err(anyhow!(
            "Error with git pull. {} is not a directory",
            repo_path.display()
        ));
    }

    let mut cmd = Command::new("git");

    cmd.arg("pull").current_dir(repo_path);

    if is_quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute process in {}", repo_path.display()))?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Error wth git pull in {}", repo_path.display()))
    }
}

fn git_diff(target_dir: &Path) -> Result<bool> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(target_dir)
        .output()
        .with_context(|| format!("Failed to execute process in {}", target_dir.display()))?;
    let stdout = str::from_utf8(&output.stdout).expect("Not valid UTF-8");

    if stdout.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Sync schemes repo; Install if it does not exist, otherwise update
pub(crate) fn sync(schemes_path: &Path, is_quiet: bool) -> Result<()> {
    if schemes_path.is_dir() {
        let is_diff = git_diff(schemes_path)?;

        if !is_diff {
            git_pull(schemes_path, is_quiet).with_context(|| {
                format!("Error pulling {} from {}", SCHEMES_REPO_NAME, SCHEMES_URL)
            })?;

            if !is_quiet {
                println!("{} up to date", SCHEMES_REPO_NAME);
            }
        } else if !is_quiet {
            println!("{} contains uncommitted changes, please commit or remove and then run `{} update` again.", SCHEMES_REPO_NAME, REPO_NAME);
        }
    } else {
        git_clone(SCHEMES_URL, schemes_path, is_quiet)?;

        if !is_quiet {
            println!("{} installed", SCHEMES_REPO_NAME);
        }
    }

    Ok(())
}
