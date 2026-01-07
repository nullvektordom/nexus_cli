use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a command instance for `nexus_cli`
fn nexus_cmd() -> Command {
    cargo_bin_cmd!("nexus")
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

    // Verify nexus.toml content (new structured format)
    let config_content = fs::read_to_string(&config_path).expect("Failed to read nexus.toml");
    assert!(
        config_content.contains("[project]"),
        "Config should contain [project] section"
    );
    assert!(
        config_content.contains(&format!("name = \"{project_name}\"")),
        "Config should contain project name"
    );
    assert!(
        config_content.contains("obsidian_path ="),
        "Config should contain obsidian_path"
    );
    assert!(
        config_content.contains("[structure]"),
        "Config should contain [structure] section"
    );
    assert!(
        config_content.contains("[gate]"),
        "Config should contain [gate] section"
    );
    assert!(
        config_content.contains("[obsidian]"),
        "Config should contain [obsidian] section"
    );
    assert!(
        config_content.contains("planning_path ="),
        "Config should contain planning_path"
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
        assert!(file_path.exists(), "Template file {file} should exist");
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
        "obsidian_path should be absolute: {obsidian_path_line}"
    );
}

#[test]
fn test_init_adhoc_mode_creates_correct_structure() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "adhoc_test";
    let project_path = temp_dir.path().join(project_name);

    // Run init command with --mode adhoc
    let mut cmd = nexus_cmd();
    cmd.arg("init")
        .arg(&project_path)
        .arg("--mode")
        .arg("adhoc")
        .assert()
        .success();

    // Verify project folder exists (the repo)
    assert!(project_path.exists(), "Repo folder should be created");

    // Verify nexus.toml exists in repo and contains adhoc mode
    let config_path = project_path.join("nexus.toml");
    assert!(config_path.exists(), "nexus.toml should exist in repo");

    let config_content = fs::read_to_string(&config_path).expect("Failed to read nexus.toml");
    assert!(
        config_content.contains("[tasks]"),
        "Config should contain [tasks] section"
    );
    assert!(
        config_content.contains("mode = \"adhoc\""),
        "Config should have adhoc mode"
    );
    assert!(
        config_content.contains("obsidian_path"),
        "Config should have obsidian_path"
    );

    // Extract obsidian_path from config to verify vault structure
    let obsidian_path_line = config_content
        .lines()
        .find(|line| line.starts_with("obsidian_path"))
        .expect("Should find obsidian_path");

    let obsidian_path = obsidian_path_line
        .split('=')
        .nth(1)
        .expect("Should split obsidian_path")
        .trim()
        .trim_matches('"');

    let obsidian_vault = std::path::PathBuf::from(obsidian_path);

    // Verify adhoc directory structure exists in Obsidian vault
    let management_dir = obsidian_vault.join("00-MANAGEMENT");
    assert!(
        management_dir.exists(),
        "00-MANAGEMENT should exist in Obsidian vault"
    );

    let planning_dir = management_dir.join("adhoc-planning");
    assert!(
        planning_dir.exists(),
        "adhoc-planning should exist in Obsidian vault"
    );

    // Verify adhoc planning files exist in Obsidian vault
    let expected_files = vec![
        "Task-Capture.md",
        "Task-Approach.md",
        "Task-Validation.md",
    ];

    for file in expected_files {
        let file_path = planning_dir.join(file);
        assert!(
            file_path.exists(),
            "Planning file {file} should exist in Obsidian vault"
        );
    }

    // Verify dashboard exists in Obsidian vault
    let dashboard = management_dir.join("00-ADHOC-TASK.md");
    assert!(
        dashboard.exists(),
        "Dashboard should exist in Obsidian vault"
    );

    // Verify dashboard content
    let dashboard_content = fs::read_to_string(&dashboard).expect("Failed to read dashboard");
    assert!(
        dashboard_content.contains("Planning Phase"),
        "Dashboard should contain Planning Phase section"
    );

    // Cleanup: remove the Obsidian vault directory
    let _ = fs::remove_dir_all(&obsidian_vault);
}

#[test]
fn test_init_default_mode_is_sprint() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "default_mode_test";
    let project_path = temp_dir.path().join(project_name);

    // Run init command without --mode flag
    let mut cmd = nexus_cmd();
    cmd.arg("init").arg(&project_path).assert().success();

    // Verify sprint structure (01-05 templates)
    assert!(
        project_path.join("01-Problem-and-Vision.md").exists(),
        "Sprint templates should exist"
    );

    // Verify no adhoc structure
    assert!(
        !project_path.join("00-MANAGEMENT/adhoc-planning").exists(),
        "Adhoc planning dir should NOT exist in sprint mode"
    );

    // Verify config does NOT have [tasks] section with adhoc mode
    let config_path = project_path.join("nexus.toml");
    let config_content = fs::read_to_string(&config_path).expect("Failed to read nexus.toml");
    assert!(
        !config_content.contains("mode = \"adhoc\""),
        "Config should NOT have adhoc mode by default"
    );
}

#[test]
fn test_init_adhoc_displays_correct_message() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "adhoc_message_test";
    let project_path = temp_dir.path().join(project_name);

    // Run init command and check output
    let mut cmd = nexus_cmd();
    let output = cmd
        .arg("init")
        .arg(&project_path)
        .arg("--mode")
        .arg("adhoc")
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ Created Obsidian vault structure"))
        .stdout(predicate::str::contains("✓ Copied planning templates to Obsidian vault"))
        .stdout(predicate::str::contains("✓ Created task dashboard in Obsidian vault"))
        .stdout(predicate::str::contains("✓ Created nexus.toml in repo"))
        .stdout(predicate::str::contains("✅ Adhoc task"))
        .stdout(predicate::str::contains("nexus gate"))
        .get_output()
        .clone();

    // Extract obsidian path and cleanup
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(line) = stdout.lines().find(|l| l.contains("Obsidian vault:"))
        && let Some(path) = line.split("Obsidian vault:").nth(1) {
            let vault_path = std::path::PathBuf::from(path.trim());
            let _ = fs::remove_dir_all(&vault_path);
    }
}

#[test]
fn test_init_invalid_mode_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let project_name = "invalid_mode_test";
    let project_path = temp_dir.path().join(project_name);

    // Try to init with invalid mode
    let mut cmd = nexus_cmd();
    cmd.arg("init")
        .arg(&project_path)
        .arg("--mode")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid mode"));
}
