//! Integration tests for the `nexus plan` command

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper: Create a test project structure with nexus.toml
fn create_test_project() -> (TempDir, PathBuf, TempDir) {
    // Create temporary repo directory
    let repo_dir = TempDir::new().unwrap();
    let repo_path = repo_dir.path().to_path_buf();

    // Create temporary Obsidian vault
    let vault_dir = TempDir::new().unwrap();
    let vault_path = vault_dir.path().to_path_buf();

    // Create nexus.toml
    let config_content = format!(
        r#"[project]
name = "test_project"
version = "0.1.0"
obsidian_path = "{}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/sprints"

[gate]
heuristics_file = ".nexus/gate-heuristics.json"
strict_mode = true

[state]
is_unlocked = false

[llm]
provider = "gemini"
model = "gemini-3-pro"
enabled = false

[brain]
qdrant_url = "http://localhost:6334"
enabled = false
"#,
        vault_path.display()
    );

    fs::write(repo_path.join("nexus.toml"), config_content).unwrap();

    // Create .nexus directory
    fs::create_dir_all(repo_path.join(".nexus")).unwrap();

    (repo_dir, repo_path, vault_dir)
}

/// Helper: Create a vision document
fn create_vision_document(vault_path: &PathBuf) {
    let vision_content = r#"# My problem (personal):
I need to manage my projects better and ensure I stay focused on the MVP.

## Solution in ONE SENTENCE:
A CLI tool for Obsidian-based project management that enforces gated workflows.

## Success criteria (3 months):
Successfully plan and execute 5 projects with clear scope boundaries.

## Anti-vision (what this project is NOT):
Not a full project management suite with team collaboration features.
"#;

    fs::write(vault_path.join("01-Problem-and-Vision.md"), vision_content).unwrap();
}

#[test]
fn test_plan_init_fails_without_vision_document() {
    let (_repo_dir, repo_path, _vault_dir) = create_test_project();

    // Don't create vision document - should fail

    let result = std::panic::catch_unwind(|| {
        let output = std::process::Command::new("cargo")
            .args([
                "run",
                "--",
                "plan",
                "--init",
                repo_path.to_str().unwrap(),
            ])
            .output()
            .unwrap();

        String::from_utf8_lossy(&output.stderr).to_string()
    });

    assert!(result.is_ok());
    let stderr = result.unwrap();
    assert!(
        stderr.contains("Vision document not found") || stderr.contains("Failed to read vision"),
        "Expected vision document error, got: {stderr}"
    );
}

#[test]
fn test_plan_init_fails_with_active_task_capsule() {
    let (_repo_dir, repo_path, vault_dir) = create_test_project();
    let vault_path = vault_dir.path().to_path_buf();

    // Create vision document
    create_vision_document(&vault_path);

    // Create heuristics file with active task capsule IN THE REPO
    // The .nexus/gate-heuristics.json lives in the Git repository, not the Obsidian vault
    let heuristics_content = r#"{
  "active_capsule": "TASK-001",
  "last_updated": "2025-01-01T00:00:00Z"
}"#;

    fs::create_dir_all(repo_path.join(".nexus")).unwrap();
    fs::write(
        repo_path.join(".nexus/gate-heuristics.json"),
        heuristics_content,
    )
    .unwrap();

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "plan",
            "--init",
            repo_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.contains("GENESIS GUARDRAIL VIOLATION")
            || combined.contains("active Task Capsule is detected"),
        "Expected guardrail violation, got stdout: {stdout}, stderr: {stderr}"
    );

    // Should fail (non-zero exit code)
    assert!(!output.status.success(), "Command should have failed");
}

#[test]
fn test_plan_init_passes_guardrails_with_no_active_capsule() {
    let (_repo_dir, repo_path, vault_dir) = create_test_project();
    let vault_path = vault_dir.path().to_path_buf();

    // Create vision document
    create_vision_document(&vault_path);

    // Create heuristics file with NO active task capsule IN THE REPO
    let heuristics_content = r#"{
  "active_capsule": null,
  "last_updated": "2025-01-01T00:00:00Z"
}"#;

    fs::create_dir_all(repo_path.join(".nexus")).unwrap();
    fs::write(
        repo_path.join(".nexus/gate-heuristics.json"),
        heuristics_content,
    )
    .unwrap();

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "plan",
            "--init",
            repo_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stdout}{stderr}");

    // Should NOT contain guardrail violation
    assert!(
        !combined.contains("GENESIS GUARDRAIL VIOLATION"),
        "Should not trigger guardrail, got stdout: {stdout}, stderr: {stderr}"
    );

    // Should pass the guardrail check (but may fail on LLM being disabled)
    // We're just testing that the guardrail logic works
    assert!(
        combined.contains("No active Task Capsule detected")
            || combined.contains("Vision document found")
            || combined.contains("LLM"),
        "Expected to pass guardrail check, got stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn test_plan_init_enforces_llm_configuration() {
    let (_repo_dir, repo_path, vault_dir) = create_test_project();
    let vault_path = vault_dir.path().to_path_buf();

    // Create vision document
    create_vision_document(&vault_path);

    // LLM is disabled in config (enabled = false)

    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "plan",
            "--init",
            repo_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("LLM is disabled") || stderr.contains("LLM not configured"),
        "Expected LLM configuration error, got: {stderr}"
    );
}
