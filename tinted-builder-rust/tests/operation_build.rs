mod utils;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use utils::COMMAND_NAME;

use crate::utils::write_to_file;
fn setup(system: &str, scheme_name: &str) -> Result<(String, String, String, String)> {
    let config_file_path: PathBuf =
        PathBuf::from(format!("./tests/fixtures/templates/{}-config.yaml", system));
    let scheme_file_path: PathBuf = PathBuf::from(format!(
        "./tests/fixtures/schemes/{}/{}.yaml",
        system, scheme_name
    ));
    let template_file_path: PathBuf = PathBuf::from(format!(
        "./tests/fixtures/templates/{}-template.mustache",
        system
    ));
    let template_rendered_path_fixture: PathBuf = PathBuf::from(format!(
        "./tests/fixtures/rendered/{}-{}.md",
        system, scheme_name
    ));

    Ok((
        fs::read_to_string(&config_file_path).context(format!(
            "Unable to get contents of config: {}",
            config_file_path.display()
        ))?,
        fs::read_to_string(&scheme_file_path).context(format!(
            "Unable to get contents of scheme: {}",
            scheme_file_path.display()
        ))?,
        fs::read_to_string(&template_file_path).context(format!(
            "Unable to get contents of template: {}",
            template_file_path.display()
        ))?,
        fs::read_to_string(&template_rendered_path_fixture).context(format!(
            "Unable to get contents of rendered file: {}",
            template_rendered_path_fixture.display()
        ))?,
    ))
}

/// Tests schemes/*.yaml generation with base16 system
#[test]
fn test_operation_build_quiet_flag() -> Result<()> {
    // -------
    // Arrange
    // -------
    let scheme_name = "silk-light";
    let system = "base16";
    let name = "operation_build_quiet_flag";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let template_mustache_path = template_templates_path.join("base16-template.mustache");
    let schemes_path = template_theme_path.join("schemes");
    let scheme_file_path = schemes_path.join(format!("{}.yaml", &scheme_name));
    let themes_path = template_theme_path.join("output-themes");
    let rendered_theme_path = themes_path.join(format!("base16-{}.md", &scheme_name));
    let (
        config_file_content,
        scheme_file_content,
        template_file_content,
        template_rendered_content_fixture,
    ) = setup(system, scheme_name)?;

    if themes_path.is_dir() {
        fs::remove_dir_all(&themes_path)?;
    }
    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&scheme_file_path, &scheme_file_content)?;
    write_to_file(&template_config_path, &config_file_content)?;
    write_to_file(&template_mustache_path, &template_file_content)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
        "build".to_string(),
        template_theme_path.display().to_string(),
        "--quiet".to_string(),
    ])
    .unwrap();
    let rendered_content = fs::read_to_string(rendered_theme_path)?;

    // ------
    // Assert
    // ------
    assert_eq!(rendered_content, template_rendered_content_fixture);
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );
    assert!(
        stdout.is_empty(),
        "stdout does not contain the exptected output"
    );

    Ok(())
}

#[test]
fn test_operation_build_with_sync() -> Result<()> {
    // -------
    // Arrange
    // -------
    let name = "operation_build_with_sync";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let expected_output = "schemes installed";
    let expected_schemes_path =
        PathBuf::from(format!("./{}/schemes", template_theme_path.display()));
    let expected_data_path = PathBuf::from(&template_theme_path);
    let expected_git_clone_str =
        format!("Cloning into '{}/schemes'", template_theme_path.display());
    if expected_data_path.exists() {
        fs::remove_dir_all(&expected_data_path)?;
    }
    fs::create_dir(expected_data_path)?;

    // ---
    // Act
    // ---
    // Build act
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        format!("--data-dir={}", template_theme_path.display()),
        "build".to_string(),
        name.to_string(),
        "--sync".to_string(),
    ])
    .unwrap();

    // Sync act
    let is_schemes_dir_empty = fs::read_dir(&expected_schemes_path)?.next().is_none();

    // ------
    // Assert
    // ------
    assert!(
        stdout.contains(expected_output),
        "stdout does not contain the expected output"
    );
    assert!(
        stderr.contains(&expected_git_clone_str),
        "stderr does not contain the expected output"
    );
    assert!(expected_schemes_path.exists() && !is_schemes_dir_empty,);

    Ok(())
}

