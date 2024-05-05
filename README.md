# tinted-builder-rust

[![Matrix Chat](https://img.shields.io/matrix/tinted-theming:matrix.org)](https://matrix.to/#/#tinted-theming:matrix.org)
[![Crates.io](https://img.shields.io/crates/v/tinted-builder-rust.svg)](https://crates.io/crates/tinted-builder-rust)
[![Tests](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/tests.yml/badge.svg)](https://github.com/tinted-theming/tinty/actions/workflows/tests.yml)

A builder for [base16] and [base24] templates using the `0.11.0` [builder
specification].

This repo contains a command-line tool, [tinted-builder-rust], to build
base16 and base24 templates. It is also contains a library crate,
[tinted-builder], which you can use to directly build templates within
your own Rust application.

## Table of Contents

- [CLI](#cli)
  - [Installation](#installation)
  - [Basic usage](#basic-usage)
  - [Commands](#commands)
  - [Flags](#flags)
- [Builder specification](#builder-specification)
- [Library](#library)
  - [Library installation](#library-installation)
  - [Library usage](#library-usage)
- [Contributing](#contributing)
- [License](#license)

## CLI

### Installation


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

### Basic Usage

```sh
tinted-builder-rust sync # To sync with latest schemes
tinted-builder-rust build path/to/base16-template
```

## Commands

The following is a table of the available subcommands for the CLI tool (tinted-builder-rust), including the descriptions and any notable arguments.

| Subcommand | Description                          | Arguments            | Example Usage                              |
|------------|--------------------------------------|----------------------|--------------------------------------------|
| `sync`  | Installs and or updates latest schemes. | - | `tinted-builder-rust sync` |
| `build` | Builds the themes of a template. | `template_path`: Path to template directory. | `tinted-builder-rust build ./path/to/base16-template` |

## Flags

| Flag/Option       | Description                             | Applicable Subcommands | Default Value | Example Usage                             |
|-------------------|-----------------------------------------|------------------------|---------------|-------------------------------------------|
| `--schemes-dir` `-s`   | Path to a custom local schemes directory to use when building. Only necessary if the [latest schemes repository] is not desired. | `build` | `tinted-builder-rust build . --schemes-dir=/path/to/schemes/dir` |
| `--data-dir` `-d`   | Specifies a custom path for the data directory. | All | Linux: `$XDG_DATA_HOME/tinted-theming/tinted-builder-rust` or `~/.local/share`. MacOS: `~/Library/Application\ Support/tinted-theming/tinted-builder-rust` | `tinted-builder-rust sync --data-dir /path/to/custom/data-dir` |
| `--help` `-h`     | Displays help information for the subcommand. | All | - | `tinted-builder-rust --help`, `tinted-builder-rust build --help`, etc |
| `--version` `-V`  | Shows the version of tinted-builder-rust. | All | - | `tinted-builder-rust --version` |

## Builder specification

tinted-builder-rust implements the `0.11.0` [builder specification]. This
specification details the scheme yaml format or schema as well as the
variables the builder should provide when rendering template mustache
file. Have a look at the [builder specification] document for more
details.

## Library

This library exposes a `Scheme` and `Template` struct which you can
use to generate your own themes using [base16] and [base24] templates and
`0.11.0` compliant base16 and base24 scheme files.

Internally tinted-builder-rust uses [ramhorns] to render the templates.

### Library installation

```sh
cargo add tinted-builder-rust
```

### Library Usage

```rust
use tinted_builder_rust::{Scheme, Template};
use std::fs::read_to_string;

let template_str = read_to_string("path/to/template.mustache").unwrap();
let scheme_str = read_to_string("path/to/scheme.yml").unwrap();

let template = Template::new(template_str).unwrap();
let scheme: Scheme = serde_yaml::from_str(&scheme_str).unwrap();

template
    .render_to_file("path/to/rendered/template", &scheme)
    .unwrap();
```

The Scheme struct is as follows:

```rust
use std::collections::HashMap;

pub struct Scheme {
    pub system: String,
    pub name: String,
    pub slug: String,
    pub author: String,
    pub description: Option<String>,
    pub variant: String,
    pub palette: HashMap<String, Color>,
}

pub struct Color {
    pub hex: (String, String, String),
    pub rgb: (u8, u8, u8),
    pub dec: (f32, f32, f32),
}
```

`Template::new`
The `Template` struct simply sets the content provided to it via
`Template::new`.

`template.render_to_file(&scheme)` takes the scheme and generates the
variables defined in the `0.11.0` [builder specification].

## Contributing

Contributions are welcome! Have a look at [CONTRIBUTING.md] for more
information.

## License

Ribboncurls is dual-licensed under the [Apache 2.0] and [MIT] licenses.
For more information about the licenses of the projects used by
Ribboncurls, have a look at [THIRD_PARTY_LICENSES.md].

[tinted-builder-rust]: tinted-builder-rust/README.md
[tinted-builder]: tinted-builder/README.md
[latest schemes repository]: https://github.com/tinted-theming/schemes
[home repository]: https://github.com/tinted-theming/home
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[base16]: https://github.com/tinted-theming/home/blob/main/styling.md
[base24]: https://github.com/tinted-theming/base24/blob/master/styling.md
[ramhorns]: https://docs.rs/ramhorns/latest/ramhorns/index.html
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[LICENSE]: LICENSE
[THIRD_PARTY_LICENSES.md]: THIRD_PARTY_LICENSES.md
[CONTRIBUTING.md]: CONTRIBUTING.md
[repository releases]: https://github.com/tinted-theming/tinty/releases/latest
