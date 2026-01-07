use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

/// Create a test project with incomplete planning (should fail gate)
fn setup_incomplete_project(project_dir: &Path) {
    // Create proper nexus.toml with new structure
    let nexus_toml = format!(
        r#"[project]
name = "test-project"
version = "0.1.0"
obsidian_path = "{}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[obsidian]
planning_path = "{}"

[state]
is_unlocked = false

[templates]
claude_template = "templates/CLAUDE.md.example"
"#,
        project_dir.display(),
        project_dir.display()
    );
    fs::write(project_dir.join("nexus.toml"), nexus_toml).unwrap();

    // Create planning directory
    let planning_dir = project_dir.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Create management directory with unchecked items
    let management_dir = project_dir.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();

    let start_here = "# Planning Dashboard\n- [ ] Unchecked task";
    fs::write(management_dir.join("00-START-HERE.md"), start_here).unwrap();

    // Create heuristics (lenient for testing - focus is on unlock, not validation)
    let heuristics = r#"{
  "min_section_length": 0,
  "required_headers": [],
  "illegal_strings": [],
  "management_files": {
    "dashboard": "00-START-HERE.md",
    "require_all_checked": true
  }
}"#;
    fs::write(project_dir.join("Gate-Heuristics.json"), heuristics).unwrap();

    // Create minimal planning docs (will fail gate due to unchecked dashboard)
    fs::write(
        planning_dir.join("01-Problem-and-Vision.md"),
        "# Vision\nTest vision content here.",
    )
    .unwrap();
    fs::write(
        planning_dir.join("02-Scope-and-Boundaries.md"),
        "# MVP\n- Feature 1",
    )
    .unwrap();
    fs::write(
        planning_dir.join("03-Tech-Stack.md"),
        "# Stack\n- Rust language",
    )
    .unwrap();
    fs::write(
        planning_dir.join("04-Architecture.md"),
        "# Architecture\nBasic architecture",
    )
    .unwrap();
    fs::write(
        planning_dir.join("05-MVP-Breakdown.md"),
        "# Sprint 0\n- Setup",
    )
    .unwrap();
}

