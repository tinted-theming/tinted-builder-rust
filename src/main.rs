mod cli;
mod config;

use crate::cli::build_cli;
use anyhow::Result;

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

    println!("{}", schemes_dir);
    println!("{}", template_dir);

    Ok(())
}
