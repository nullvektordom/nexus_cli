//! Stress Tests for Gate Command - Chaos Engineering
//!
//! Tests the gate command's resilience to edge cases, malformed data,
//! and system errors.

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper to create nexus.toml config
fn create_nexus_config(project_path: &Path, vault_path: &Path) {
    let vault_path_str = vault_path.to_str().unwrap();
    let config_content = format!(
        r#"[project]
name = "test_project"
version = "0.1.0"
obsidian_path = "{vault_path_str}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[state]
is_unlocked = false

[state.active_sprint]
current = "sprint-1"
status = "in_progress"

[templates]
claude_template = "templates/CLAUDE.md.example"
"#
    );
    fs::write(project_path.join("nexus.toml"), &config_content).unwrap();
}

/// Helper to create heuristics file
fn create_heuristics(vault_path: &Path) {
    let heuristics_content = r#"{
  "min_section_length": 50,
  "required_headers": [
    "Problem",
    "Vision",
    "Scope",
    "Boundaries",
    "Tech Stack",
    "Architecture"
  ],
  "illegal_strings": [
    "TODO",
    "FIXME",
    "TBD",
    "...",
    "insert here",
    "fill me in",
    "[ ]"
  ],
  "management_files": {
    "dashboard": "00-START-HERE.md",
    "require_all_checked": true
  }
}
"#;
    fs::write(vault_path.join("Gate-Heuristics.json"), heuristics_content).unwrap();
}

/// Helper to create a valid dashboard
fn create_valid_dashboard(path: &PathBuf) {
    let content = r"# Dashboard
- [x] Task 1 completed
- [x] Task 2 completed
";
    fs::write(path, content).unwrap();
}

// ============================================================================
// STRESS TEST 1: THE "GHOST" VAULT
// ============================================================================

#[test]
fn test_stress_ghost_vault_missing_path() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create config pointing to non-existent vault
    let ghost_vault = project_path.join("nonexistent_vault");
    create_nexus_config(project_path, &ghost_vault);

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Obsidian vault not found"))
        .stderr(predicate::str::contains("Check the 'obsidian_path'"));
}

#[test]
fn test_stress_ghost_vault_is_file_not_directory() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create a file where vault should be a directory
    let fake_vault = project_path.join("fake_vault.txt");
    fs::write(&fake_vault, "this is a file, not a directory").unwrap();

    create_nexus_config(project_path, &fake_vault);

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not a directory"));
}

// ============================================================================
// STRESS TEST 2: THE "EMPTY" VAULT
// ============================================================================

#[test]
fn test_stress_empty_vault_no_planning_docs() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());
    create_heuristics(&vault_path.clone());

    // Create empty planning directory
    let planning_dir = vault_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Create valid dashboard
    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("No planning documents found"))
        .stdout(predicate::str::contains("GATE CLOSED"));
}

#[test]
fn test_stress_empty_vault_missing_dashboard() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());
    create_heuristics(&vault_path.clone());

    // Create empty management directory (no dashboard)
    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();

    // Create empty planning directory
    let planning_dir = vault_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Dashboard file not found"));
}

// ============================================================================
// STRESS TEST 3: MALFORMED UTF-8
// ============================================================================

#[test]
fn test_stress_malformed_utf8_in_planning_doc() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());
    create_heuristics(&vault_path.clone());

    let planning_dir = vault_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Write binary data to a .md file (invalid UTF-8)
    let bad_file = planning_dir.join("corrupted.md");
    fs::write(&bad_file, [0xFF, 0xFE, 0xFD, 0x00, 0x80, 0x81]).unwrap();

    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("corrupted.md"))
        .stdout(
            predicate::str::contains("Failed to read file").or(predicate::str::contains("UTF-8")),
        );
}

