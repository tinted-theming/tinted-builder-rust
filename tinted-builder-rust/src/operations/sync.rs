use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use which::which;

const REPO_NAME: &str = env!("CARGO_PKG_NAME");
const SCHEMES_REPO_NAME: &str = "schemes";
const SCHEMES_URL: &str = "https://github.com/tinted-theming/schemes";

/// This function checks if the schemes repository exists at the specified path. If the repository
/// exists and has no uncommitted changes, it performs a `git pull` to update the repository. If
/// the repository contains uncommitted changes, it notifies the user and does not perform the
/// update. If the repository does not exist, it clones the repository from the specified URL.
///
/// This function is typically used in the context of a CLI tool to ensure that the latest schemes
/// are available before performing operations that depend on them.
///
/// # Arguments
///
/// * `schemes_path` - A `impl AsRef<Path>` representing the directory where the schemes
///   repository is or should be located.
/// * `is_quiet` - A boolean flag that, when set to `true`,
///   suppresses most of the output, making the operation quieter.
///
/// # Returns
///
/// Returns a `Result<()>` indicating success (`Ok(())`) or an error (`Err`) if any issues occur
/// during the synchronization process.
///
/// # Errors
///
/// This function can return an error in the following scenarios:
///
/// * If the target directory already exists when attempting to clone the repository. * If there is
///   an issue executing the `git` commands (`clone`, `pull`, or `status`). * If the repository
///   contains uncommitted changes, the function will not perform the update and will notify the
///   user.
///
/// # Usage
///
/// This function is typically called from a CLI context, for example:
///
/// ```sh
/// tinted-builder-rust sync
/// ```
///
/// The function will ensure that the schemes repository is up-to-date, either by pulling the
/// latest changes or by cloning the repository if it does not already exist.
pub fn sync(schemes_path: impl AsRef<Path>, is_quiet: bool) -> Result<()> {
    // Ensure git is installed
    let binary = "git";
    let binary_result = which(binary);
    if binary_result.is_err() {
        return Err(anyhow!("`{binary}` is required for pulling repositories from GitHub. Either install `{binary}` or manually provide the Schemes directory with `--schemes-dir` flag."));
    }

    if schemes_path.as_ref().is_dir() {
        let is_diff = git_diff(&schemes_path)?;

        if !is_diff {
            git_pull(schemes_path, is_quiet)
                .with_context(|| format!("Error pulling {SCHEMES_REPO_NAME} from {SCHEMES_URL}"))?;

            if !is_quiet {
                println!("{SCHEMES_REPO_NAME} up to date");
            }
        } else if !is_quiet {
            println!("{SCHEMES_REPO_NAME} contains uncommitted changes, please commit or remove and then run `{REPO_NAME} update` again.");
        }
    } else {
        git_clone(SCHEMES_URL, schemes_path, is_quiet)?;

        if !is_quiet {
            println!("{SCHEMES_REPO_NAME} installed");
        }
    }

    Ok(())
}

fn git_clone(repo_url: &str, target_dir: impl AsRef<Path>, is_quiet: bool) -> Result<()> {
    if target_dir.as_ref().exists() {
        return Err(anyhow!(
            "Error cloning {}. Target directory '{}' already exists",
            repo_url,
            target_dir.as_ref().display()
        ));
    }

    let mut cmd = Command::new("git");

    cmd.arg("clone").arg(repo_url).arg(target_dir.as_ref());

    if is_quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    cmd.status()
        .with_context(|| format!("Failed to clone repository from {repo_url}"))?;

    Ok(())
}

fn git_pull(repo_path: impl AsRef<Path>, is_quiet: bool) -> Result<()> {
    if !repo_path.as_ref().is_dir() {
        return Err(anyhow!(
            "Error with git pull. {} is not a directory",
            repo_path.as_ref().display()
        ));
    }

    let mut cmd = Command::new("git");

    cmd.arg("pull").current_dir(&repo_path);

    if is_quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let status = cmd.status().with_context(|| {
        format!(
            "Failed to execute process in {}",
            repo_path.as_ref().display()
        )
    })?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Error wth git pull in {}",
            repo_path.as_ref().display()
        ))
    }
}

fn git_diff(target_dir: impl AsRef<Path>) -> Result<bool> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(&target_dir)
        .output()
        .with_context(|| {
            format!(
                "Failed to execute process in {}",
                target_dir.as_ref().display()
            )
        })?;
    let stdout = str::from_utf8(&output.stdout).expect("Not valid UTF-8");

    if stdout.is_empty() {
        Ok(false)
    } else {
        Ok(true)
    }
}
