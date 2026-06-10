mod test_utils;
use anyhow::Result;
use std::fs::{self, create_dir_all};
use test_utils::write_to_file;

use crate::test_utils::{run_command, unique_tmp_dir};

#[test]
fn e305_missing_template_config() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e305")?;
    let schemes = tmp_dir.join("schemes");
    let template = tmp_dir.join("template");

    create_dir_all(&schemes)?;
    create_dir_all(&template)?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();
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
  supports:
    styling-spec: "0.2.0"
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
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();
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
  supports:
    styling-spec: "0.2.0"
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
    tinted8-styling: ">=0.2.0"
    tinted8-builder: ">=0.2.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;

    #[allow(clippy::unwrap_used)]
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();
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
  supports:
    styling-spec: "0.2.0"
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
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();

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
  supports:
    styling-spec: "0.2.0"
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
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();

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
  supports:
    styling-spec: "0.2.0"
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
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();

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
  supports:
    styling-spec: "0.2.0"
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
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();

    assert!(err.to_string().contains("E302"));
    Ok(())
}

#[test]
fn non_scheme_files_at_root_are_ignored() -> Result<()> {
    // The schemes-dir root may contain non-scheme files (e.g. `LICENSE`, `README.md`).
    // These must be skipped during discovery rather than surfacing an E111 error.
    let tmp_dir = unique_tmp_dir("non_scheme_root")?;
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
    write_to_file(schemes.join("LICENSE"), "license text")?;
    write_to_file(schemes.join("README.md"), "# readme")?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(templates_dir.join("default.mustache"), "Hello\n")?;

    // No schemes are present, but discovery must not fail on the non-scheme files.
    tinted_builder_rust::build(&template, &schemes, &[], true)?;

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
  supports:
    styling-spec: "0.2.0"
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
    let err = tinted_builder_rust::build(&template, &schemes, &[], true).unwrap_err();

    assert!(err.to_string().contains("E110"));
    Ok(())
}

#[test]
fn w001_no_schemes_found() -> Result<()> {
    let tmp_dir = unique_tmp_dir("e400")?;
    let templates_dir = tmp_dir.join("templates");
    let schemes_path = tmp_dir.join("schemes");
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
    tinted8-styling: ">=0.2.0"
    tinted8-builder: ">=0.2.0"
"#;

    create_dir_all(&schemes_path)?;
    write_to_file(schemes_path.join("base16.yaml"), base16)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(
        templates_dir.join("default.mustache"),
        "Hello {{scheme-name}}\n",
    )?;

    let (_, stderr) = run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        format!("--schemes-dir={}", schemes_path.display()),
        "build".to_string(),
        format!("{}", tmp_dir.display()),
    ])
    .expect("Unable to run command");

    assert!(stderr.contains("W001"), "expected W001, got: {stderr}");
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
  supports:
    styling-spec: "0.2.0"
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
    tinted8-styling: ">=0.2.0"
    tinted8-builder: ">=0.2.0"
"#;

    create_dir_all(&schemes)?;
    write_to_file(schemes.join("test.yaml"), scheme_yaml)?;
    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), config)?;
    write_to_file(
        templates_dir.join("default.mustache"),
        "Hello {{scheme.name}}\nBlue is #{{palette.blue.normal.hex}}",
    )?;

    tinted_builder_rust::build(&template, &schemes, &[], true)?;
    let out_path = template.join("out/tinted8-test.txt");
    let out = fs::read_to_string(&out_path)?;

    assert!(out.contains("Hello Test\nBlue is #0000ff"));
    Ok(())
}

#[test]
fn get_scheme_files_skips_non_scheme_and_hidden_files() -> Result<()> {
    use tinted_builder_rust::operation_build::utils::get_scheme_files;

    let tmp_dir = unique_tmp_dir("walker_lenient")?;
    let schemes = tmp_dir.join("schemes");
    create_dir_all(&schemes)?;

    // Scheme files at the root are discovered...
    write_to_file(schemes.join("good.yaml"), "x")?;
    write_to_file(schemes.join("note.yml"), "x")?;
    // ...while non-scheme files and hidden entries are skipped.
    write_to_file(schemes.join("LICENSE"), "x")?;
    write_to_file(schemes.join("README.md"), "x")?;
    write_to_file(schemes.join(".yamllint.yml"), "x")?;
    create_dir_all(schemes.join(".github").join("workflows"))?;
    write_to_file(
        schemes.join(".github").join("workflows").join("ci.yml"),
        "x",
    )?;

    let files = get_scheme_files(&schemes, &[], true)?;
    let mut names: Vec<String> = files
        .iter()
        .filter_map(|f| f.get_path().file_name()?.to_str().map(String::from))
        .collect();
    names.sort();

    assert_eq!(names, vec!["good.yaml".to_string(), "note.yml".to_string()]);
    Ok(())
}

