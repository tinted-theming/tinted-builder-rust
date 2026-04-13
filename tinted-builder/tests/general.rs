use anyhow::Result;
use tinted_builder::{Scheme, SchemeSystem, SchemeVariant, Template, TintedBuilderError};

#[test]
fn render_without_content() -> Result<(), TintedBuilderError> {
    let template_source = "Hello!".to_string();
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source, scheme);

    let output = template.render()?;

    assert_eq!(output, "Hello!");
    Ok(())
}

#[test]
fn comments() -> Result<(), TintedBuilderError> {
    let template_source =
        r#"<div style="background-color: #{{base09-hex}};">{{ ! some # comment }}</div>"#;
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(&output, r#"<div style="background-color: #d27f46;"></div>"#);
    Ok(())
}

#[test]
fn escaped_and_unescaped_vars() -> Result<(), TintedBuilderError> {
    let template_source = r"Author: {{{scheme-author}}}
Author escaped: {{scheme-author}}";
    let expected = r#"Author: <a href="https://github.com/Misterio77">Gabriel Fontes</a>
Author escaped: &lt;a href=&quot;https://github.com/Misterio77&quot;&gt;Gabriel Fontes&lt;/a&gt;"#;
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_CRAZY)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn with_basic_sections() -> Result<(), TintedBuilderError> {
    let template_source =
        "Does base17 var exist: {{#base17-hex}}Yes{{/base17-hex}}{{^base17-hex}}No{{/base17-hex}}";
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, "Does base17 var exist: No");
    Ok(())
}

#[test]
fn with_nested_sections() -> Result<(), TintedBuilderError> {
    let template_source = "{{#scheme-author}}{{#scheme-slug}}{{#base0A-hex}}#{{.}}{{/base0A-hex}}{{/scheme-slug}}{{/scheme-author}}";
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, "#cfad25");
    Ok(())
}

#[test]
fn render_hex() -> Result<()> {
    let template_source = "{{base0A-hex}}";
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, "cfad25");
    Ok(())
}

#[test]
fn render_rgb() -> Result<()> {
    let template_source = "{{base0A-rgb-r}} {{base0A-rgb-g}} {{base0A-rgb-b}}";
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, "207 173 37");
    Ok(())
}

#[test]
fn render_rgb16() -> Result<()> {
    let template_source = "{{base0A-rgb16-r}} {{base0A-rgb16-g}} {{base0A-rgb16-b}}";
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, "53199 44461 9509");
    Ok(())
}

#[test]
fn render_dec() -> Result<()> {
    let template_source = "{{base0A-dec-r}} {{base0A-dec-g}} {{base0A-dec-b}}";
    let scheme = Scheme::Base16(serde_yaml::from_str(SCHEME_SILK_LIGHT)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, "0.81176471 0.67843137 0.14509804");
    Ok(())
}

#[test]
fn render_is_dark_variant() -> Result<()> {
    let template_source =
        "{{#option.is-dark-variant}}it is a dark variant!{{/option.is-dark-variant}}";
    let scheme = Scheme::Tinted8(serde_yaml::from_str(SCHEME_TINTED_CATPPUCCIN_MOCHA)?);
    let template = Template::new(template_source.to_string(), scheme);

    let output = template.render()?;

    assert_eq!(output, "it is a dark variant!");
    Ok(())
}

// ---------------------------------------------------------------------------
// Scheme::from_yaml() tests
// ---------------------------------------------------------------------------

#[test]
fn from_yaml_base16() -> Result<()> {
    let scheme = Scheme::from_yaml(SCHEME_SILK_LIGHT)?;
    assert_eq!(scheme.get_scheme_system(), SchemeSystem::Base16);
    assert_eq!(scheme.get_scheme_name(), "Silk Light");
    assert_eq!(scheme.get_scheme_variant(), SchemeVariant::Light);

    let template = Template::new("{{base0A-hex}}".to_string(), scheme);
    assert_eq!(template.render()?, "cfad25");
    Ok(())
}

#[test]
fn from_yaml_base24() -> Result<()> {
    let scheme = Scheme::from_yaml(SCHEME_BASE24)?;
    assert_eq!(scheme.get_scheme_system(), SchemeSystem::Base24);
    assert_eq!(scheme.get_scheme_name(), "Base24 Test");
    assert_eq!(scheme.get_scheme_variant(), SchemeVariant::Dark);

    let template = Template::new("{{base00-hex}}".to_string(), scheme);
    assert_eq!(template.render()?, "1a1b26");
    Ok(())
}