/// Tests schemes/*.yaml generation with base16 system
#[test]
fn test_operation_build_base16() -> Result<()> {
    // -------
    // Arrange
    // -------
    let scheme_name = "silk-light";
    let system = "base16";
    let name = "operation_build_base16";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let template_mustache_path = template_templates_path.join("base16-template.mustache");
    let schemes_path = template_theme_path.join("schemes");
    let scheme_file_path = schemes_path.join(format!("{}.yaml", &scheme_name));
    let themes_path = template_theme_path.join("output-themes");
    let rendered_theme_path = themes_path.join(format!("base16-{}.md", &scheme_name));
    // Ensure dotfile yaml/yml files are ignored as scheme files
    let hidden_yaml_path = schemes_path.join(".hidden.yaml");
    // Ensure base24 scheme is not built
    let base24_schemes_path = schemes_path.join("base24");
    let base24_scheme_content = fs::read_to_string(PathBuf::from(
        "./tests/fixtures/schemes/base24/dracula.yaml",
    ))?;
    let base24_scheme_file_path: PathBuf = schemes_path.join("base24/dracula.yaml");
    let base24_theme_output_file = themes_path.join("base24-dracula.md");

    let (
        base16_config_file_content,
        base16_scheme_file_content,
        base16_template_file_content,
        base16_template_rendered_content_fixture,
    ) = setup(system, scheme_name)?;

    if themes_path.is_dir() {
        fs::remove_dir_all(&themes_path)?;
    }
    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    fs::create_dir(base24_schemes_path)?;
    write_to_file(&scheme_file_path, &base16_scheme_file_content)?;
    write_to_file(&base24_scheme_file_path, &base24_scheme_content)?;
    write_to_file(&hidden_yaml_path, "content: invalid")?;
    write_to_file(&template_config_path, &base16_config_file_content)?;
    write_to_file(&template_mustache_path, &base16_template_file_content)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();
    let rendered_content = fs::read_to_string(rendered_theme_path)?;

    // ------
    // Assert
    // ------
    assert_eq!(rendered_content, base16_template_rendered_content_fixture);
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );
    assert!(
        !&base24_theme_output_file.is_file(),
        "file should not exist: {}",
        base24_theme_output_file.display()
    );
    assert!(
        &hidden_yaml_path.is_file(),
        "file does not exist: {}",
        hidden_yaml_path.display()
    );
    assert!(
        stdout.contains(
            format!(
                "Successfully generated \"base16\" themes for \"base16-template\" with filename \"{}\"",
                themes_path
                    .join("{{ scheme-system }}-{{ scheme-slug }}.md")
                    .display()
            ).as_str()
        ),
        "stdout does not contain the exptected output"
    );

    Ok(())
}

/// Tests schemes/*.yaml generation with base24 system
#[test]
fn test_operation_build_base24() -> Result<()> {
    // -------
    // Arrange
    // -------
    let scheme_name = "dracula";
    let system = "base24";
    let name = "operation_build_base24";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let template_mustache_path = template_templates_path.join("base24-template.mustache");
    let schemes_path = template_theme_path.join("schemes");
    let scheme_file_path = schemes_path.join(format!("{}.yaml", &scheme_name));
    let themes_path = template_theme_path.join("output-themes");
    let output_extension = "-custom-extension";
    let rendered_theme_path =
        themes_path.join(format!("base24-{}{}", &scheme_name, &output_extension));
    let (
        base24_config_file_content,
        base24_scheme_file_content,
        base24_template_file_content,
        base24_template_rendered_content_fixture,
    ) = setup(system, scheme_name)?;

    if themes_path.is_dir() {
        fs::remove_dir_all(&themes_path)?;
    }
    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&scheme_file_path, &base24_scheme_file_content)?;
    write_to_file(&template_config_path, &base24_config_file_content)?;
    write_to_file(&template_mustache_path, &base24_template_file_content)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();
    let rendered_content = fs::read_to_string(rendered_theme_path)?;

    // ------
    // Assert
    // ------
    assert_eq!(rendered_content, base24_template_rendered_content_fixture);
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );
    assert!(
        stdout.contains(
            format!(
                "Successfully generated \"base24\" themes for \"base24-template\" with filename \"{}\"",
                themes_path
                    .join(format!("{{{{ scheme-system }}}}-{{{{ scheme-slug }}}}{}", output_extension))
                    .display()
            ).as_str()
        ),
        "stdout does not contain the exptected output"
    );

    Ok(())
}

