use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a command instance for nexus_cli
fn nexus_cmd() -> Command {
    Command::cargo_bin("nexus_cli").expect("Failed to find binary")
}

#[test]
fn test_init_creates_project_successfully() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "test_project";
    let project_path = temp_dir.path().join(project_name);

    // Run init command with absolute path
    let mut cmd = nexus_cmd();
    cmd.arg("init").arg(&project_path).assert().success();

    // Verify project folder exists
    assert!(
        project_path.exists(),
        "Project folder should be created at {}",
        project_path.display()
    );

    // Verify nexus.toml exists
    let config_path = project_path.join("nexus.toml");
    assert!(
        config_path.exists(),
        "nexus.toml should exist at {}",
        config_path.display()
    );

    // Verify nexus.toml content
    let config_content = fs::read_to_string(&config_path).expect("Failed to read nexus.toml");
    assert!(
        config_content.contains(&format!("project_name = \"{}\"", project_name)),
        "Config should contain project_name"
    );
    assert!(
        config_content.contains("obsidian_path ="),
        "Config should contain obsidian_path"
    );
    assert!(
        config_content.contains("created_at ="),
        "Config should contain created_at timestamp"
    );
    assert!(
        !config_content.contains("repo_path"),
        "Config should not contain repo_path initially"
    );
    assert!(
        !config_content.contains("current_sprint"),
        "Config should not contain current_sprint initially"
    );

    // Verify template files are copied
    let expected_files = vec![
        "00-START-HERE.md",
        "01-Problem-and-Vision.md",
        "02-Scope-and-Boundaries.md",
        "03-Tech-Stack.md",
        "04-Architecture.md",
        "05-MVP-Breakdown.md",
        "06-PROJECT-UNLOCKED.md",
    ];

    for file in expected_files {
        let file_path = project_path.join(file);
        assert!(file_path.exists(), "Template file {} should exist", file);
    }

    // Verify subdirectories are copied
    assert!(
        project_path.join("decisions").exists(),
        "decisions directory should exist"
    );
    assert!(
        project_path.join("dev-sessions").exists(),
        "dev-sessions directory should exist"
    );

    // Verify template files in subdirectories
    assert!(
        project_path
            .join("decisions/_tech-decision-template.md")
            .exists(),
        "Template files in decisions/ should exist"
    );
    assert!(
        project_path
            .join("dev-sessions/_session-template.md")
            .exists(),
        "Template files in dev-sessions/ should exist"
    );
}

#[test]
fn test_init_fails_when_folder_exists() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "existing_project";
    let project_path = temp_dir.path().join(project_name);

    // Create the folder first
    fs::create_dir(&project_path).expect("Failed to create test folder");

    // Try to init with the same name - should fail
    let mut cmd = nexus_cmd();
    cmd.arg("init")
        .arg(&project_path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_init_displays_success_message() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "success_test";
    let project_path = temp_dir.path().join(project_name);

    // Run init command and check output
    let mut cmd = nexus_cmd();
    cmd.arg("init")
        .arg(&project_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ Created project folder"))
        .stdout(predicate::str::contains("✓ Copied template files"))
        .stdout(predicate::str::contains("✓ Created nexus.toml"))
        .stdout(predicate::str::contains("✅ Project"))
        .stdout(predicate::str::contains("initialized successfully"));
}

#[test]
fn test_init_creates_absolute_path_in_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "absolute_path_test";
    let project_path = temp_dir.path().join(project_name);

    // Run init command
    let mut cmd = nexus_cmd();
    cmd.arg("init").arg(&project_path).assert().success();

    // Read config
    let config_path = project_path.join("nexus.toml");
    let config_content = fs::read_to_string(&config_path).expect("Failed to read nexus.toml");

    // Parse the obsidian_path value
    let obsidian_path_line = config_content
        .lines()
        .find(|line| line.starts_with("obsidian_path"))
        .expect("Should find obsidian_path in config");

    // Verify it's an absolute path (starts with / on Unix)
    assert!(
        obsidian_path_line.contains('/'),
        "obsidian_path should be absolute: {}",
        obsidian_path_line
    );
}
