# tinted-builder

[![Matrix Chat](https://img.shields.io/matrix/tinted-theming:matrix.org)](https://matrix.to/#/#tinted-theming:matrix.org)
[![Crates.io](https://img.shields.io/crates/v/tinted-builder-rust.svg)](https://crates.io/crates/tinted-builder-rust)
[![Tests](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tinted-theming/tinted-builder-rust/actions/workflows/ci.yml)

A Rust library to generate [base16] and [base24] templates using the `0.11.0` [builder
specification].

This library exposes a `Scheme` and `Template` struct which you can
use to generate your own themes using [base16] and [base24] templates and
`0.11.0` compliant base16 and base24 scheme files.

Internally tinted-builder uses [ribboncurls] to render the templates.

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
let scheme_str = r#"system: "base16"
name: "UwUnicorn"
author: "Fernando Marques (https://github.com/RakkiUwU) and Gabriel Fontes (https://github.com/Misterio77)"
variant: "dark"
palette:
  base00: "241b26"
  base01: "2f2a3f"
  base02: "46354a"
  base03: "6c3cb2"
  base04: "7e5f83"
  base05: "eed5d9"
  base06: "d9c2c6"
  base07: "e4ccd0"
  base08: "877bb6"
  base09: "de5b44"
  base0A: "a84a73"
  base0B: "c965bf"
  base0C: "9c5fce"
  base0D: "6a9eb5"
  base0E: "78a38f"
  base0F: "a3a079""#;
let scheme = Scheme::Base16(serde_yaml::from_str(&scheme_str).unwrap());
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
   `0.11.0` [builder specification] and returns a new string.
3. Render the template by running a method which returns a
   `Result<String, TintedBuilderError>` type: 
   `let output = template.render().unwrap();`

## Contributing

Contributions are welcome! Have a look at [CONTRIBUTING.md] for more
information.

## License

Ribboncurls is dual-licensed under the [Apache 2.0] and [MIT] licenses.
For more information about the licenses of the projects used by
Ribboncurls, have a look at [THIRD_PARTY_LICENSES.md].

[latest schemes repository]: https://github.com/tinted-theming/schemes
[home repository]: https://github.com/tinted-theming/home
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[base16]: https://github.com/tinted-theming/home/blob/main/styling.md
[base24]: https://github.com/tinted-theming/base24/blob/master/styling.md
[ribboncurls]: https://github.com/tinted-theming/ribboncurls
[builder specification]: https://github.com/tinted-theming/home/blob/main/builder.md
[LICENSE]: LICENSE
[THIRD_PARTY_LICENSES.md]: THIRD_PARTY_LICENSES.md
[CONTRIBUTING.md]: CONTRIBUTING.md
[repository releases]: https://github.com/tinted-theming/tinted-builder-rust/releases/latest