/// Tests schemes/base16/*.yaml and schemes/base24/*.yaml generation
#[test]
fn test_operation_build_mixed() -> Result<()> {
    // -------
    // Arrange
    // -------
    let name = "operation_build_mixed";
    let base16_scheme_name = "silk-light";
    let base24_scheme_name = "dracula";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let base24_template_mustache_path = template_templates_path.join("mixed-template.mustache");
    let schemes_path = template_theme_path.join("schemes");
    let base16_schemes_path = schemes_path.join("base16");
    let base24_schemes_path = schemes_path.join("base24");
    let base16_scheme_file_path = base16_schemes_path.join(format!("{}.yaml", &base16_scheme_name));
    let base24_scheme_file_path = base24_schemes_path.join(format!("{}.yaml", &base24_scheme_name));
    let themes_path = template_theme_path.join("output-themes");
    let base16_rendered_theme_path = themes_path.join(format!("base16-{}.md", &base16_scheme_name));
    let base24_rendered_theme_path = themes_path.join(format!("base24-{}.md", &base24_scheme_name));
    let base16_template_rendered_content_fixture = fs::read_to_string(format!(
        "./tests/fixtures/rendered/base16-mixed-{}.md",
        base16_scheme_name
    ))?;
    let (_, base16_scheme_file_content, _, _) = setup("base16", base16_scheme_name)?;
    let (
        _,
        base24_scheme_file_content,
        base24_template_file_content,
        base24_template_rendered_content_fixture,
    ) = setup("base24", base24_scheme_name)?;

    if themes_path.is_dir() {
        fs::remove_dir_all(&themes_path)?;
    }
    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&base16_schemes_path)?;
    fs::create_dir(&base24_schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&base16_scheme_file_path, &base16_scheme_file_content)?;
    write_to_file(&base24_scheme_file_path, &base24_scheme_file_content)?;
    write_to_file(
        &template_config_path,
        fs::read_to_string("./tests/fixtures/templates/mixed-config.yaml")?.as_str(),
    )?;
    write_to_file(
        &base24_template_mustache_path,
        &base24_template_file_content,
    )?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();
    let base16_rendered_content = fs::read_to_string(base16_rendered_theme_path)?;
    let base24_rendered_content = fs::read_to_string(base24_rendered_theme_path)?;

    // ------
    // Assert
    // ------
    assert_eq!(
        base16_rendered_content,
        base16_template_rendered_content_fixture
    );
    assert_eq!(
        base24_rendered_content,
        base24_template_rendered_content_fixture
    );
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );
    assert!(
        stdout.contains(
            format!(
                "Successfully generated \"base16, base24\" themes for \"mixed-template\" with filename \"{}\"",
                themes_path
                    .join("{{ scheme-system }}-{{ scheme-slug }}.md")
                    .display()
            ).as_str()
        ),
        "stdout does not contain the exptected output"
    );

    Ok(())
}

/// Tests error message when invalid scheme system is provided in config.yaml
#[test]
fn test_operation_build_invalid_system() -> Result<()> {
    // -------
    // Arrange
    // -------
    let system = "invalid-system";
    let name = "operation_build_invalid_system";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let schemes_path = template_theme_path.join("schemes");
    let base16_config_file_content = format!(
        r#"
invalid:
  filename: output-themes/{{ scheme-system }}-{{ scheme-slug }}.md
  supported-systems: [{}]"#,
        system
    );

    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&template_config_path, &base16_config_file_content)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();

    // ------
    // Assert
    // ------
    assert!(
        stderr.contains(format!("unknown variant `{}`", system).as_str()),
        "stderr does not contain the expected output"
    );
    assert!(
        stdout.is_empty(),
        "stdout does not contain the exptected output"
    );

    Ok(())
}

/// Tests a base16 scheme with a missing palette.base00 property
#[test]
fn test_operation_build_base16_missing_base00() -> Result<()> {
    // -------
    // Arrange
    // -------
    let scheme_name = "invalid";
    let system = "base16";
    let name = "operation_build_base16_missing_base00";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let template_mustache_path = template_templates_path.join("base16-template.mustache");
    let schemes_path = template_theme_path.join("schemes");
    let scheme_file_path = schemes_path.join(format!("{}.yaml", &scheme_name));
    let themes_path = template_theme_path.join("output-themes");
    let scheme_file_content = r#"
system: "base16"
name: "UwUnicorn"
author: "Fernando Marques (https://github.com/RakkiUwU) and Gabriel Fontes (https://github.com/Misterio77)"
variant: "dark"
palette:
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
  base0F: "a3a079"
"#;

    if themes_path.is_dir() {
        fs::remove_dir_all(&themes_path)?;
    }
    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&scheme_file_path, scheme_file_content)?;

    let base16_config_file_content = fs::read_to_string(PathBuf::from(format!(
        "./tests/fixtures/templates/{}-config.yaml",
        system
    )))?;
    let base16_template_file_content = fs::read_to_string(PathBuf::from(format!(
        "./tests/fixtures/templates/{}-template.mustache",
        system
    )))?;
    write_to_file(&template_config_path, &base16_config_file_content)?;
    write_to_file(&template_mustache_path, &base16_template_file_content)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();

    // ------
    // Assert
    // ------
    assert!(
        stderr.contains("base16 scheme does not contain the required palette properties"),
        "stderr does not contain the expected output"
    );
    assert!(
        stdout.is_empty(),
        "stdout does not contain the exptected output"
    );

    Ok(())
}

