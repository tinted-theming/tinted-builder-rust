// With the new split of tinted-builder-rust (cli) and
// tinted-builder (lib), tinted-builder is exported here for bakward
// compatibility for a time. Everyone should move to using
// tinted-builder as the rust library.

/// # Add tinted-builder library test since tinted-builder-rust is
/// exporting the structs
///
/// ```
/// use tinted_builder_rust::{Scheme, Template};
///
/// let template = String::from(r#"/* Some CSS file with {{scheme-name}} theme */
/// .someCssSelector { background-color: #{{base00-hex}} }
/// .someOtherCssSelector { background-color: #{{base0F-hex}} }"#);
/// let scheme_str = r#"system: "base16"
/// name: "UwUnicorn"
/// author: "Fernando Marques (https://github.com/RakkiUwU) and Gabriel Fontes (https://github.com/Misterio77)"
/// variant: "dark"
/// palette:
///   base00: "241b26"
///   base01: "2f2a3f"
///   base02: "46354a"
///   base03: "6c3cb2"
///   base04: "7e5f83"
///   base05: "eed5d9"
///   base06: "d9c2c6"
///   base07: "e4ccd0"
///   base08: "877bb6"
///   base09: "de5b44"
///   base0A: "a84a73"
///   base0B: "c965bf"
///   base0C: "9c5fce"
///   base0D: "6a9eb5"
///   base0E: "78a38f"
///   base0F: "a3a079""#;
/// let template = Template::new(template).unwrap();
/// let scheme: Scheme = serde_yaml::from_str(&scheme_str).unwrap();
/// let output = template
///     .render(&scheme)
///     .unwrap();
///
///  assert_eq!(output, r#"/* Some CSS file with UwUnicorn theme */
/// .someCssSelector { background-color: #241b26 }
/// .someOtherCssSelector { background-color: #a3a079 }"#);
/// ```
pub use tinted_builder::Scheme;
pub use tinted_builder::Template;
