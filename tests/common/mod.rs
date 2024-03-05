use std::io::Write;
use std::{
    error::Error,
    fs::{remove_file, File},
    path::Path,
    process::Command,
};

use anyhow::{Context, Result};

// pub const REPO_NAME: &str = env!("CARGO_PKG_NAME");
pub const COMMAND_NAME: &str = "./target/release/builder-rust";

pub fn run_command(command_vec: Vec<String>) -> Result<(String, String), Box<dyn Error>> {
    println!("{:?}", command_vec);
    let output = Command::new(&command_vec[0])
        .args(&command_vec[1..])
        .output()
        .expect("Failed to execute command");

    println!("1");

    if !output.stderr.is_empty() {
        println!(
            "Init command stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    println!("2");

    let stdout = strip_ansi_escapes::strip(String::from_utf8(output.stdout)?);
    let stderr = strip_ansi_escapes::strip(String::from_utf8(output.stderr)?);

    Ok((String::from_utf8(stdout)?, String::from_utf8(stderr)?))
}

pub fn write_to_file(path: &Path, contents: &str) -> Result<()> {
    if path.exists() {
        remove_file(path).with_context(|| format!("Unable to remove file: {}", path.display()))?;
    }

    let mut file =
        File::create(path).with_context(|| format!("Unable to create file: {}", path.display()))?;

    file.write_all(contents.as_bytes())?;

    Ok(())
}
