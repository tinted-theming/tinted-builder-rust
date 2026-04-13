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
- [Development](#development)
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
| `--schemes-dir` `-s`   | Path to local schemes directories. Used by `build` to find schemes and by `sync` to clone/pull into those paths. | `build`, `sync` | Defaults to `<data-dir>/schemes` | `tinted-builder-rust build . -s /path/one -s /path/two` |
| `--ignore` `-i`   | One or more glob patterns to skip when scanning schemes. Repeat this flag to add multiple ignores. | `build` | - | `tinted-builder-rust build . --ignore "**/LICENSE"` |
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

- Supported specs: Tinted8 Styling `~0.2.0`, Tinted8 Builder `~0.2.0`.
- UI keys use flat, kebab-case names (see below).

Example minimal Tinted8 scheme (YAML):

```
scheme:
  system: "tinted8"
  supports:
    styling-spec: "0.2.0"
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

This library exposes a `Scheme` enum and `Template` struct which you can
use to generate your own themes using [base16], [base24], and Tinted8
templates and scheme files.

Internally tinted-builder-rust uses [ribboncurls] to render the templates.

### Library installation

```sh
cargo add tinted-builder
```

### Library Usage

```rust
use tinted_builder::{Scheme, Template};
use std::fs::read_to_string;

let template_str = read_to_string("path/to/template.mustache").unwrap();
let scheme_str = read_to_string("path/to/scheme.yml").unwrap();

// Auto-detect the scheme system from the YAML
let scheme = Scheme::from_yaml(&scheme_str).unwrap();
let template = Template::new(template_str, scheme);
let output = template.render().unwrap();
```

You can also construct scheme variants directly:

```rust
use tinted_builder::Scheme;

// Base16
let scheme = Scheme::Base16(serde_yaml::from_str(&scheme_str).unwrap());

// Base24
let scheme = Scheme::Base24(serde_yaml::from_str(&scheme_str).unwrap());

// Tinted8
let scheme = Scheme::Tinted8(Box::new(serde_yaml::from_str(&scheme_str).unwrap()));
```

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

### Scheme types

Each scheme system has its own type under a dedicated module:

- `base16::Scheme` — 16-color palette (`base00` through `base0F`)
- `base24::Scheme` — 24-color palette (`base00` through `base17`)
- `tinted8::Scheme` — palette, syntax, and UI colors

All three share the same struct shape for `Color`:

```rust
pub struct Color {
    pub hex: (String, String, String),  // (rr, gg, bb) lowercase
    pub rgb: (u8, u8, u8),
    pub dec: (f32, f32, f32),           // [0.0, 1.0]
    pub name: ColorName,
    pub variant: ColorVariant,
}
```

`Template::new` takes a `Scheme` enum and template content string.
`template.render()` replaces placeholders with values from the scheme
as defined in the [builder specification].

## Development

A [justfile] is provided for common development tasks. Run `just` to
list available recipes.

| Recipe | Description | Example |
|--------|-------------|---------|
| `test` | Run tests for all crates, or a specific crate | `just test`, `just test tinted-builder` |
| `test-wasm` | Run wasm-specific tests (requires `nix develop`) | `just test-wasm` |
| `fmt` | Format Rust and Nix files | `just fmt` |
| `lint` | Run clippy | `just lint` |
| `clean` | Remove build artifacts | `just clean` |

### WASM tests

The `tinted-builder` crate includes WASM-specific unit tests that run
under wasmtime. These require the `wasm32-wasip2` target, which is
provided by the Nix dev shell:

```sh
nix develop
just test-wasm --target wasm32-wasip2
```

## Contributing

Contributions are welcome! Have a look at [CONTRIBUTING.md] for more
information.

## License

tinted-builder-rust falls under the [GPL-3.0] license. Have a look at the
[LICENSE] file.

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
[CONTRIBUTING.md]: CONTRIBUTING.md
[repository releases]: https://github.com/tinted-theming/tinted-builder-rust/releases/latest
[justfile]: https://just.systems/
[GPL-3.0]: https://github.com/IQAndreas/markdown-licenses/blob/master/gnu-gpl-v3.0.md
[LICENSE]: ./LICENSE
