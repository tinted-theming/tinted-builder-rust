mod test_utils;

use anyhow::Result;
use std::fs;
use std::process::Command;
use test_utils::{run_command, unique_tmp_dir, write_to_file};

/// Install - First time sync
#[test]
fn operation_sync_first_time() -> Result<()> {
    // -------
    // Arrange
    // -------
    let tmp_dir = unique_tmp_dir("operation_sync_first_time")?;
    let expected_output = "schemes installed";
    let expected_schemes_path = tmp_dir.join("schemes");
    let expected_git_clone_str = format!("Cloning into '{}/schemes'", tmp_dir.display());
    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir)?;
    }
    fs::create_dir_all(&tmp_dir)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
    ])
    .expect("Unable to run command");
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

#[test]
fn operation_sync_first_time_with_quiet_flag() -> Result<()> {
    // -------
    // Arrange
    // -------
    let tmp_dir = unique_tmp_dir("operation_sync_first_time_with_quiet_flag")?;
    let expected_schemes_path = tmp_dir.join("schemes");
    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir)?;
    }
    fs::create_dir_all(&tmp_dir)?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
        "--quiet".to_string(),
    ])
    .expect("Unable to run command");
    let is_schemes_dir_empty = fs::read_dir(&expected_schemes_path)?.next().is_none();

    // ------
    // Assert
    // ------
    assert!(
        stdout.is_empty(),
        "stdout does not contain the expected output"
    );
    assert!(
        stderr.is_empty(),
        "stderr does not contain the expected output"
    );
    assert!(expected_schemes_path.exists() && !is_schemes_dir_empty,);

    Ok(())
}

// Update - Install has already completed
#[test]
fn operation_sync_update() -> Result<()> {
    // -------
    // Arrange
    // -------
    let tmp_dir = unique_tmp_dir("operation_sync_update")?;
    let expected_output = "schemes up to date";
    let expected_schemes_path = tmp_dir.join("schemes");
    fs::create_dir_all(&tmp_dir)?;
    let command_vec = vec![
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
    ];

    // ---
    // Act
    // ---
    run_command(&command_vec).expect("Unable to run command");
    let (stdout, stderr) = run_command(&command_vec).expect("Unable to run command");
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

#[test]
fn operation_sync_update_with_custom_schemes_dir() -> Result<()> {
    // -------
    // Arrange
    // -------
    let tmp_dir = unique_tmp_dir("operation_sync_update_with_custom_schemes_dir")?;
    let expected_output = "schemes up to date";
    let data_dir = tmp_dir.join("data");
    let expected_schemes_path = data_dir.join("schemes");
    let custom_schemes_path = tmp_dir.join("custom-schemes");
    fs::create_dir_all(&data_dir)?;
    fs::create_dir_all(&custom_schemes_path)?;
    let command_vec = vec![
        format!("--data-dir={}", data_dir.display()),
        format!("--schemes-dir={}", custom_schemes_path.display()),
        "sync".to_string(),
    ];

    // ---
    // Act
    // ---
    run_command(&command_vec).expect("Unable to run command");
    let (stdout, stderr) = run_command(&command_vec).expect("Unable to run command");
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

/// Sync should report uncommitted changes and skip the pull
#[test]
fn operation_sync_uncommitted_changes_skips_pull() -> Result<()> {
    // -------
    // Arrange
    // -------
    let tmp_dir = unique_tmp_dir("operation_sync_uncommitted_changes")?;
    let schemes_path = tmp_dir.join("schemes");
    fs::create_dir_all(&tmp_dir)?;

    // First sync to clone the repo
    run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
    ])
    .expect("Unable to run first sync");

    // Create an uncommitted change in the schemes repo
    let dirty_file = schemes_path.join("dirty-file.txt");
    write_to_file(&dirty_file, "uncommitted change")?;
    Command::new("git")
        .args(["add", "dirty-file.txt"])
        .current_dir(&schemes_path)
        .output()?;

    // ---
    // Act
    // ---
    let (stdout, _stderr) = run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
    ])
    .expect("Unable to run command");

    // ------
    // Assert
    // ------
    assert!(
        stdout.contains("uncommitted changes"),
        "expected uncommitted changes message, got stdout: {stdout}"
    );

    Ok(())
}

/// Sync on a non-git directory should fail with a git error
#[test]
fn operation_sync_pull_on_non_git_directory() -> Result<()> {
    // -------
    // Arrange
    // -------
    let tmp_dir = unique_tmp_dir("operation_sync_pull_non_git")?;
    let schemes_path = tmp_dir.join("schemes");
    // Create a plain directory (not a git repo) where schemes would be
    fs::create_dir_all(&schemes_path)?;

    // ---
    // Act
    // ---
    let (_stdout, stderr) = run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
    ])
    .expect("Unable to run command");

    // ------
    // Assert
    // ------
    assert!(
        !stderr.is_empty(),
        "expected stderr output for non-git directory sync"
    );

    Ok(())
}

/// Sync quiet mode should suppress uncommitted changes message
#[test]
fn operation_sync_uncommitted_changes_quiet() -> Result<()> {
    // -------
    // Arrange
    // -------
    let tmp_dir = unique_tmp_dir("operation_sync_uncommitted_quiet")?;
    let schemes_path = tmp_dir.join("schemes");
    fs::create_dir_all(&tmp_dir)?;

    // First sync to clone the repo
    run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
        "--quiet".to_string(),
    ])
    .expect("Unable to run first sync");

    // Create an uncommitted change
    let dirty_file = schemes_path.join("dirty-file.txt");
    write_to_file(&dirty_file, "uncommitted change")?;
    Command::new("git")
        .args(["add", "dirty-file.txt"])
        .current_dir(&schemes_path)
        .output()?;

    // ---
    // Act
    // ---
    let (stdout, stderr) = run_command(&[
        format!("--data-dir={}", tmp_dir.display()),
        "sync".to_string(),
        "--quiet".to_string(),
    ])
    .expect("Unable to run command");

    // ------
    // Assert
    // ------
    assert!(
        stdout.is_empty(),
        "expected no stdout in quiet mode, got: {stdout}"
    );
    assert!(
        stderr.is_empty(),
        "expected no stderr in quiet mode, got: {stderr}"
    );

    Ok(())
}
