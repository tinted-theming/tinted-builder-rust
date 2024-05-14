mod utils;

use crate::utils::COMMAND_NAME;
use anyhow::Result;
use std::{fs, path::PathBuf};

/// Install - First time sync
#[test]
fn operation_sync_first_time() -> Result<()> {
    // -------
    // Arrange
    // -------
    let name = "test_operation_sync_first_time";
    let expected_output = "schemes installed";
    let expected_schemes_path = PathBuf::from(format!("./{}/schemes", name));
    let expected_data_path = PathBuf::from(name);
    let expected_git_clone_str = format!("Cloning into '{}/schemes'", name);
    if expected_data_path.exists() {
        fs::remove_dir_all(&expected_data_path)?;
        fs::create_dir(expected_data_path)?;
    }

    // ---
    // Act
    // ---
    let (stdout, stderr) = utils::run_command(vec![
        COMMAND_NAME.to_string(),
        format!("--data-dir={}", name),
        "sync".to_string(),
    ])
    .unwrap();
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

/// Update - Install has already completed
#[test]
fn operation_sync_update() -> Result<()> {
    // -------
    // Arrange
    // -------
    let name = "test_operation_sync_update";
    let expected_output = "schemes up to date";
    let expected_schemes_path = PathBuf::from(format!("./{}/schemes", name));
    let command_vec = vec![
        COMMAND_NAME.to_string(),
        format!("--data-dir={}", name),
        "sync".to_string(),
    ];

    // ---
    // Act
    // ---
    utils::run_command(command_vec.clone()).unwrap();
    let (stdout, stderr) = utils::run_command(command_vec).unwrap();
    let is_schemes_dir_empty = fs::read_dir(&expected_schemes_path)?.next().is_none();

    // ------
    // Assert
    // ------
    assert!(
        stdout.contains(expected_output),
        "stdout does not contain the expected output"
    );
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );
    assert!(expected_schemes_path.exists() && !is_schemes_dir_empty,);

    Ok(())
}
