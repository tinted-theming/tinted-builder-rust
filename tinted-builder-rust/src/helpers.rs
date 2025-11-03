use anyhow::{Context, Result};
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

#[allow(dead_code)]
pub fn write_to_file(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    if path.as_ref().exists() {
        remove_file(path.as_ref())
            .with_context(|| format!("Unable to remove file: {}", path.as_ref().display()))?;
    }

    let mut file = File::create(path.as_ref())
        .with_context(|| format!("Unable to create file: {}", path.as_ref().display()))?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}
