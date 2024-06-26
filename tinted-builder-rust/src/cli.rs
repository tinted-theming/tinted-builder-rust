use clap::{builder::styling, Arg, ArgAction, ArgMatches, Command};

fn build_cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("data-dir")
                .short('d')
                .help("Path to the data directory")
                .value_name("DIRECTORY")
                .long("data-dir")
                .global(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("schemes-dir")
                .action(ArgAction::Set)
                .global(true)
                .help("Path to the schemes directory")
                .long("schemes-dir")
                .short('s')
                .value_name("DIRECTORY"),
        )
        .subcommand(
            Command::new("build")
                .about("Builds the target theme template")
                .arg(
                    Arg::new("template-dir")
                        .help("Local path to the theme template you want to build")
                        .required(true),
                )
                .arg(
                    Arg::new("quiet")
                        .long("quiet")
                        .short('q')
                        .help("Silence stdout")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("sync")
                        .long("sync")
                        .help("Sync with latest schemes before build")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("sync")
                .about("Clones {} and if it exists it does a git pull on the local clone")
                .arg(
                    Arg::new("quiet")
                        .long("quiet")
                        .short('q')
                        .help("Silence stdout")
                        .action(ArgAction::SetTrue),
                ),
        )
}

pub(crate) fn get_matches() -> ArgMatches {
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());

    build_cli().styles(styles).get_matches()
}
