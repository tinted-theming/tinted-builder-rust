mod utils;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::utils::{write_to_file, COMMAND_NAME};

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
    let name = "test_operation_build_with_sync";
    let template_theme_path = PathBuf::from(format!("./template-{}", name));
    let name = "test_operation_sync_first_time";
    let expected_output = "schemes installed";
    let expected_schemes_path = PathBuf::from(format!("./{}/schemes", name));
    println!("expect: {:?}", expected_schemes_path);
    let expected_data_path = PathBuf::from(name);
    let expected_git_clone_str = format!("Cloning into '{}/schemes'", name);
    if expected_data_path.exists() {
        fs::remove_dir_all(&expected_data_path)?;
        fs::create_dir(expected_data_path)?;
    }

    // ---
    // Act
    // ---
    // Build act
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        format!("--data-dir={}", name),
        "build".to_string(),
        template_theme_path.display().to_string(),
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
    write_to_file(&scheme_file_path, &base16_scheme_file_content)?;
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
        stdout.contains(
            format!(
                "base16 themes generated for \"base16-template\" at \"{}/base16-*.md\"",
                themes_path.display()
            )
            .as_str()
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
    let rendered_theme_path = themes_path.join(format!("base24-{}.md", &scheme_name));
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
                "base24 themes generated for \"base24-template\" at \"{}/base24-*.md\"",
                themes_path.display()
            )
            .as_str()
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
                "base16 themes generated for \"mixed-template\" at \"{}/base16-*.md\"",
                themes_path.display()
            )
            .as_str()
        ),
        "stdout does not contain the exptected output"
    );
    assert!(
        stdout.contains(
            format!(
                "base24 themes generated for \"mixed-template\" at \"{}/base24-*.md\"",
                themes_path.display()
            )
            .as_str()
        ),
        "stdout does not contain the exptected output"
    );

    Ok(())
}