#[test]
fn test_stress_malformed_utf8_in_dashboard() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());
    create_heuristics(&vault_path.clone());

    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();

    // Write binary data to dashboard
    let dashboard = management_dir.join("00-START-HERE.md");
    fs::write(&dashboard, [0xFF, 0xFE, 0xFD, 0x00]).unwrap();

    let planning_dir = vault_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("DASHBOARD").or(predicate::str::contains("dashboard")))
        .stdout(
            predicate::str::contains("Failed to read dashboard")
                .or(predicate::str::contains("UTF-8")),
        );
}

// ============================================================================
// STRESS TEST 4: PERMISSION DENIED
// ============================================================================

#[test]
#[cfg(unix)] // Permission tests only work on Unix-like systems
fn test_stress_permission_denied_planning_doc() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());
    create_heuristics(&vault_path.clone());

    let planning_dir = vault_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Create file with no read permissions
    let locked_file = planning_dir.join("locked.md");
    fs::write(&locked_file, "# Problem\nSome content").unwrap();
    let mut perms = fs::metadata(&locked_file).unwrap().permissions();
    perms.set_mode(0o000); // Remove all permissions
    fs::set_permissions(&locked_file, perms).unwrap();

    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    let result = cmd.assert().failure();

    // Clean up permissions before temp_dir cleanup
    let mut perms = fs::metadata(&locked_file).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&locked_file, perms).unwrap();

    result.stdout(predicate::str::contains("locked.md")).stdout(
        predicate::str::contains("Failed to read file")
            .or(predicate::str::contains("Permission denied")),
    );
}

// ============================================================================
// STRESS TEST 5: MISSING HEURISTICS FILE
// ============================================================================

#[test]
fn test_stress_missing_heuristics_file() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());

    // Don't create heuristics file

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Heuristics file not found"))
        .stderr(predicate::str::contains("Check the 'heuristics_file'"));
}

// ============================================================================
// STRESS TEST 6: LARGE FILE HANDLING
// ============================================================================

#[test]
fn test_stress_large_markdown_file() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());
    create_heuristics(&vault_path.clone());

    let planning_dir = vault_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Create a large file (150MB of valid markdown)
    let large_file = planning_dir.join("mega.md");
    let mut content = String::from("# Problem\n");
    // Generate 150MB of text
    for i in 0..3_000_000 {
        content.push_str(&format!(
            "This is line number {i} with some padding text to make it longer. "
        ));
        if i % 20 == 0 {
            content.push('\n');
        }
    }
    fs::write(&large_file, content).unwrap();

    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command - should warn about large file but process it
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure() // Will fail due to missing headers, but shouldn't crash
        .stdout(predicate::str::contains("mega.md"))
        .stdout(predicate::str::contains("File very large"));
}

// ============================================================================
// STRESS TEST 7: MALFORMED HEURISTICS JSON
// ============================================================================

#[test]
fn test_stress_malformed_heuristics_json() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());

    // Write malformed JSON to heuristics file
    let bad_json = r#"{
  "min_section_length": "not a number",
  "required_headers": "not an array"
  missing comma and bracket
"#;
    fs::write(vault_path.join("Gate-Heuristics.json"), bad_json).unwrap();

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse heuristics"));
}

// ============================================================================
// STRESS TEST 8: EMPTY PLANNING DIRECTORY (NOT MISSING, JUST EMPTY)
// ============================================================================

#[test]
fn test_stress_planning_dir_with_non_md_files() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let vault_path = project_path.join("vault");

    fs::create_dir_all(&vault_path).unwrap();
    create_nexus_config(project_path, &vault_path.clone());
    create_heuristics(&vault_path.clone());

    let planning_dir = vault_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Add non-markdown files (should be ignored)
    fs::write(planning_dir.join("notes.txt"), "some text").unwrap();
    fs::write(planning_dir.join("image.png"), [0x89, 0x50, 0x4E, 0x47]).unwrap();
    fs::create_dir_all(planning_dir.join("subfolder")).unwrap();

    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command - should report no planning documents found
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("No planning documents found"));
}
