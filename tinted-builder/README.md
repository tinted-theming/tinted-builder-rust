# tinted-builder

[![Matrix Chat](https://img.shields.io/matrix/tinted-theming:matrix.org)](https://matrix.to/#/#tinted-theming:matrix.org)
[![Crates.io](https://img.shields.io/crates/v/tinted-builder.svg)](https://crates.io/crates/tinted-builder)
[![Tests](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml)

A Rust library to generate [base16] and [base24] templates using the
`0.11.1` [builder specification].

This library exposes a `Scheme` and `Template` struct which you can use
to generate your own themes using [base16] and [base24] templates and
`0.11.1` compliant base16 and base24 scheme files.

Internally tinted-builder uses [ribboncurls] to render the templates.

## Tinted8 (library)

In addition to Base16/Base24, the library supports Tinted8 schemes. Deserialize a Tinted8 scheme and wrap it in `Scheme::Tinted8` to render templates with nested variables.

```rust
use tinted_builder::{Scheme, Template};
use tinted_builder::tinted8::Scheme as T8Scheme;

let yml = r##"
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
"##;

let t8: T8Scheme = serde_yaml::from_str(yml).unwrap();
let scheme = Scheme::Tinted8(Box::new(t8));
let tpl = Template::new("{{ palette.blue.bright.hex }}".to_string(), scheme);
let out = tpl.render().unwrap();
```

### Tinted8 template variables

- Palette: `palette.<color>.<variant>.<field>` (e.g., `palette.red.normal.hex`)
- UI: `ui.<key>.<field>` (e.g., `ui.background.rgb.r`)
- Syntax: `syntax.<key>.<field>` (e.g., `syntax.string.dec.g`)

Each color object provides:

- hex: 6-digit hex string without `#`
- hex-r / hex-g / hex-b: 2-digit hex components
- hex-bgr: 6-digit hex in BGR order
- rgb: numbers { r, g, b } in 0–255
- rgb16: numbers { r, g, b } in 0–65535 (8-bit × 257)
- dec: strings { r, g, b } in 0–1 with 8-decimal precision

Note: Base16/Base24 templates use flat keys such as `base0A-hex`, `base0A-rgb-r`. Tinted8 uses nested objects as shown above.

## Installation

```sh
cargo add tinted-builder
```

## Usage

```rust
use tinted_builder::{Scheme, Template};

let template = String::from(r#"/* Some CSS file with {{scheme-name}} theme */
.someCssSelector { background-color: #{{base00-hex}} }
.someOtherCssSelector { background-color: #{{base0F-hex}} }"#);
let scheme_str = r##"system: "base16"
name: "UwUnicorn"
author: "Fernando Marques (https://github.com/RakkiUwU) and Gabriel Fontes (https://github.com/Misterio77)"
variant: "dark"
palette:
  base00: "#241b26"
  base01: "#2f2a3f"
  base02: "#46354a"
  base03: "#6c3cb2"
  base04: "#7e5f83"
  base05: "#eed5d9"
  base06: "#d9c2c6"
  base07: "#e4ccd0"
  base08: "#877bb6"
  base09: "#de5b44"
  base0A: "#a84a73"
  base0B: "#c965bf"
  base0C: "#9c5fce"
  base0D: "#6a9eb5"
  base0E: "#78a38f"
  base0F: "#a3a079""##;
let scheme = Scheme::Base16(serde_yaml::from_str(scheme_str).unwrap());
let template = Template::new(template, scheme);
let output = template
  .render()
  .unwrap();

  assert_eq!(output, r#"/* Some CSS file with UwUnicorn theme */
.someCssSelector { background-color: #241b26 }
.someOtherCssSelector { background-color: #a3a079 }"#);
```

1. Create a scheme (`Scheme`) enum variant while providing the
   deserialized data into into the variant:
   `Scheme::Base16(serde_yaml::from_str(&scheme_str).unwrap())` in this
   case
2. Create a template by passing the serialized mustache text and the
   `Scheme` variant in step 1 into the `Template` struct:
   `Template::new(mustache_text, scheme)`. The `template.render()`
   method takes the scheme, generates the variables defined in the 
   `0.11.1` [builder specification] and returns a new string.
3. Render the template by running a method which returns a
   `Result<String, TintedBuilderError>` type: 
   `let output = template.render().unwrap();`

## Contributing

Contributions are welcome! Have a look at [CONTRIBUTING.md] for more
information.

## License

tinted-builder falls under the [GPL-3.0] license. Have a look at the
[LICENSE] file.

[latest schemes repository]: https://github.com/tinted-theming/schemes
[home repository]: https://github.com/tinted-theming/home
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[base16]: https://github.com/tinted-theming/home/blob/main/styling.md
[base24]: https://github.com/tinted-theming/base24/blob/master/styling.md
[ribboncurls]: https://github.com/tinted-theming/ribboncurls
[CONTRIBUTING.md]: ../CONTRIBUTING.md
[repository releases]: https://github.com/tinted-theming/tinted-builder-rust/releases/latest
[GPL-3.0]: https://github.com/IQAndreas/markdown-licenses/blob/master/gnu-gpl-v3.0.md
[LICENSE]: ../LICENSE