#[test]
fn from_yaml_tinted8() -> Result<()> {
    let scheme = Scheme::from_yaml(SCHEME_TINTED_CATPPUCCIN_MOCHA)?;
    assert_eq!(scheme.get_scheme_system(), SchemeSystem::Tinted8);
    assert_eq!(scheme.get_scheme_name(), "Catppuccin Mocha");
    assert_eq!(scheme.get_scheme_variant(), SchemeVariant::Dark);

    let template = Template::new("{{scheme.name}}".to_string(), scheme);
    assert_eq!(template.render()?, "Catppuccin Mocha");
    Ok(())
}

#[test]
fn from_yaml_tinted8_palette_render() -> Result<()> {
    let scheme = Scheme::from_yaml(SCHEME_TINTED_CATPPUCCIN_MOCHA)?;
    let template = Template::new("{{palette.red.normal.hex}}".to_string(), scheme);
    assert_eq!(template.render()?, "f38ba8");
    Ok(())
}

#[test]
fn from_yaml_tinted8_is_dark_variant() -> Result<()> {
    let scheme = Scheme::from_yaml(SCHEME_TINTED_CATPPUCCIN_MOCHA)?;
    let template = Template::new(
        "{{#option.is-dark-variant}}dark{{/option.is-dark-variant}}".to_string(),
        scheme,
    );
    assert_eq!(template.render()?, "dark");
    Ok(())
}

#[test]
fn from_yaml_missing_system() {
    let yaml = r#"
name: "No System"
author: "Test"
variant: "dark"
palette:
  base00: "000000"
"#;
    let result = Scheme::from_yaml(yaml);
    assert!(result.is_err());
}

#[test]
fn from_yaml_invalid_system() {
    let yaml = r#"
system: "unknown"
name: "Bad System"
author: "Test"
variant: "dark"
palette:
  base00: "000000"
"#;
    let result = Scheme::from_yaml(yaml);
    assert!(result.is_err());
}

const SCHEME_SILK_LIGHT: &str = r##"
system: "base16"
name: "Silk Light"
slug: "siłk light"
author: "Gabriel Fontes (https://github.com/Misterio77)"
variant: "light"
palette:
  base00: "#E9F1EF"
  base01: "#CCD4D3"
  base02: "#90B7B6"
  base03: "#5C787B"
  base04: "#4B5B5F"
  base05: "#385156"
  base06: "#0e3c46"
  base07: "#D2FAFF"
  base08: "#CF432E"
  base09: "#D27F46"
  base0A: "#CFAD25"
  base0B: "#6CA38C"
  base0C: "#329CA2"
  base0D: "#39AAC9"
  base0E: "#6E6582"
  base0F: "#865369"
"##;

const SCHEME_CRAZY: &str = r#"
system: "base16"
name: "Silk Light"
author: <a href="https://github.com/Misterio77">Gabriel Fontes</a>
variant: "light"
palette:
  base00: "E9F1EF"
  base01: "CCD4D3"
  base02: "90B7B6"
  base03: "5C787B"
  base04: "4B5B5F"
  base05: "385156"
  base06: "0e3c46"
  base07: "D2FAFF"
  base08: "CF432E"
  base09: "D27F46"
  base0A: "CFAD25"
  base0B: "6CA38C"
  base0C: "329CA2"
  base0D: "39AAC9"
  base0E: "6E6582"
  base0F: "865369"
"#;

const SCHEME_BASE24: &str = r##"
system: "base24"
name: "Base24 Test"
author: "Test Author"
variant: "dark"
palette:
  base00: "#1a1b26"
  base01: "#16161e"
  base02: "#2f3549"
  base03: "#444b6a"
  base04: "#787c99"
  base05: "#a9b1d6"
  base06: "#cbccd1"
  base07: "#d5d6db"
  base08: "#c0caf5"
  base09: "#a9b1d6"
  base0A: "#0db9d7"
  base0B: "#9ece6a"
  base0C: "#b4f9f8"
  base0D: "#2ac3de"
  base0E: "#bb9af7"
  base0F: "#f7768e"
  base10: "#0a0e14"
  base11: "#06080a"
  base12: "#ff7733"
  base13: "#ff9e64"
  base14: "#73daca"
  base15: "#7dcfff"
  base16: "#7aa2f7"
  base17: "#c0caf5"
"##;

const SCHEME_TINTED_CATPPUCCIN_MOCHA: &str = r##"
scheme:
  system: "tinted8"
  supports:
    styling-spec: "0.2.0"
  name: "Catppuccin Mocha"
  author: "https://github.com/catppuccin/catppuccin"
variant: "dark"
palette:
  black: "#1e1e2e"
  white: "#cdd6f4"
  red: "#f38ba8"
  yellow: "#f9e2af"
  green: "#a6e3a1"
  cyan: "#94e2d5"
  blue: "#89b4fa"
  magenta: "#cba6f7"
"##;
