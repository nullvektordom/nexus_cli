//! Sprint Scaffolding Module
//!
//! Creates the Obsidian folder structure for sprint management.

use crate::planning::SprintData;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Scaffold the sprint folder structure in the Obsidian vault
///
/// Creates the complete folder structure for a sprint:
/// - 00-MANAGEMENT/sprints/sprint-{number}-{name}/
///   - Tasks.md (extracted tasks from MVP)
///   - Sprint-Context.md (scope and boundaries)
///   - approvals/ (empty folder for approval artifacts)
///   - sessions/ (empty folder for dev session notes)
///
/// # Arguments
/// * `planning_path` - Path to the Obsidian vault root (where 00-MANAGEMENT lives)
/// * `sprint_data` - Parsed sprint information from MVP breakdown
///
/// # Returns
/// * `Ok(())` - Sprint folder created successfully
/// * `Err` - If folder creation or file writing fails
pub fn scaffold_sprint_folder(planning_path: &Path, sprint_data: &SprintData) -> Result<()> {
    // Build sprint folder path
    let sprint_folder = planning_path
        .join("00-MANAGEMENT")
        .join("sprints")
        .join(format!("sprint-{}-{}", sprint_data.number, sprint_data.name));

    // Create main sprint folder
    fs::create_dir_all(&sprint_folder).with_context(|| {
        format!(
            "Failed to create sprint folder: {}",
            sprint_folder.display()
        )
    })?;

    // Create Tasks.md
    create_tasks_file(&sprint_folder, sprint_data)?;

    // Create Sprint-Context.md
    create_context_file(&sprint_folder, sprint_data)?;

    // Create approvals/ folder
    let approvals_dir = sprint_folder.join("approvals");
    fs::create_dir_all(&approvals_dir)
        .with_context(|| format!("Failed to create approvals folder: {}", approvals_dir.display()))?;

    // Create sessions/ folder
    let sessions_dir = sprint_folder.join("sessions");
    fs::create_dir_all(&sessions_dir)
        .with_context(|| format!("Failed to create sessions folder: {}", sessions_dir.display()))?;

    Ok(())
}

/// Create Tasks.md with extracted task list
fn create_tasks_file(sprint_folder: &Path, sprint_data: &SprintData) -> Result<()> {
    let tasks_content = format!(
        r#"# Sprint {} Tasks: {}

## Task List
{}

## Notes
- Update task status as you progress
- Mark completed tasks with [x]
- Add blockers or issues below

## Blockers
- (none yet)
"#,
        sprint_data.number, sprint_data.title, sprint_data.tasks
    );

    let tasks_path = sprint_folder.join("Tasks.md");
    fs::write(&tasks_path, tasks_content)
        .with_context(|| format!("Failed to write Tasks.md: {}", tasks_path.display()))?;

    Ok(())
}

