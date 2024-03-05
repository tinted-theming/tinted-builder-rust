use anyhow::{Context, Result};
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
pub fn write_to_file(path: &Path, contents: &str) -> Result<()> {
    if path.exists() {
        remove_file(path).with_context(|| format!("Unable to remove file: {}", path.display()))?;
    }

    let mut file =
        File::create(path).with_context(|| format!("Unable to create file: {}", path.display()))?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}