#[test]
fn get_scheme_files_is_strict_within_scheme_system_dir() -> Result<()> {
    use tinted_builder_rust::operation_build::utils::get_scheme_files;

    let tmp_dir = unique_tmp_dir("walker_strict")?;
    let schemes = tmp_dir.join("schemes");
    let base16 = schemes.join("base16");
    create_dir_all(&base16)?;

    // An unrecognized file directly inside a scheme-system directory must surface an error...
    write_to_file(base16.join("not-a-scheme.txt"), "x")?;
    #[allow(clippy::unwrap_used)]
    let err = get_scheme_files(&schemes, &[], true).unwrap_err();
    assert!(err.to_string().contains("E111"), "got: {err}");

    // ...whereas the same file at the lenient root is simply skipped.
    let tmp_dir = unique_tmp_dir("walker_strict_root")?;
    let schemes = tmp_dir.join("schemes");
    create_dir_all(&schemes)?;
    write_to_file(schemes.join("not-a-scheme.txt"), "x")?;
    assert!(get_scheme_files(&schemes, &[], true)?.is_empty());

    Ok(())
}

#[test]
fn get_scheme_files_by_name_keys_and_filters_by_system() -> Result<()> {
    use tinted_builder::SchemeSystem;
    use tinted_builder_rust::operation_build::utils::get_scheme_files_by_name;

    let tmp_dir = unique_tmp_dir("walker_by_name")?;
    let schemes = tmp_dir.join("schemes");
    create_dir_all(schemes.join("base16"))?;
    create_dir_all(schemes.join("base24"))?;
    write_to_file(schemes.join("base16").join("github.yaml"), "x")?;
    write_to_file(schemes.join("base24").join("dracula.yaml"), "x")?;
    // Root-level files are not part of the `<system>/`-keyed map.
    write_to_file(schemes.join("LICENSE"), "x")?;
    write_to_file(schemes.join("boo-base16.yaml"), "x")?;

    let all = get_scheme_files_by_name(&schemes, None)?;
    assert_eq!(all.len(), 2);
    assert!(all.contains_key("base16-github"));
    assert!(all.contains_key("base24-dracula"));

    let only_base16 = get_scheme_files_by_name(&schemes, Some(SchemeSystem::Base16))?;
    assert_eq!(only_base16.len(), 1);
    assert!(only_base16.contains_key("base16-github"));

    Ok(())
}

#[test]
fn build_renders_system_dir_scheme_despite_root_junk() -> Result<()> {
    // A full build against a `<system>/`-organized schemes dir succeeds even when the root
    // carries non-scheme files and hidden non-scheme YAML.
    let config = fs::read_to_string("./tests/fixtures/templates/base16-config.yaml")?;
    let mustache = fs::read_to_string("./tests/fixtures/templates/base16-template.mustache")?;
    let scheme = fs::read_to_string("./tests/fixtures/schemes/base16/silk-light.yaml")?;

    let tmp_dir = unique_tmp_dir("build_with_junk")?;
    let schemes = tmp_dir.join("schemes");
    let template = tmp_dir.join("template");
    let templates_dir = template.join("templates");

    create_dir_all(schemes.join("base16"))?;
    write_to_file(schemes.join("base16").join("silk-light.yaml"), &scheme)?;
    write_to_file(schemes.join("LICENSE"), "license text")?;
    create_dir_all(schemes.join(".github").join("workflows"))?;
    write_to_file(
        schemes.join(".github").join("workflows").join("ci.yml"),
        "on: push",
    )?;

    create_dir_all(&templates_dir)?;
    write_to_file(templates_dir.join("config.yaml"), &config)?;
    write_to_file(templates_dir.join("base16-template.mustache"), &mustache)?;

    tinted_builder_rust::build(&template, &schemes, &[], true)?;

    let output_dir = template.join("output-themes");
    let rendered_count = fs::read_dir(&output_dir)?.count();
    assert_eq!(rendered_count, 1, "expected the base16 scheme to render");

    Ok(())
}
