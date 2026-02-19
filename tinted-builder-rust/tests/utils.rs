mod test_utils;

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use test_utils::{copy_dir_all, write_to_file};
use tinted_builder::{SchemeSystem, SchemeVariant};
use tinted_builder_rust::operations::build::utils::get_scheme_files;

#[test]
fn test_get_scheme_files_recursive() -> Result<()> {
    // -------
    // Arrange
    // -------
    let name = "test_get_scheme_files_recursive";
    let data_path = PathBuf::from(name);
    let schemes_path = data_path.join("schemes");
    let fixtures_schemes_path = PathBuf::from("./tests/fixtures/schemes");
    if data_path.exists() {
        fs::remove_dir_all(&data_path)?;
    }
    fs::create_dir(&data_path)?;
    copy_dir_all(fixtures_schemes_path, &schemes_path)?;

    // ---
    // Act
    // ---
    let scheme_files = get_scheme_files(&[schemes_path], true)?;
    let mut scheme_names: Vec<String> = vec![];
    let mut scheme_authors: Vec<String> = vec![];
    let mut scheme_variants: Vec<SchemeVariant> = vec![];
    let mut scheme_systems: Vec<SchemeSystem> = vec![];

    for scheme_file in scheme_files {
        // match  scheme_file.get_scheme()? {}
        let scheme = scheme_file.get_scheme()?;
        let name = scheme.get_scheme_name();
        let author = scheme.get_scheme_author();
        let variant = scheme.get_scheme_variant();
        let system = scheme.get_scheme_system();

        scheme_names.push(name);
        scheme_authors.push(author);
        scheme_variants.push(variant);
        scheme_systems.push(system);
    }

    // ------
    // Assert
    // ------

    // base24/dracula.yaml
    assert!(
        scheme_names.contains(&"Dracula".to_string()),
        "scheme_names does not contain the correct name"
    );
    assert!(
        scheme_authors.contains(&"FredHappyface (https://github.com/fredHappyface)".to_string()),
        "scheme_authors does not contain the correct author"
    );
    assert!(
        scheme_variants.contains(&SchemeVariant::Dark),
        "scheme_variants does not contain the correct variant"
    );
    assert!(
        scheme_systems.contains(&SchemeSystem::Base24),
        "scheme_systems does not contain the correct system"
    );

    // base16/silk-light.yaml
    assert!(
        scheme_names.contains(&"Silk Light".to_string()),
        "scheme_names does not contain the correct name"
    );
    assert!(
        scheme_authors.contains(&"Gabriel Fontes ('https://github.com/Misterio77')".to_string()),
        "scheme_authors does not contain the correct author"
    );
    assert!(
        scheme_variants.contains(&SchemeVariant::Light),
        "scheme_variants does not contain the correct variant"
    );
    assert!(
        scheme_systems.contains(&SchemeSystem::Base16),
        "scheme_systems does not contain the correct system"
    );

    Ok(())
}
#[test]
fn test_get_scheme_files_flat() -> Result<()> {
    // -------
    // Arrange
    // -------
    let name = "test_get_scheme_files_flat";
    let data_path = PathBuf::from(name);
    let schemes_dir_path = data_path.join("schemes");
    let schemes_file_path = data_path.join("test-scheme.yaml");
    let fixtures_schemes_path = PathBuf::from("./tests/fixtures/schemes");
    let scheme_name = "Test scheme";
    let scheme_description = "some fancy description";
    let scheme_author = "test author (some url)";
    let scheme_slug = "test-scheme";
    let scheme_variant = SchemeVariant::Dark;
    let scheme_system = SchemeSystem::Base16;
    let scheme_content = format!(
        r#"
system: "{scheme_system}"
description: "{scheme_description}"
name: "{scheme_name}"
slug: "{scheme_slug}"
author: "{scheme_author}"
variant: "{scheme_variant}"
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

"#
    );
    if data_path.exists() {
        fs::remove_dir_all(&data_path)?;
    }
    fs::create_dir(&data_path)?;
    copy_dir_all(fixtures_schemes_path, schemes_dir_path)?;
    write_to_file(&schemes_file_path, &scheme_content)?;

    // ---
    // Act
    // ---
    let scheme_files = get_scheme_files(&[data_path], false)?;
    let scheme_container = scheme_files
        .first()
        .expect("Unable to extract scheme_file")
        .get_scheme()?;

    // ------
    // Assert
    // ------
    assert_eq!(scheme_files.len(), 1);
    assert_eq!(scheme_container.get_scheme_name(), scheme_name);
    assert_eq!(scheme_container.get_scheme_system(), scheme_system);
    assert_eq!(
        scheme_container.get_scheme_description(),
        scheme_description
    );
    assert_eq!(scheme_container.get_scheme_author(), scheme_author);
    assert_eq!(scheme_container.get_scheme_variant(), scheme_variant);
    assert_eq!(scheme_container.get_scheme_slug(), scheme_slug);

    Ok(())
}