/// Create a test project with complete planning (should pass gate)
fn setup_complete_project(project_dir: &Path) {
    // Create proper nexus.toml with new structure
    let nexus_toml = format!(
        r#"[project]
name = "test-project"
version = "0.1.0"
obsidian_path = "{}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[obsidian]
planning_path = "{}"

[state]
is_unlocked = false

[templates]
claude_template = "templates/CLAUDE.md.example"
"#,
        project_dir.display(),
        project_dir.display()
    );
    fs::write(project_dir.join("nexus.toml"), nexus_toml).unwrap();

    // Create planning directory
    let planning_dir = project_dir.join("01-PLANNING");
    fs::create_dir_all(&planning_dir).unwrap();

    // Create management directory with ALL items checked
    let management_dir = project_dir.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir).unwrap();

    let start_here = "# Planning Dashboard\n- [x] All tasks completed\n- [x] Ready to unlock";
    fs::write(management_dir.join("00-START-HERE.md"), start_here).unwrap();

    // Create heuristics (lenient for testing - focus is on unlock, not validation)
    let heuristics = r#"{
  "min_section_length": 0,
  "required_headers": [],
  "illegal_strings": [],
  "management_files": {
    "dashboard": "00-START-HERE.md",
    "require_all_checked": true
  }
}"#;
    fs::write(project_dir.join("Gate-Heuristics.json"), heuristics).unwrap();

    // Create complete planning docs
    fs::write(
        planning_dir.join("01-Problem-and-Vision.md"),
        r"# What is the problem?

## My problem (personal):
I need a tool to manage my projects better and stay organized always.

## Who else has this problem?
Developers who struggle with project planning and staying focused on execution consistently.

## Solution in ONE SENTENCE:
A CLI tool that enforces planning discipline through gates and validation rules consistently.

## Success criteria (3 months):
1. Successfully plan 5 projects
2. Complete at least 3 projects
3. Reduce planning time by 50%

## Anti-vision (what this project is NOT):
- Not: A project management SaaS
- Not: An AI-powered tool
- Not: A replacement for existing tools
",
    )
    .unwrap();

    fs::write(
        planning_dir.join("02-Scope-and-Boundaries.md"),
        r"# What am I building? (Scope)

## MVP (Minimum Viable Product):
Absolute minimum version that solves core problem:
- [x] Feature 1: Initialize projects
- [x] Feature 2: Validate planning
- [x] Feature 3: Generate CLAUDE.md

## Never (things I will NOT build):
- Web interface
- Cloud sync

## Tech constraints:
- Budget: Free/open source
- Deadline: 3 months
- Platform: CLI only
",
    )
    .unwrap();

    fs::write(
        planning_dir.join("03-Tech-Stack.md"),
        r"# Technical choices

## Stack (force yourself to choose NOW):
- **Frontend:** CLI only
- **Backend:** Rust
- **Database:** None (filesystem)
- **Hosting:** Local only

## Why these choices?
Rust provides performance and reliability for a CLI tool.

## What I will NOT use:
- Not: Node.js
- Not: Python

## Dependencies (max 10 important ones):
1. clap
2. serde
3. tera

## Development environment:
- IDE: VS Code
- OS: Linux
- Device: Desktop
",
    )
    .unwrap();

    fs::write(
        planning_dir.join("04-Architecture.md"),
        r"# System design

## Folder structure:
```
project/
├── src/
│   ├── commands/
│   └── lib/
├── tests/
└── docs/
```

## Data model (main entities):
1. **NexusConfig:** Configuration for project
   - Fields: name, paths, settings
2. **PlanningContext:** Extracted planning data
   - Fields: problem, vision, scope, etc.

## Flow (user journey):
1. User runs nexus init
2. User fills planning docs
3. User runs nexus gate
4. User runs nexus unlock
5. Development begins

## Critical technical decisions:
- State management: File-based (nexus.toml)
- Navigation: CLI subcommands
- Data persistence: Markdown files
",
    )
    .unwrap();

    fs::write(
        planning_dir.join("05-MVP-Breakdown.md"),
        r"# MVP broken into sprints

## Sprint 0: Setup (day 1)
- [x] Create repo
- [x] Setup dev environment
- [x] Hello World runs
**Exit criteria:** Can build and run empty app

## Sprint 1: Init Command (days 2-4)
- [x] Implement init command
- [x] Create template structure
- [x] Generate nexus.toml
**Exit criteria:** Can initialize new projects

## Sprint 2: Gate Command (days 5-7)
- [x] Implement validation logic
- [x] Add heuristics support
**Exit criteria:** Gate validates planning

## Sprint 3: Unlock Command (days 8-10)
- [ ] Implement unlock command
- [ ] Generate CLAUDE.md
- [ ] Initialize git repo
**Exit criteria:** Can unlock projects

## Definition of Done (each sprint):
- [x] Builds without errors
- [x] Tested on device/browser
- [x] Committed to git
- [x] Session log updated
",
    )
    .unwrap();
}

#[test]
fn test_unlock_fails_if_gate_fails() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    setup_incomplete_project(temp_dir.path());

    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("unlock").arg(temp_dir.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("UNLOCK ABORTED"));

    // Verify CLAUDE.md was NOT created
    let claude_path = temp_dir.path().join("CLAUDE.md");
    assert!(
        !claude_path.exists(),
        "CLAUDE.md should not be created when gate fails"
    );

    Ok(())
}

#[test]
fn test_unlock_succeeds_with_complete_planning() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    setup_complete_project(temp_dir.path());

    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("unlock").arg(temp_dir.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("PROJECT UNLOCKED"))
        .stdout(predicate::str::contains("CLAUDE.md generated"));

    // Verify CLAUDE.md was created
    let claude_path = temp_dir.path().join("CLAUDE.md");
    assert!(claude_path.exists(), "CLAUDE.md should be created");

    // Verify content includes project name (parsed from directory)
    let content = fs::read_to_string(&claude_path)?;
    // The project name comes from the parent directory name, not the config
    // Since we're using a temp directory, check for common content instead
    assert!(
        content.contains("PROJECT CONSTITUTION"),
        "CLAUDE.md should contain header. Content: {}",
        &content[..content.len().min(500)]
    );

    // Verify git repo was initialized
    let git_dir = temp_dir.path().join(".git");
    assert!(git_dir.exists(), "Git repository should be initialized");

    Ok(())
}

#[test]
fn test_unlock_is_idempotent() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    setup_complete_project(temp_dir.path());

    // First unlock
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("unlock").arg(temp_dir.path());
    cmd.assert().success();

    // Second unlock should still succeed (idempotent)
    let mut cmd2 = cargo_bin_cmd!("nexus");
    cmd2.arg("unlock").arg(temp_dir.path());
    cmd2.assert()
        .success()
        .stdout(predicate::str::contains("already has commits"));

    Ok(())
}
