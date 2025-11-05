# tinted-builder-rust

[![Matrix Chat](https://img.shields.io/matrix/tinted-theming:matrix.org)](https://matrix.to/#/#tinted-theming:matrix.org)
[![Crates.io](https://img.shields.io/crates/v/tinted-builder-rust.svg)](https://crates.io/crates/tinted-builder-rust)
[![Tests](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml)

A builder for [base16] and [base24] templates using the `0.11.1` [builder
specification].

This repo contains a command-line tool, [tinted-builder-rust], to build
base16 and base24 templates. It is also contains a library crate,
[tinted-builder], which you can use to directly build templates within
your own Rust application.

## Table of Contents

- [CLI](#cli)
  - [Requirements](#requirements)
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

### Requirements

`git` is required to be installed if using the `tinted-builder-rust sync`
subcommand.

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

| Subcommand | Description                          | Arguments            | Example Usage                              | Flags |
|------------|--------------------------------------|----------------------|--------------------------------------------|-------|
| `sync`  | Installs and or updates latest schemes. | - | `tinted-builder-rust sync` | `--quiet` (silence stderr and stdout) |
| `build` | Builds the themes of a template. | `template_path`: Path to template directory. | `tinted-builder-rust build ./path/to/base16-template` | `--quiet` (silence stderr and stdout), `--sync` (equivalent of running `tinted-builder-rust sync` before `tinted-builder-rust build`) |

## Flags

| Flag/Option       | Description                             | Applicable Subcommands | Default Value | Example Usage                             |
|-------------------|-----------------------------------------|------------------------|---------------|-------------------------------------------|
| `--schemes-dir` `-s`   | Path to one or more local schemes directories. Repeat this flag to include multiple directories. Used by `build` to find schemes and by `sync` to clone/pull into those paths. | `build`, `sync` | Defaults to `<data-dir>/schemes` | `tinted-builder-rust build . -s /path/one -s /path/two` |
| `--data-dir` `-d`   | Specifies a custom path for the data directory. | All | Linux: `$XDG_DATA_HOME/tinted-theming/tinted-builder-rust` or `~/.local/share`. MacOS: `~/Library/Application\ Support/tinted-theming/tinted-builder-rust` | `tinted-builder-rust sync --data-dir /path/to/custom/data-dir` |
| `--help` `-h`     | Displays help information for the subcommand. | All | - | `tinted-builder-rust --help`, `tinted-builder-rust build --help`, etc |
| `--version` `-V`  | Shows the version of tinted-builder-rust. | All | - | `tinted-builder-rust --version` |

## Builder specification

tinted-builder-rust implements the `0.11.1` [builder specification]. This
specification details the scheme yaml format or schema as well as the
variables the builder should provide when rendering template mustache
file. Have a look at the [builder specification] document for more
details.

### Tinted8 support

In addition to Base16/Base24, this repository provides Tinted8 support in the
library crate and CLI. Tinted8 simplifies schemes into a small, consistent set
of keys and lets builders derive variants and supplemental colors.

- Supported specs: Tinted8 Styling `~0.1.0`, Tinted8 Builder `~0.1.0`.
- UI keys use flat, kebab-case names (see below).

Example minimal Tinted8 scheme (YAML):

```
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  author: "User <user@example.com>"
  name: "Ayu Mirage"
  slug: "ayu-mirage"
variant: "dark"
palette:
  black:   "#131721"
  red:     "#f07178"
  green:   "#b8cc52"
  yellow:  "#ffb454"
  blue:    "#59c2ff"
  magenta: "#d2a6ff"
  cyan:    "#95e6cb"
  white:   "#e6e1cf"
ui:
  background.normal: "#131721"
  foreground.normal: "#e6e1cf"
```

Example `templates/config.yaml` for a Tinted8 template:

```
default:
  filename: "output/{{ scheme-system }}-{{ scheme-slug }}.ext"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.1.0"
    tinted8-builder: ">=0.1.0"
```

Example Mustache variables in a Tinted8 template:

- `{{ scheme.name }}` — scheme name
- `{{ scheme.slug }}` — scheme slug
- `{{ palette.blue.bright.hex }}` — hex of blue bright variant (derived if missing)
- `{{ ui.background.hex }}` — hex of the UI background color
- `{{ syntax.string.hex }}` — hex for the default string color

Notes:
- The builder derives missing `dim/bright` variants and supplemental `gray`,
  `orange`, and `brown` per the Tinted8 Builder spec.
- Decimal channels are normalized 0–1 and available as strings at
  `rgb`/`rgb16`/`dec` under each color (e.g. `palette.red.normal.dec.r`).

### Error codes (high level)

The CLI returns structured error codes grouped by stage. See `specs/tinted8/builder.md` for details.

- E1xx — Intake & System Validation (e.g., E001 invalid system, E110 unknown system)
- E2xx — Spec Compatibility (E002/E003 version mismatches)
- E3xx — Template Configuration (E300–E305 missing/invalid config or templates)
- E4xx — Build-Time Selection (E400 no schemes found)

## Library

This library exposes a `Scheme` and `Template` struct which you can
use to generate your own themes using [base16] and [base24] templates and
`0.11.1` compliant base16 and base24 scheme files.

Internally tinted-builder-rust uses [ribboncurls] to render the templates.

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
let scheme = Scheme::Base16(serde_yaml::from_str(&scheme_str).unwrap());
let template = Template::new(template_str, scheme);

template
    .render()
    .unwrap();

### Programmatic CLI usage (build helper)

The CLI crate also exposes a small helper to drive a template build from code:

```rust
// in Cargo.toml: tinted-builder-rust = { version = "*" }

let template_dir = std::path::PathBuf::from("/path/to/template");
let schemes_dir = std::path::PathBuf::from("/path/to/schemes");

// Quiet output, returns Result<(), anyhow::Error>
if let Err(err) = tinted_builder_rust::build(&template_dir, &schemes_dir, true) {
    eprintln!("build failed: {err}");
}
```

Common errors include E300/E301/E302 (missing tinted8 supports), E303 (missing mustache),
E304 (invalid filename config), E305 (missing/invalid template config), and E400 (no schemes found).
```

The Scheme struct is as follows:

```rust
use std::collections::HashMap;
use tinted_builder::{SchemeSystem, SchemeVariant};

pub struct Scheme {
    pub system: SchemeSystem,
    pub name: String,
    pub slug: String,
    pub author: String,
    pub description: Option<String>,
    pub variant: SchemeVariant,
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
variables defined in the `0.11.1` [builder specification].

## Contributing

Contributions are welcome! Have a look at [CONTRIBUTING.md] for more
information.

## License

Ribboncurls is dual-licensed under the [Apache 2.0] and [MIT] licenses.
For more information about the licenses of the projects used by
Ribboncurls, have a look at [LICENSES-THIRD-PARTY.md].

[tinted-builder-rust]: tinted-builder-rust/README.md
[tinted-builder]: tinted-builder/README.md
[latest schemes repository]: https://github.com/tinted-theming/schemes
[home repository]: https://github.com/tinted-theming/home
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[tinted8 styling spec]: specs/tinted8/styling.md
[tinted8 builder spec]: specs/tinted8/builder.md
[base16]: https://github.com/tinted-theming/home/blob/main/styling.md
[base24]: https://github.com/tinted-theming/base24/blob/master/styling.md
[ribboncurls]: https://github.com/tinted-theming/ribboncurls
[LICENSE]: LICENSE
[LICENSES-THIRD-PARTY.md]: LICENSES-THIRD-PARTY.md
[CONTRIBUTING.md]: CONTRIBUTING.md
[repository releases]: https://github.com/tinted-theming/tinted-builder-rust/releases/latest
