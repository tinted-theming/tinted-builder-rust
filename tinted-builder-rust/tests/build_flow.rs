mod test_utils;
use anyhow::Result;
use std::fs::{self, create_dir_all};
use test_utils::write_to_file;

use crate::test_utils::unique_tmp_dir;

#[test]
fn e305_missing_template_config() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e305")?;
    let schemes = tmp_dir.join("schemes");
    let template = tmp_dir.join("template");

    create_dir_all(&schemes)?;
    create_dir_all(&template)?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();
    let msg = err.to_string();

    assert!(msg.contains("E305"), "expected E305, got: {msg}");
    Ok(())
}

#[test]
fn e300_missing_supports_for_tinted8() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e300")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let scheme_yaml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  name: "Test"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "{{scheme.name}}\n")?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();
    let msg = err.to_string();

    assert!(msg.contains("E300"), "expected E300, got: {msg}");
    Ok(())
}

#[test]
fn e303_missing_mustache_template() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e303")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let scheme_yaml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  name: "Test"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.1.0"
    tinted8-builder: ">=0.1.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();
    let msg = err.to_string();

    assert!(msg.contains("E303"), "expected E303, got: {msg}");
    Ok(())
}

#[test]
fn e002_styling_version_incompatible_template() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e002")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let scheme_yaml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  name: "Test"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=999.0.0"
    tinted8-builder: ">=0.0.0"
"#;
    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "Hello\n")?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();

    assert!(err.to_string().contains("E002"));
    Ok(())
}

#[test]
fn e003_builder_version_incompatible() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e003")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let scheme_yaml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  name: "Test"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.0.0"
    tinted8-builder: ">=999.0.0"
"#;
    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "Hello\n")?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();

    assert!(err.to_string().contains("E003"));
    Ok(())
}

#[test]
fn e301_missing_styling_entry() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e301")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let scheme_yaml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  name: "Test"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-builder: ">=0.0.0"
"#;
    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "Hello\n")?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();

    assert!(err.to_string().contains("E301"));
    Ok(())
}

#[test]
fn e302_missing_builder_entry() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e302")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let scheme_yaml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  name: "Test"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.0.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "Hello\n")?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();

    assert!(err.to_string().contains("E302"));
    Ok(())
}

#[test]
fn e111_invalid_extension_in_schemes() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e111")?;
    let schemes = tmp_dir.join("schemes");
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.0.0"
    tinted8-builder: ">=0.0.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("bad.txt"), "not a scheme")?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "Hello\n")?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();

    assert!(err.to_string().contains("E111"));
    Ok(())
}

#[test]
fn e110_unknown_scheme_system() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e110")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let unknown_yaml = r##"
scheme:
  system: "tinted9"
  system-version: "0.1.0"
  name: "Bad"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.0.0"
    tinted8-builder: ">=0.0.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("bad.yaml"), unknown_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "Hello\n")?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();

    assert!(err.to_string().contains("E110"));
    Ok(())
}

#[test]
fn e400_no_schemes_found() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e400")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let base16 = r##"
system: "base16"
name: "Test"
slug: "test"
author: "Me"
variant: "dark"
palette:
  base00: "#000000"
  base01: "#111111"
  base02: "#222222"
  base03: "#333333"
  base04: "#444444"
  base05: "#555555"
  base06: "#666666"
  base07: "#777777"
  base08: "#888888"
  base09: "#999999"
  base0A: "#aaaaaa"
  base0B: "#bbbbbb"
  base0C: "#cccccc"
  base0D: "#dddddd"
  base0E: "#eeeeee"
  base0F: "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.1.0"
    tinted8-builder: ">=0.1.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("base16.yaml"), base16)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(
        templates_dir.join("default.mustache"),
        "Hello {{scheme-name}}\n",
    )?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &[schemes], true).unwrap_err();
    let msg = err.to_string();

    assert!(msg.contains("E400"), "expected E400, got: {msg}");
    Ok(())
}

#[test]
fn happy_path_generates_output() -> Result<()> {
    let tmp_dir = unique_tmp_dir("happy_path_generates_output")?;
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");
    let schemes = tmp_dir.join("schemes");
    let scheme_yaml = r##"
scheme:
  system: "tinted8"
  system-version: "0.1.0"
  name: "Test"
  author: "Me"
variant: "dark"
palette:
  black:   "#000000"
  red:     "#ff0000"
  green:   "#00ff00"
  yellow:  "#ffff00"
  blue:    "#0000ff"
  magenta: "#ff00ff"
  cyan:    "#00ffff"
  white:   "#ffffff"
"##;
    let config = r#"
default:
  filename: "out/{{ scheme-system }}-{{ scheme-slug }}.txt"
  supported-systems: [tinted8]
  supports:
    tinted8-styling: ">=0.1.0"
    tinted8-builder: ">=0.1.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(
        templates_dir.join("default.mustache"),
        "Hello {{scheme.name}}\nBlue is #{{palette.blue.normal.hex}}",
    )?;

    tinted_builder_rust::build(&template, &[schemes], true)?;
    let out_path = template.join("out/tinted8-test.txt");
    let out = fs::read_to_string(&out_path)?;

    assert!(out.contains("Hello Test\nBlue is #0000ff"));
    Ok(())
}
