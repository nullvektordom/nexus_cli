//! Integration tests for Sprint command

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper to create nexus.toml config
fn create_nexus_config(
    project_path: &Path,
    vault_path: &Path,
    sprint_status: Option<(&str, &str)>,
) {
    let vault_path_str = vault_path.to_str().unwrap();
    let active_sprint_section = if let Some((current, status)) = sprint_status {
        format!(
            r#"
[state.active_sprint]
current = "{current}"
status = "{status}"
"#
        )
    } else {
        String::new()
    };

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
is_unlocked = true
{active_sprint_section}
[templates]
claude_template = "templates/CLAUDE.md.example"
"#
    );
    fs::write(project_path.join("nexus.toml"), &config_content).unwrap();
}

/// Helper to create MVP breakdown file with test sprints
fn create_mvp_breakdown(planning_dir: &Path) {
    let mvp_content = r"# MVP broken into sprints

## Sprint 0: Setup (day 1)
- [x] Create nexus repo with Cargo.toml
- [x] Add clap with derive feature
**Exit criteria:** All commands parse

## Sprint 1: Init Command (days 2-3)
_Focus: Configuration and initialization._

- [x] Implement config.rs
- [x] Implement init command
**Exit criteria:** `nexus init` creates project

## Sprint 2: The Gatekeeper
_Focus: Validation logic._

- [x] Implement heuristics engine
- [x] Implement planning validation
**Exit criteria:** `nexus gate` blocks on incomplete docs

## Sprint 3: The Unlock
_Focus: Generating CLAUDE.md._

- [x] Integrate Tera templating
- [x] Generate CLAUDE.md
**Exit criteria:** `nexus unlock` creates CLAUDE.md

## Sprint 4: The Sprint Orchestrator
_Focus: Creating the Tactical Staging Area._

- [ ] MVP Parser: Extract specific sprint tasks
- [ ] Branching Logic: Use the git2 crate
- [ ] Obsidian Scaffolding: Create sprint folders
**Exit criteria:** `nexus sprint X` creates branch and workspace
";

    fs::write(planning_dir.join("05-MVP-Breakdown.md"), mvp_content).unwrap();
}

/// Helper to initialize a test git repository and commit all files
fn init_test_git_repo(repo_path: &PathBuf) {
    // Initialize repo
    let repo = git2::Repository::init(repo_path).unwrap();

    // Stage all files (including nexus.toml if it exists)
    let mut index = repo.index().unwrap();
    index
        .add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();

    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    // Create initial commit
    let signature = git2::Signature::now("Test User", "test@example.com").unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )
    .unwrap();
}

#[test]
fn test_sprint_command_creates_branch_and_folders() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create initial files BEFORE git init
    create_nexus_config(project_path, project_path, None);

    // Create planning directory with MVP breakdown
    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_mvp_breakdown(&planning_dir);

    // Create a README so we have something to commit
    fs::write(project_path.join("README.md"), "# Test Project\n").unwrap();

    // Setup git repository and commit all files
    init_test_git_repo(&project_path.to_path_buf());

    // Run sprint command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("sprint").arg(project_path).arg("4");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("SPRINT READY"))
        .stdout(predicate::str::contains("Sprint 4"))
        .stdout(predicate::str::contains("The Sprint Orchestrator"));

    // Verify branch was created
    let repo = git2::Repository::open(project_path).unwrap();
    let branch = repo.find_branch("sprint-4-the-sprint-orchestrator", git2::BranchType::Local);
    assert!(branch.is_ok(), "Branch should be created");

    // Verify we're on the new branch
    let head = repo.head().unwrap();
    assert_eq!(
        head.shorthand(),
        Some("sprint-4-the-sprint-orchestrator"),
        "Should be on new branch"
    );

    // Verify sprint folder was created
    let sprint_folder = project_path
        .join("00-MANAGEMENT")
        .join("sprints")
        .join("sprint-4-the-sprint-orchestrator");
    assert!(sprint_folder.exists(), "Sprint folder should exist");

    // Verify Tasks.md was created
    let tasks_file = sprint_folder.join("Tasks.md");
    assert!(tasks_file.exists(), "Tasks.md should exist");
    let tasks_content = fs::read_to_string(tasks_file).unwrap();
    assert!(
        tasks_content.contains("MVP Parser"),
        "Tasks should contain sprint tasks"
    );

    // Verify Sprint-Context.md was created
    let context_file = sprint_folder.join("Sprint-Context.md");
    assert!(context_file.exists(), "Sprint-Context.md should exist");
    let context_content = fs::read_to_string(context_file).unwrap();
    assert!(
        context_content.contains("Tactical Staging Area"),
        "Context should contain focus"
    );

    // Verify folders were created
    assert!(
        sprint_folder.join("approvals").exists(),
        "approvals/ should exist"
    );
    assert!(
        sprint_folder.join("sessions").exists(),
        "sessions/ should exist"
    );

    // Verify config was updated
    let config_content = fs::read_to_string(project_path.join("nexus.toml")).unwrap();
    assert!(
        config_content.contains("sprint-4"),
        "Config should reference sprint-4"
    );
    assert!(
        config_content.contains("in_progress"),
        "Config should mark sprint as in_progress"
    );
}

#[test]
fn test_sprint_command_fails_if_previous_not_approved() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create files before git init
    create_nexus_config(
        project_path,
        project_path,
        Some(("sprint-3", "in_progress")),
    );

    // Create planning directory with MVP breakdown
    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_mvp_breakdown(&planning_dir);

    fs::write(project_path.join("README.md"), "# Test Project\n").unwrap();

    // Setup git repository
    init_test_git_repo(&project_path.to_path_buf());

    // Run sprint command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("sprint").arg(project_path).arg("4");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("SPRINT BLOCKED"))
        .stdout(predicate::str::contains("not approved"));
}

#[test]
fn test_sprint_command_succeeds_if_previous_approved() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create files before git init
    create_nexus_config(project_path, project_path, Some(("sprint-3", "approved")));

    // Create planning directory with MVP breakdown
    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_mvp_breakdown(&planning_dir);

    fs::write(project_path.join("README.md"), "# Test Project\n").unwrap();

    // Setup git repository
    init_test_git_repo(&project_path.to_path_buf());

    // Run sprint command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("sprint").arg(project_path).arg("4");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("SPRINT READY"));
}

#[test]
fn test_sprint_command_fails_with_dirty_working_directory() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create files before git init
    create_nexus_config(project_path, project_path, None);

    // Create planning directory with MVP breakdown
    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_mvp_breakdown(&planning_dir);

    fs::write(project_path.join("README.md"), "# Test Project\n").unwrap();

    // Setup git repository
    init_test_git_repo(&project_path.to_path_buf());

    // Create uncommitted changes
    fs::write(project_path.join("dirty.txt"), "uncommitted").unwrap();

    // Run sprint command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("sprint").arg(project_path).arg("4");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to create sprint branch"));
}

#[test]
fn test_sprint_command_fails_with_invalid_sprint_number() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    // Create files before git init
    create_nexus_config(project_path, project_path, None);

    // Create planning directory with MVP breakdown
    let planning_dir = project_path.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();
    create_mvp_breakdown(&planning_dir);

    fs::write(project_path.join("README.md"), "# Test Project\n").unwrap();

    // Setup git repository
    init_test_git_repo(&project_path.to_path_buf());

    // Run sprint command with non-existent sprint number
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("sprint").arg(project_path).arg("99");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Sprint 99 not found"));
}