/// Tests schemes/invalid.yaml prints error to stderr
#[test]
fn test_operation_build_invalid_base16() -> Result<()> {
    // -------
    // Arrange
    // -------
    let scheme_name = "invalid";
    let system = "base16";
    let name = "operation_build_invalid_base16";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let template_mustache_path = template_templates_path.join("base16-template.mustache");
    let schemes_path = template_theme_path.join("schemes");
    let scheme_file_path = schemes_path.join(format!("{}.yaml", &scheme_name));
    let themes_path = template_theme_path.join("output-themes");

    if themes_path.is_dir() {
        fs::remove_dir_all(&themes_path)?;
    }
    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&scheme_file_path, "content: invalid")?;

    let base16_config_file_content = fs::read_to_string(PathBuf::from(format!(
        "./tests/fixtures/templates/{}-config.yaml",
        system
    )))?;
    let base16_template_file_content = fs::read_to_string(PathBuf::from(format!(
        "./tests/fixtures/templates/{}-template.mustache",
        system
    )))?;
    write_to_file(&template_config_path, &base16_config_file_content)?;
    write_to_file(&template_mustache_path, &base16_template_file_content)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();

    // ------
    // Assert
    // ------
    assert!(
        stderr.contains(
            format!(
                r#"Error: Unable to deserialize scheme "{}": missing field `system`"#,
                scheme_file_path.display()
            )
            .as_str()
        ),
        "stderr does not contain the expected output"
    );
    assert!(
        stdout.is_empty(),
        "stdout does not contain the exptected output"
    );

    Ok(())
}

/// Tests deprecated error messages when "output" or "extension" config properties are used
#[test]
fn test_operation_build_with_deprecated_config_properties() -> Result<()> {
    // -------
    // Arrange
    // -------
    let system = "base16";
    let scheme_name = "silk-light";
    let name = "operation_build_with_deprecated_config_properties";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let template_templates_path = template_theme_path.join("templates");
    let template_config_path = template_templates_path.join("config.yaml");
    let schemes_path = template_theme_path.join("schemes");
    let scheme_file_path = schemes_path.join(format!("{}.yaml", &scheme_name));
    let template_mustache_path = template_templates_path.join("base16-template.mustache");
    let themes_path = template_theme_path.join("output-themes");
    let rendered_theme_path = themes_path.join(format!("{}-{}.md", &system, &scheme_name));
    let base16_config_file_content = r#"
base16-template:
  output: output-themes
  extension: .md"#;
    let (_, scheme_file_content, template_file_content, base16_template_rendered_content_fixture) =
        setup(system, scheme_name)?;

    if template_theme_path.is_dir() {
        fs::remove_dir_all(&template_theme_path)?;
    }
    fs::create_dir(&template_theme_path)?;
    fs::create_dir(&schemes_path)?;
    fs::create_dir(&template_templates_path)?;
    write_to_file(&template_config_path, base16_config_file_content)?;
    write_to_file(&template_mustache_path, &template_file_content)?;
    write_to_file(&scheme_file_path, &scheme_file_content)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        "build".to_string(),
        template_theme_path.display().to_string(),
        format!("--schemes-dir={}", schemes_path.display()),
    ])
    .unwrap();
    let rendered_content = fs::read_to_string(rendered_theme_path)?;
    assert_eq!(rendered_content, base16_template_rendered_content_fixture);

    // ------
    // Assert
    // ------
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );
    assert!(
        stdout.contains(
            "Warning: \"output\" is a deprecated config property, use \"filename\" instead."
        ),
        "stdout does not contain the exptected output"
    );
    assert!(
        stdout.contains(
            "Warning: \"extension\" is a deprecated config property, use \"filename\" instead."
        ),
        "stdout does not contain the exptected output"
    );

    Ok(())
}
