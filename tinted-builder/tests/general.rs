use anyhow::Result;
use tinted_builder::{Scheme, Template, TintedBuilderError};

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
    let template_source = r#"Author: {{{scheme-author}}}
Author escaped: {{scheme-author}}"#;
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
