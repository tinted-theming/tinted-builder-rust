# tinted-builder-rust

[![Matrix Chat](https://img.shields.io/matrix/tinted-theming:matrix.org)](https://matrix.to/#/#tinted-theming:matrix.org)
[![Crates.io](https://img.shields.io/crates/v/tinted-builder-rust.svg)](https://crates.io/crates/tinted-builder-rust)
[![Tests](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml)

A builder for [base16] and [base24] templates using the `0.11.1`
[builder specification].

This crate contains a command-line tool to build base16 and base24
templates. It is also a library crate which you can use to directly
build templates within your own Rust application.

## Table of Contents

- [Installation](#installation)
- [Basic usage](#basic-usage)
- [Commands](#commands)
- [Flags](#flags)
- [Builder specification](#builder-specification)
- [Contributing](#contributing)
- [License](#license)

## Installation

**Cargo**

```sh
cargo install tinted-builder-rust
```

**Homebrew**

```sh
brew tap tinted-theming/tinted
brew install tinted-builder-rust
```

**Binaries**

Download the relevant binary from the [repository releases] page.

## Basic Usage

```sh
tinted-builder-rust sync # To sync with latest schemes
tinted-builder-rust build path/to/base16-template
```

## Commands

The following is a table of the available subcommands for the CLI tool (tinted-builder-rust), including the descriptions and any notable arguments.

| Subcommand | Description                          | Arguments            | Example Usage                              | Flags |
|------------|--------------------------------------|----------------------|--------------------------------------------|-------|
| `sync`  | Installs and or updates latest schemes. | - | `tinted-builder-rust sync` | `--quiet` (silence stderr and stdout) |
| `build` | Builds the themes of a template. | `template_path`: Path to template directory. | `tinted-builder-rust build ./path/to/base16-template` | `--quiet` (silence stderr and stdout), `--sync` (equivalent of running `tinted-builder-rust sync` before `tinted-builder-rust build`) |

## Flags

| Flag/Option       | Description                             | Applicable Subcommands | Default Value | Example Usage                             |
|-------------------|-----------------------------------------|------------------------|---------------|-------------------------------------------|
| `--schemes-dir` `-s`   | Path to a custom local schemes directory to use when building. Only necessary if the [latest schemes repository] is not desired. | `build` | `tinted-builder-rust build . --schemes-dir=/path/to/schemes/dir` |
| `--data-dir` `-d`   | Specifies a custom path for the data directory. | All | Linux: `$XDG_DATA_HOME/tinted-theming/tinted-builder-rust` or `~/.local/share`. MacOS: `~/Library/Application\ Support/tinted-theming/tinted-builder-rust` | `tinted-builder-rust sync --data-dir /path/to/custom/data-dir` |
| `--help` `-h`     | Displays help information for the subcommand. | All | - | `tinted-builder-rust --help`, `tinted-builder-rust build --help`, etc |
| `--version` `-V`  | Shows the version of tinted-builder-rust. | All | - | `tinted-builder-rust --version` |

## Builder specification

tinted-builder-rust implements the `0.11.1` [builder specification]. This
specification details the scheme yaml format or schema as well as the
variables the builder should provide when rendering template mustache
file. Have a look at the [builder specification] document for more
details.

## Contributing

Contributions are welcome! Have a look at [CONTRIBUTING.md] for more
information.

## License

Ribboncurls is dual-licensed under the [Apache 2.0] and [MIT] licenses.
For more information about the licenses of the projects used by
Ribboncurls, have a look at [LICENSES-THIRD-PARTY.md].

[latest schemes repository]: https://github.com/tinted-theming/schemes
[home repository]: https://github.com/tinted-theming/home
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[base16]: https://github.com/tinted-theming/home/blob/main/styling.md
[base24]: https://github.com/tinted-theming/base24/blob/master/styling.md
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[LICENSE]: LICENSE
[LICENSES-THIRD-PARTY.md]: LICENSES-THIRD-PARTY.md
[CONTRIBUTING.md]: CONTRIBUTING.md
[repository releases]: https://github.com/tinted-theming/tinted-builder-rust/releases/latest
