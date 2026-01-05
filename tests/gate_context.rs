use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

use nexus::commands::gate;

fn create_project(dir: &Path, toml_content: &str) -> Result<()> {
    // Create nexus.toml
    fs::write(dir.join("nexus.toml"), toml_content)?;

    // Create 00-MANAGEMENT/00-START-HERE.md
    let mgmt_dir = dir.join("00-MANAGEMENT");
    fs::create_dir_all(&mgmt_dir)?;
    fs::write(
        mgmt_dir.join("00-START-HERE.md"),
        "# Dashboard\n- [x] Task 1\n",
    )?;

    // Create 01-PLANNING
    let planning_dir = dir.join("01-PLANNING");
    fs::create_dir_all(&planning_dir)?;

    // Create Gate-Heuristics.json at the root
    fs::write(
        dir.join("Gate-Heuristics.json"),
        r#"{
            "min_section_length": 1,
            "required_headers": [],
            "illegal_strings": [],
            "management_files": {
                "dashboard": "00-START-HERE.md",
                "require_all_checked": true
            }
        }"#,
    )?;

    Ok(())
}

fn create_planning_docs(dir: &Path) -> Result<()> {
    let planning_dir = dir.join("01-PLANNING");
    fs::write(planning_dir.join("01-Problem.md"), "# Problem\nDescription")?;
    Ok(())
}

fn create_sprint_structure(dir: &Path, sprint_name: &str) -> Result<()> {
    // Note: This matches the structure in `test_gate_sprint_phase_passed` config
    // which is "00-MANAGEMENT/Sprints" (case sensitive in config, but maybe lower on disk?)
    // The test config says "sprint_dir = "00-MANAGEMENT/Sprints"".
    // But gate uses config.structure.sprint_dir.
    // So we should create the dir at "00-MANAGEMENT/Sprints".

    let sprint_dir = dir
        .join("00-MANAGEMENT")
        .join("Sprints")
        .join(sprint_name);
    fs::create_dir_all(&sprint_dir)?;

    // Create valid sprint files
    fs::write(
        sprint_dir.join("Tasks.md"),
        "# Tasks\n- [x] Task 1\n"
    )?;
    fs::write(
        sprint_dir.join("Sprint-Context.md"),
        "# Context\nFocus: stuff\n"
    )?;

    Ok(())
}

#[test]
fn test_gate_init_phase_passed() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let path = temp_dir.path();
    let path_str = path.to_str().unwrap().replace("\\", "/");

    let config = format!(r#"
[project]
name = "test"
version = "0.1.0"
obsidian_path = "{}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true
"#, path_str);

    create_project(path, &config)?;
    create_planning_docs(path)?;

    // Init phase should pass
    gate::execute(path)?;

    Ok(())
}

#[test]
fn test_gate_init_phase_failed() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let path = temp_dir.path();
    let path_str = path.to_str().unwrap().replace("\\", "/");

    let config = format!(r#"
[project]
name = "test"
version = "0.1.0"
obsidian_path = "{}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true
"#, path_str);

    create_project(path, &config)?;
    // We don't create planning docs, so it should fail (or warn, let's see logic)

    let result = gate::execute(path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_gate_sprint_phase_passed() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let path = temp_dir.path();
    let path_str = path.to_str().unwrap().replace("\\", "/");

    let config = format!(r#"
[project]
name = "test"
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
is_unlocked = true

[state.active_sprint]
current = "sprint-1"
status = "in_progress"
"#, path_str);

    create_project(path, &config)?;
    create_sprint_structure(path, "sprint-1-test")?;

    // Sprint phase should pass
    gate::execute(path)?;

    Ok(())
}

#[test]
fn test_gate_sprint_phase_failed_unchecked_tasks() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let path = temp_dir.path();
    let path_str = path.to_str().unwrap().replace("\\", "/");

    let config = format!(r#"
[project]
name = "test"
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
is_unlocked = true

[state.active_sprint]
current = "sprint-1"
status = "in_progress"
"#, path_str);

    create_project(path, &config)?;
    create_sprint_structure(path, "sprint-1-test")?;

    // Modify Tasks.md to have unchecked item
    let tasks_path = path
        .join("00-MANAGEMENT")
        .join("Sprints")
        .join("sprint-1-test")
        .join("Tasks.md");
    fs::write(tasks_path, "- [ ] Unchecked task")?;

    let result = gate::execute(path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_gate_no_active_sprint() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let path = temp_dir.path();
    let path_str = path.to_str().unwrap().replace("\\", "/");

    let config = format!(r#"
[project]
name = "test"
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
is_unlocked = true
"#, path_str);

    create_project(path, &config)?;

    // Should pass (informational only)
    gate::execute(path)?;

    Ok(())
}
