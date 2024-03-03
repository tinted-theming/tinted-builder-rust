mod cli;
// mod commands;
mod constants;
mod operations;
mod utils;
// mod scheme;

// use std::fs;

// use crate::cli::build_cli;
// use crate::commands::{
//     create_colorscheme_from_scheme, get_scheme_from_path, load_scheme_paths, Scheme,
// };
use anyhow::{anyhow, Context, Result};
use base16_color_scheme::{Scheme, Template};
use cli::get_matches;
use constants::{REPO_NAME, SCHEMES_REPO_NAME};
use utils::replace_tilde_slash_with_home;
// use crate::scheme::ColorScheme;
use std::{
    fs::{self, read_to_string},
    path::PathBuf,
    str::FromStr,
};
// use commands::create_expanded_scheme;
// use std::path::PathBuf;

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

    // let data_path_result: Result<PathBuf> =
    //     if let Some(schemes_dir) = matches.get_one::<String>("") {
    //         let schemes_path = PathBuf::from(schemes_dir);
    //         if !schemes_path.exists() {
    //             anyhow::bail!("The provided schemes path does not exist: {}", schemes_dir);
    //         }

    //         replace_tilde_slash_with_home(schemes_dir)
    //     } else {

    //         let data_path = dirs::data_dir()
    //             .ok_or_else(|| anyhow!("Error getting data directory"))?
    //             .join(format!("tinted-theming/{}", REPO_NAME));

    //         if !data_path.exists() {
    //             fs::create_dir_all(&data_path)
    //                 .with_context(|| format!("Failed to create directory at {}", data_path.display()))?;
    //         }

    //         Ok(data_path)
    //     };
    // let data_path = data_path_result?;
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

    // let cmd = build_cli();
    // let matches = cmd.get_matches();
    // let schemes_dir = matches
    //     .get_one::<String>("schemes-dir")
    //     .expect("Required argument")
    //     .clone();
    // let template_dir = matches
    //     .get_one::<String>("template-dir")
    //     .map(|s| s.clone())
    //     .expect("Default value should be available")
    //     .clone();
    // let schemes_collection = load_scheme_paths(schemes_dir).context("Failed to load schemes")?;
    // let mut schemes: Vec<Scheme> = Vec::new();
    // let template_mustache_path = PathBuf::from(template_dir).join("templates/default.mustache");
    // let template_source: String = fs::read_to_string(template_mustache_path).map_err(|err| {
    //     anyhow::Error::new(err).context(format!("Failed to load template from provided path"))
    // })?;

    // for scheme_path in schemes_collection {
    //     schemes.push(get_scheme_from_path(scheme_path.clone())?);
    //     // let some_str = create_content_from_template(scheme_path.clone(), template)?;
    //     // println!("Scheme directory: {}", some_str);
    // }

    // for scheme in &schemes {
    //     // println!("{}", scheme.author);
    //     let content = create_colorscheme_from_scheme(&scheme, &template_source)
    //         .context("Failed to generate colorscheme from scheme")?;
    //     println!("{}", content);

    //     let sm = create_expanded_scheme(scheme)?;
    //     println!(" something {}", sm.0);
    // }

    // println!("Templates directory: {}", template_dir);

    // let template_str = read_to_string("path/to/template.mustache").unwrap();
    // let template = Template::new(template_str).unwrap();

    // let scheme_str = read_to_string("./ayu-dark.yaml").unwrap();
    // let scheme: ColorScheme = serde_yaml::from_str(&scheme_str).unwrap();
    // print!("{}", scheme);

    // use base16_color_scheme::{Scheme, Template};

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            if let Some(template_dir) = sub_matches.get_one::<String>("template_dir") {
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
