use crate::config::{DEFAULT_SCHEMES_VALUE, DEFAULT_TEMPLATE_VALUE};
use clap::{Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    Command::new("base16-builder-rust")
        .version("1.0.0")
        .author("Tinted Theming")
        .about("A base16 builder tool to generate template colorschemes")
        .arg(
            Arg::new("schemes-dir")
                .long("schemes-dir")
                .action(ArgAction::Set)
                .value_name("FILE")
                .help("Path to base16-schemes directory")
                .default_value(DEFAULT_SCHEMES_VALUE),
        )
        .arg(
            Arg::new("template-dir")
                .long("template-dir")
                .action(ArgAction::Set)
                .value_name("FILE")
                .help("Path to the base16 template directory")
                .default_value(DEFAULT_TEMPLATE_VALUE),
        )
}
