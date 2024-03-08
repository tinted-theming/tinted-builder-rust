// tinted-builder-rust is a Tinted Theming template builder which uses color
// schemes to generate theme files.
// Copyright (C) 2024  Tinted Theming

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("sync")
                .about("Clones {} and if it exists it does a git pull on the local clone"),
        )
}

pub fn get_matches() -> ArgMatches {
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default());

    build_cli().styles(styles).get_matches()
}
