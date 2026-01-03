use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// Helper functions removed - using direct TempDir creation in tests instead

/// Helper to create nexus.toml config
fn create_nexus_config(project_path: &Path, vault_path: &Path) {
    let vault_path_str = vault_path.to_str().unwrap();
    let config_content = format!(
        r#"[project]
name = "test_project"
version = "0.1.0"
obsidian_path = "{}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

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
"#,
        vault_path_str
    );
    fs::write(project_path.join("nexus.toml"), &config_content).unwrap();
}

/// Helper to create heuristics file
fn create_heuristics(project_path: &Path) {
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
    fs::write(
        project_path.join("Gate-Heuristics.json"),
        heuristics_content,
    )
    .unwrap();
}

/// Helper to create a valid planning document
fn create_valid_planning_doc(path: &PathBuf) {
    let content = r#"# Problem
This is a detailed problem statement with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. The problem is clearly defined and articulated with sufficient context and detail to meet all validation criteria and requirements specified in the heuristics configuration file for proper gate validation functionality.

# Vision
This is a comprehensive vision statement with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. The vision describes the desired future state with clarity and precision to meet all validation criteria and requirements specified in the heuristics configuration file for proper gate validation functionality.

# Scope
This is a detailed scope definition with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. The scope clearly outlines what is included and excluded from the project with sufficient detail and context to meet all validation criteria and requirements specified in the heuristics.

# Boundaries
This is a comprehensive boundaries section with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. The boundaries are clearly defined with adequate context and explanation to meet all validation criteria and requirements specified in the heuristics configuration file for proper gate validation.

# Tech Stack
This is a detailed tech stack description with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. All technologies are listed and described with sufficient detail and context to meet all validation criteria and requirements specified in the heuristics configuration file.

# Architecture
This is a comprehensive architecture overview with more than fifty words to ensure it passes the minimum word count validation requirement for our comprehensive testing purposes today. The architecture is described clearly with adequate technical detail and context to meet all validation criteria and requirements specified in the heuristics configuration.
"#;
    fs::write(path, content).unwrap();
}

/// Helper to create a planning doc with issues
fn create_invalid_planning_doc(path: &PathBuf) {
    let content = r#"# Problem
TODO: Fill this in later

# Vision
Not enough words here.
"#;
    fs::write(path, content).unwrap();
}

/// Helper to create a valid dashboard
fn create_valid_dashboard(path: &PathBuf) {
    let content = r#"# Dashboard
- [x] Task 1 completed
- [x] Task 2 completed
- [x] Task 3 completed
"#;
    fs::write(path, content).unwrap();
}

/// Helper to create a dashboard with unchecked items
fn create_invalid_dashboard(path: &PathBuf) {
    let content = r#"# Dashboard
- [x] Task 1 completed
- [ ] Task 2 not done
- [x] Task 3 completed
"#;
    fs::write(path, content).unwrap();
}

#[test]
fn test_gate_passes_with_valid_documents() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Setup project structure
    create_nexus_config(project_path, project_path);
    create_heuristics(project_path);

    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_valid_planning_doc(&planning_dir.join("01-Project-Brief.md"));

    let management_dir = project_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command
    let mut cmd = Command::cargo_bin("nexus").unwrap();
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("MISSION READY"))
        .stdout(predicate::str::contains("Gate is open"));
}

#[test]
fn test_gate_fails_with_invalid_planning_docs() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Setup project structure
    create_nexus_config(project_path, project_path);
    create_heuristics(project_path);

    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_invalid_planning_doc(&planning_dir.join("01-Project-Brief.md"));

    let management_dir = project_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command
    let mut cmd = Command::cargo_bin("nexus").unwrap();
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("GATE CLOSED"))
        .stdout(predicate::str::contains("Fix the issues"));
}

#[test]
fn test_gate_fails_with_unchecked_dashboard() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Setup project structure
    create_nexus_config(project_path, project_path);
    create_heuristics(project_path);

    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_valid_planning_doc(&planning_dir.join("01-Project-Brief.md"));

    let management_dir = project_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_invalid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command
    let mut cmd = Command::cargo_bin("nexus").unwrap();
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("GATE CLOSED"))
        .stdout(predicate::str::contains("unchecked"));
}

#[test]
fn test_gate_shows_context_for_issues() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Setup project structure
    create_nexus_config(project_path, project_path);
    create_heuristics(project_path);

    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_invalid_planning_doc(&planning_dir.join("01-Project-Brief.md"));

    let management_dir = project_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();
    create_valid_dashboard(&management_dir.join("00-START-HERE.md"));

    // Run gate command
    let mut cmd = Command::cargo_bin("nexus").unwrap();
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("TODO"))
        .stdout(predicate::str::contains("line ~"));
}

#[test]
fn test_gate_fails_with_missing_config() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Don't create config file

    // Run gate command
    let mut cmd = Command::cargo_bin("nexus").unwrap();
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read config"));
}
