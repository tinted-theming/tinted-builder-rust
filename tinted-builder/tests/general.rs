use anyhow::Result;
use tinted_builder::{Scheme, Template};

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
fn render_without_content() -> Result<()> {
    let template_source = "Hello!".to_string();
    let scheme: Scheme = serde_yaml::from_str(SCHEME_SILK_LIGHT)?;
    let template = Template::new(template_source, scheme.system.clone());

    let output = template.render(&scheme)?;

    assert_eq!(output, "Hello!");
    Ok(())
}

#[test]
fn comments() -> Result<()> {
    let template_source =
        r#"<div style="background-color: #{{base09-hex}};">{{ ! some # comment }}</div>"#;
    let scheme: Scheme = serde_yaml::from_str(SCHEME_SILK_LIGHT)?;
    let template = Template::new(template_source.to_string(), scheme.system.clone());

    let output = template.render(&scheme)?;

    assert_eq!(&output, r#"<div style="background-color: #d27f46;"></div>"#);
    Ok(())
}

#[test]
fn escaped_and_unescaped_vars() -> Result<()> {
    let template_source = r#"Author: {{{scheme-author}}}
Author escaped: {{scheme-author}}"#;
    let expected = r#"Author: <a href="https://github.com/Misterio77">Gabriel Fontes</a>
Author escaped: &lt;a href=&quot;https://github.com/Misterio77&quot;&gt;Gabriel Fontes&lt;/a&gt;"#;
    let scheme: Scheme = serde_yaml::from_str(SCHEME_CRAZY)?;
    let template = Template::new(template_source.to_string(), scheme.system.clone());

    let output = template.render(&scheme)?;

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn with_basic_sections() -> Result<()> {
    let template_source =
        "Does base17 var exist: {{#base17-hex}}Yes{{/base17-hex}}{{^base17-hex}}No{{/base17-hex}}";
    let scheme: Scheme = serde_yaml::from_str(SCHEME_SILK_LIGHT)?;
    let template = Template::new(template_source.to_string(), scheme.system.clone());

    let output = template.render(&scheme)?;

    assert_eq!(output, "Does base17 var exist: No");
    Ok(())
}

#[test]
fn with_nested_sections() -> Result<()> {
    let template_source = "{{#scheme-author}}{{#scheme-slug}}{{#base0A-hex}}#{{.}}{{/base0A-hex}}{{/scheme-slug}}{{/scheme-author}}";
    let scheme: Scheme = serde_yaml::from_str(SCHEME_SILK_LIGHT)?;
    let template = Template::new(template_source.to_string(), scheme.system.clone());

    let output = template.render(&scheme)?;

    assert_eq!(output, "#cfad25");
    Ok(())
}