/// Create Sprint-Context.md with boundary rules and scope
fn create_context_file(sprint_folder: &Path, sprint_data: &SprintData) -> Result<()> {
    let context_content = format!(
        r#"# Sprint {} Context: {}

## Focus
{}

## Scope Boundaries

### Allowed (MVP Only)
- Implement features exactly as specified in the sprint tasks
- Add necessary error handling and validation
- Write tests for new functionality
- Update documentation for changes made

### Forbidden (Outside MVP Scope)
- Adding features not in the task list
- Refactoring existing code unless required for the task
- Optimizations beyond basic functionality
- UI/UX improvements not specified in tasks
- Additional dependencies not approved in planning

## Success Criteria
- All tasks marked complete
- Tests pass
- Code builds without warnings
- Sprint approved by commander

## Resources
- Planning docs: `01-PLANNING/`
- MVP breakdown: `01-PLANNING/05-MVP-Breakdown.md`
- Tech stack: `01-PLANNING/03-Tech-Stack.md`
"#,
        sprint_data.number, sprint_data.title, sprint_data.context
    );

    let context_path = sprint_folder.join("Sprint-Context.md");
    fs::write(&context_path, context_content)
        .with_context(|| format!("Failed to write Sprint-Context.md: {}", context_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_sprint_data() -> SprintData {
        SprintData {
            number: 4,
            name: "the-sprint-orchestrator".to_string(),
            title: "The Sprint Orchestrator (The Leash)".to_string(),
            tasks: "- [ ] Task one\n- [ ] Task two\n- [ ] Task three".to_string(),
            context: "_Focus: Creating the Tactical Staging Area._\n**Exit criteria:** All tasks complete".to_string(),
        }
    }

    #[test]
    fn test_scaffold_sprint_folder_creates_structure() {
        let temp_dir = TempDir::new().unwrap();
        let planning_path = temp_dir.path();

        let sprint_data = create_test_sprint_data();

        let result = scaffold_sprint_folder(planning_path, &sprint_data);
        assert!(result.is_ok(), "Should create sprint folder successfully");

        // Verify main folder exists
        let sprint_folder = planning_path
            .join("00-MANAGEMENT")
            .join("sprints")
            .join("sprint-4-the-sprint-orchestrator");
        assert!(
            sprint_folder.exists(),
            "Sprint folder should exist: {}",
            sprint_folder.display()
        );

        // Verify Tasks.md exists
        let tasks_file = sprint_folder.join("Tasks.md");
        assert!(tasks_file.exists(), "Tasks.md should exist");

        // Verify Sprint-Context.md exists
        let context_file = sprint_folder.join("Sprint-Context.md");
        assert!(context_file.exists(), "Sprint-Context.md should exist");

        // Verify approvals/ folder exists
        let approvals_dir = sprint_folder.join("approvals");
        assert!(
            approvals_dir.exists() && approvals_dir.is_dir(),
            "approvals/ folder should exist"
        );

        // Verify sessions/ folder exists
        let sessions_dir = sprint_folder.join("sessions");
        assert!(
            sessions_dir.exists() && sessions_dir.is_dir(),
            "sessions/ folder should exist"
        );
    }

    #[test]
    fn test_tasks_file_contains_correct_content() {
        let temp_dir = TempDir::new().unwrap();
        let planning_path = temp_dir.path();
        let sprint_data = create_test_sprint_data();

        scaffold_sprint_folder(planning_path, &sprint_data).unwrap();

        let tasks_file = planning_path
            .join("00-MANAGEMENT")
            .join("sprints")
            .join("sprint-4-the-sprint-orchestrator")
            .join("Tasks.md");

        let content = fs::read_to_string(tasks_file).unwrap();

        assert!(
            content.contains("Sprint 4 Tasks"),
            "Should contain sprint title"
        );
        assert!(content.contains("Task one"), "Should contain tasks");
        assert!(content.contains("Task two"), "Should contain tasks");
        assert!(
            content.contains("Blockers"),
            "Should have blockers section"
        );
    }

    #[test]
    fn test_context_file_contains_correct_content() {
        let temp_dir = TempDir::new().unwrap();
        let planning_path = temp_dir.path();
        let sprint_data = create_test_sprint_data();

        scaffold_sprint_folder(planning_path, &sprint_data).unwrap();

        let context_file = planning_path
            .join("00-MANAGEMENT")
            .join("sprints")
            .join("sprint-4-the-sprint-orchestrator")
            .join("Sprint-Context.md");

        let content = fs::read_to_string(context_file).unwrap();

        assert!(
            content.contains("Sprint 4 Context"),
            "Should contain sprint title"
        );
        assert!(content.contains("Focus"), "Should have focus section");
        assert!(
            content.contains("Scope Boundaries"),
            "Should have boundaries section"
        );
        assert!(
            content.contains("Allowed (MVP Only)"),
            "Should have allowed scope"
        );
        assert!(
            content.contains("Forbidden (Outside MVP Scope)"),
            "Should have forbidden scope"
        );
        assert!(
            content.contains("Creating the Tactical Staging Area"),
            "Should include focus text"
        );
    }

    #[test]
    fn test_scaffold_creates_nested_directories() {
        let temp_dir = TempDir::new().unwrap();
        let planning_path = temp_dir.path();
        let sprint_data = create_test_sprint_data();

        // Don't pre-create any directories - test that scaffold creates them all
        let result = scaffold_sprint_folder(planning_path, &sprint_data);
        assert!(
            result.is_ok(),
            "Should create all nested directories: {:?}",
            result.err()
        );

        let management_dir = planning_path.join("00-MANAGEMENT");
        assert!(
            management_dir.exists(),
            "Should create 00-MANAGEMENT directory"
        );

        let sprints_dir = management_dir.join("sprints");
        assert!(sprints_dir.exists(), "Should create sprints directory");
    }
}
