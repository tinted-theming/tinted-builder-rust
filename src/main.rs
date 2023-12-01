mod cli;
mod commands;
mod config;

use crate::commands::load_schemes;
use crate::{cli::build_cli, commands::validate_scheme_from_path};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let cmd = build_cli();
    let matches = cmd.get_matches();
    let schemes_dir = matches
        .get_one::<String>("schemes-dir")
        .expect("Required argument")
        .clone();
    let template_dir = matches
        .get_one::<String>("template-dir")
        .map(|s| s.clone())
        .expect("Default value should be available")
        .clone();
    let schemes_collection = load_schemes(schemes_dir).context("Failed to load schemes")?;

    for scheme in schemes_collection {
        validate_scheme_from_path(scheme.clone())?;
        println!("Scheme directory: {}", scheme.display());
    }

    println!("Templates directory: {}", template_dir);

    Ok(())
}
