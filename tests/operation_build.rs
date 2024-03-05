mod common;

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::common::{write_to_file, COMMAND_NAME};

#[test]
fn test_operation_build() -> Result<()> {
    // -------
    // Arrange
    // -------
    let name = "operation_build";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let template_mustache_path = template_templates_path.join("default.mustache");
    let schemes_path = template_theme_path.join("schemes");
    let scheme_file_path = schemes_path.join("some-theme.yaml");
    let themes_path = template_theme_path.join("output-themes");
    let rendered_theme_path = themes_path.join("some-theme.md");

    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&scheme_file_path, SCHEME_CONTENT)?;
    write_to_file(&template_config_path, TEMPLATE_CONFIG_CONTENT)?;
    write_to_file(&template_mustache_path, TEMPLATE_MUSTACHE_CONTENT)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = common::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();
    println!(
        "out:{}\nerr:{}, path:{}",
        stdout,
        stderr,
        rendered_theme_path.display()
    );

    let rendered_content = fs::read_to_string(rendered_theme_path)?;
    println!("{}", rendered_content);

    // ------
    // Assert
    // ------
    assert_eq!(rendered_content, RENDERED_CONTENT);
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );

    Ok(())
}

const TEMPLATE_MUSTACHE_CONTENT: &str = r##"
name: {{scheme-name}}
system: {{scheme-system}}
variant: {{scheme-variant}}
slug: {{scheme-slug}}
slug_underscored: {{scheme-slug-underscored}}
description: {{scheme-description}}
author: {{scheme-author}}
is light: {{#scheme-is-light-variant}}true{{/scheme-is-light-variant}}{{^scheme-is-light-variant}}false{{/scheme-is-light-variant}}
is dark: {{#scheme-is-dark-variant}}true{{/scheme-is-dark-variant}}{{^scheme-is-dark-variant}}false{{/scheme-is-dark-variant}}

## hex
base00: {{base00-hex}} base01: {{base01-hex}} base02: {{base02-hex}} base03: {{base03-hex}} base04: {{base04-hex}} base05: {{base05-hex}} base06: {{base06-hex}} base07: {{base07-hex}} base08: {{base08-hex}} base09: {{base09-hex}} base0A: {{base0A-hex}} base0B: {{base0B-hex}} base0C: {{base0C-hex}} base0D: {{base0D-hex}} base0E: {{base0E-hex}} base0F: {{base0F-hex}}

## hex-bgr
base00: {{base00-hex-bgr}} base01: {{base01-hex-bgr}} base02: {{base02-hex-bgr}} base03: {{base03-hex-bgr}} base04: {{base04-hex-bgr}} base05: {{base05-hex-bgr}} base06: {{base06-hex-bgr}} base07: {{base07-hex-bgr}} base08: {{base08-hex-bgr}} base09: {{base09-hex-bgr}} base0A: {{base0A-hex-bgr}} base0B: {{base0B-hex-bgr}} base0C: {{base0C-hex-bgr}} base0D: {{base0D-hex-bgr}} base0E: {{base0E-hex-bgr}} base0F: {{base0F-hex-bgr}}

## hex-r
base00: {{base00-hex-r}} base01: {{base01-hex-r}} base02: {{base02-hex-r}} base03: {{base03-hex-r}} base04: {{base04-hex-r}} base05: {{base05-hex-r}} base06: {{base06-hex-r}} base07: {{base07-hex-r}} base08: {{base08-hex-r}} base09: {{base09-hex-r}} base0A: {{base0A-hex-r}} base0B: {{base0B-hex-r}} base0C: {{base0C-hex-r}} base0D: {{base0D-hex-r}} base0E: {{base0E-hex-r}} base0F: {{base0F-hex-r}}

## hex-g
base00: {{base00-hex-g}} base01: {{base01-hex-g}} base02: {{base02-hex-g}} base03: {{base03-hex-g}} base04: {{base04-hex-g}} base05: {{base05-hex-g}} base06: {{base06-hex-g}} base07: {{base07-hex-g}} base08: {{base08-hex-g}} base09: {{base09-hex-g}} base0A: {{base0A-hex-g}} base0B: {{base0B-hex-g}} base0C: {{base0C-hex-g}} base0D: {{base0D-hex-g}} base0E: {{base0E-hex-g}} base0F: {{base0F-hex-g}}

## hex-b
base00: {{base00-hex-b}} base01: {{base01-hex-b}} base02: {{base02-hex-b}} base03: {{base03-hex-b}} base04: {{base04-hex-b}} base05: {{base05-hex-b}} base06: {{base06-hex-b}} base07: {{base07-hex-b}} base08: {{base08-hex-b}} base09: {{base09-hex-b}} base0A: {{base0A-hex-b}} base0B: {{base0B-hex-b}} base0C: {{base0C-hex-b}} base0D: {{base0D-hex-b}} base0E: {{base0E-hex-b}} base0F: {{base0F-hex-b}}

## rgb-r
base00: {{base00-rgb-r}} base01: {{base01-rgb-r}} base02: {{base02-rgb-r}} base03: {{base03-rgb-r}} base04: {{base04-rgb-r}} base05: {{base05-rgb-r}} base06: {{base06-rgb-r}} base07: {{base07-rgb-r}} base08: {{base08-rgb-r}} base09: {{base09-rgb-r}} base0A: {{base0A-rgb-r}} base0B: {{base0B-rgb-r}} base0C: {{base0C-rgb-r}} base0D: {{base0D-rgb-r}} base0E: {{base0E-rgb-r}} base0F: {{base0F-rgb-r}}

## rgb-g
base00: {{base00-rgb-g}} base01: {{base01-rgb-g}} base02: {{base02-rgb-g}} base03: {{base03-rgb-g}} base04: {{base04-rgb-g}} base05: {{base05-rgb-g}} base06: {{base06-rgb-g}} base07: {{base07-rgb-g}} base08: {{base08-rgb-g}} base09: {{base09-rgb-g}} base0A: {{base0A-rgb-g}} base0B: {{base0B-rgb-g}} base0C: {{base0C-rgb-g}} base0D: {{base0D-rgb-g}} base0E: {{base0E-rgb-g}} base0F: {{base0F-rgb-g}}

## rgb-b
base00: {{base00-rgb-b}} base01: {{base01-rgb-b}} base02: {{base02-rgb-b}} base03: {{base03-rgb-b}} base04: {{base04-rgb-b}} base05: {{base05-rgb-b}} base06: {{base06-rgb-b}} base07: {{base07-rgb-b}} base08: {{base08-rgb-b}} base09: {{base09-rgb-b}} base0A: {{base0A-rgb-b}} base0B: {{base0B-rgb-b}} base0C: {{base0C-rgb-b}} base0D: {{base0D-rgb-b}} base0E: {{base0E-rgb-b}} base0F: {{base0F-rgb-b}}
"##;
const RENDERED_CONTENT: &str = r##"
name: Some Scheme
system: base16
variant: dark
slug: some-scheme
slug_underscored: some_scheme
description: Some Description
author: Some Author
is light: false
is dark: true

## hex
base00: 0f1419 base01: 131721 base02: 272d38 base03: 3e4b59 base04: bfbdb6 base05: e6e1cf base06: e6e1cf base07: f3f4f5 base08: f07178 base09: ff8f40 base0A: ffb454 base0B: b8cc52 base0C: 95e6cb base0D: 59c2ff base0E: d2a6ff base0F: e6b673

## hex-bgr
base00: 19140f base01: 211713 base02: 382d27 base03: 594b3e base04: b6bdbf base05: cfe1e6 base06: cfe1e6 base07: f5f4f3 base08: 7871f0 base09: 408fff base0A: 54b4ff base0B: 52ccb8 base0C: cbe695 base0D: ffc259 base0E: ffa6d2 base0F: 73b6e6

## hex-r
base00: 0f base01: 13 base02: 27 base03: 3e base04: bf base05: e6 base06: e6 base07: f3 base08: f0 base09: ff base0A: ff base0B: b8 base0C: 95 base0D: 59 base0E: d2 base0F: e6

## hex-g
base00: 14 base01: 17 base02: 2d base03: 4b base04: bd base05: e1 base06: e1 base07: f4 base08: 71 base09: 8f base0A: b4 base0B: cc base0C: e6 base0D: c2 base0E: a6 base0F: b6

## hex-b
base00: 19 base01: 21 base02: 38 base03: 59 base04: b6 base05: cf base06: cf base07: f5 base08: 78 base09: 40 base0A: 54 base0B: 52 base0C: cb base0D: ff base0E: ff base0F: 73

## rgb-r
base00: 15 base01: 19 base02: 39 base03: 62 base04: 191 base05: 230 base06: 230 base07: 243 base08: 240 base09: 255 base0A: 255 base0B: 184 base0C: 149 base0D: 89 base0E: 210 base0F: 230

## rgb-g
base00: 20 base01: 23 base02: 45 base03: 75 base04: 189 base05: 225 base06: 225 base07: 244 base08: 113 base09: 143 base0A: 180 base0B: 204 base0C: 230 base0D: 194 base0E: 166 base0F: 182

## rgb-b
base00: 25 base01: 33 base02: 56 base03: 89 base04: 182 base05: 207 base06: 207 base07: 245 base08: 120 base09: 64 base0A: 84 base0B: 82 base0C: 203 base0D: 255 base0E: 255 base0F: 115"##;
const SCHEME_CONTENT: &str = r##"
name: "Some Scheme"
author: "Some Author"
description: "Some Description"
system: "base16"
palette:
  base00: "0F1419"
  base01: "131721"
  base02: "272D38"
  base03: "3E4B59"
  base04: "BFBDB6"
  base05: "E6E1CF"
  base06: "E6E1CF"
  base07: "F3F4F5"
  base08: "F07178"
  base09: "FF8F40"
  base0A: "FFB454"
  base0B: "B8CC52"
  base0C: "95E6CB"
  base0D: "59C2FF"
  base0E: "D2A6FF"
  base0F: "E6B673"
"##;
const TEMPLATE_CONFIG_CONTENT: &str = r##"
default: 
  extension: .md
  output: output-themes
"##;
