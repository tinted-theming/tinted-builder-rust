use std::fs::{self, remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use std::{error::Error, path::Path, process::Command};

use anyhow::{Context, Result};

#[allow(dead_code, clippy::missing_panics_doc, clippy::missing_errors_doc)]
pub fn run_command(command_vec: &[String]) -> Result<(String, String), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_tinted-builder-rust"))
        .args(command_vec)
        .output()
        .expect("Failed to execute command");

    if !output.stderr.is_empty() {
        println!(
            "Init command stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = strip_ansi_escapes::strip(String::from_utf8(output.stdout)?);
    let stderr = strip_ansi_escapes::strip(String::from_utf8(output.stderr)?);

    Ok((String::from_utf8(stdout)?, String::from_utf8(stderr)?))
}

#[allow(dead_code, clippy::missing_errors_doc)]
pub fn write_to_file(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    if path.as_ref().exists() {
        remove_file(&path)
            .with_context(|| format!("Unable to remove file: {}", path.as_ref().display()))?;
    }

    let mut file = File::create(&path)
        .with_context(|| format!("Unable to create file: {}", path.as_ref().display()))?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}

#[allow(dead_code, clippy::missing_errors_doc)]
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.as_ref().join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), &dest_path)?;
        }
    }
    Ok(())
}

#[allow(dead_code, clippy::missing_errors_doc)]
pub fn unique_tmp_dir(sub: &str) -> Result<PathBuf> {
    let p = std::env::var_os("CARGO_TARGET_TMPDIR").map_or_else(std::env::temp_dir, PathBuf::from);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos();

    Ok(p.join(format!("tinted_builder_rust_{sub}_{nanos}")))
}
