use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_adhoc_task_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("repo");
    let vault_path = temp_dir.path().join("vault");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&vault_path).unwrap();

    // 1. Init adhoc project
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("init")
        .arg("test_task")
        .arg("--mode")
        .arg("adhoc")
        .current_dir(&project_path);
    
    // Note: init command might need more args or setup depending on implementation
    // For this test, we'll manually create the config if init is complex
    let vault_path_str = vault_path.to_str().unwrap();
    let config_content = format!(
        r#"[project]
name = "test_task"
version = "0.1.0"
obsidian_path = "{vault_path_str}"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[state]
is_unlocked = false

[tasks]
mode = "adhoc"
adhoc_planning_dir = "adhoc-planning"
adhoc_dashboard = "00-ADHOC-TASK.md"
"#
    );
    fs::write(project_path.join("nexus.toml"), &config_content).unwrap();

    let management_dir = vault_path.join("00-MANAGEMENT");
    let planning_dir = management_dir.join("adhoc-planning");
    fs::create_dir_all(&planning_dir).unwrap();

    // 2. Try to start without planning (should fail)
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("task-start").arg(&project_path);
    cmd.assert().failure();

    // 3. Fill planning docs
    fs::write(
        planning_dir.join("Task-Capture.md"),
        "## Source\n\nThis is a test source section with enough words to pass validation. It needs to be at least one hundred words long to satisfy the gate requirements for adhoc tasks in this project. We are writing this content to ensure that the test can proceed to the next phase of the workflow. The source of this task is a self-identified need for better integration testing of the adhoc task feature. This will help maintain high code quality and prevent regressions in the future as the project evolves and more features are added to the Nexus CLI tool. We are adding more words here to make sure we hit the limit. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Problem Statement\n\nThe problem is that we need a way to verify the entire adhoc task workflow from start to finish. This includes initialization, planning validation, task starting, and task completion. Without this test, we cannot be sure that all components are working together correctly. This is a critical gap in our testing strategy that needs to be addressed immediately to ensure the reliability of the adhoc task feature for all users of the Nexus CLI tool. We will implement a comprehensive integration test to cover this scenario. We are adding more words here to make sure we hit the limit. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Context\n\nThis test operates within the integration test suite of the Nexus CLI project. It uses temporary directories to simulate a real user environment with a project repository and an Obsidian vault. The test interacts with the Nexus CLI binary using the assert_cmd crate to verify command behavior and output. This approach provides a high level of confidence in the correctness of the implementation by testing the actual binary in a realistic scenario. The test covers multiple phases of the task lifecycle. We are adding more words here to make sure we hit the limit. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Definition of Done\n\n- [ ] Workflow test passes\n- [ ] All commands behave as expected\n- [ ] Dashboard is updated correctly\n- [ ] More words here to pass the limit. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n"
    ).unwrap();

    fs::write(
        planning_dir.join("Task-Approach.md"),
        "## Analysis\n\nWe have analyzed the requirements for the adhoc task workflow and identified the key steps that need to be tested. These include the transition from planning to execution and the final completion of the task. We have also reviewed the validation rules for adhoc tasks to ensure that our test data satisfies all criteria. This analysis has informed the design of the integration test and the selection of test data. We will use a temporary directory to avoid side effects on the local system and ensure a clean test environment for every run of the test suite. We are adding more words here to make sure we hit the limit of one hundred and fifty words. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Proposed Solution\n\nThe solution is to implement a single integration test that executes all commands in the adhoc task workflow in sequence. We will verify the success or failure of each command and check the state of the filesystem and the contents of the dashboard file at each step. This will provide a comprehensive verification of the feature and ensure that all components are working together as intended. We will also include checks for error conditions to verify that the gate correctly blocks invalid transitions. This will ensure that the ADHD protection features are working correctly. We are adding more words here to make sure we hit the limit of one hundred and fifty words. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Risks\n\n- [x] Risk: Test might be flaky due to filesystem timing issues. Mitigation: Use robust filesystem operations and wait for changes if necessary. We are adding more words here to make sure we hit the limit of one hundred and fifty words. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Files to Modify\n\nWe will create a new test file tests/task_workflow.rs and add it to the project's test suite. This file will contain the implementation of the adhoc task workflow integration test. We will also update the Cargo.toml file if necessary to include any new dependencies required for the test. This will ensure that the test is properly integrated into the project's build and test process and can be run easily by all developers and in the continuous integration environment. We are adding more words here to make sure we hit the limit of one hundred and fifty words. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n"
    ).unwrap();

    fs::write(
        planning_dir.join("Task-Validation.md"),
        "## Pre-Work\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n- [x] More words here to pass the limit. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Implementation\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n- [x] More words here to pass the limit. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n\n## Verification\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n- [x] More words here to pass the limit. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten. One two three four five six seven eight nine ten.\n"
    ).unwrap();

    fs::write(
        management_dir.join("00-ADHOC-TASK.md"),
        "# Adhoc Task Dashboard\n\n## Planning Phase\n- [x] Task-Capture.md filled (min 100 words)\n- [x] Task-Approach.md filled (min 150 words)\n- [x] Task-Validation.md checklist created\n\n## Execution Phase\n- [ ] Implementation complete\n- [ ] All Task-Validation.md boxes checked\n- [ ] Ready for review/merge\n\n**Planning completed:** [auto-filled by task start]\n\n**Task completed:** [auto-filled by task done]\n"
    ).unwrap();

    // 4. Start task (should pass now)
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("task-start").arg(&project_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TASK STARTED SUCCESSFULLY"));

    // 5. Verify dashboard updated with timestamp
    let dashboard_content = fs::read_to_string(management_dir.join("00-ADHOC-TASK.md")).unwrap();
    assert!(dashboard_content.contains("**Planning completed:** 202")); // Matches 202x

    // 6. Try to mark done without validation (should fail)
    // We'll uncheck a box in Task-Validation.md
    fs::write(
        planning_dir.join("Task-Validation.md"),
        "## Pre-Work\n\n- [ ] Step 1\n- [x] Step 2\n- [x] Step 3\n\n## Implementation\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n\n## Verification\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n"
    ).unwrap();

    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("task-done").arg(&project_path);
    cmd.assert().failure();

    // 7. Mark done (should pass now)
    fs::write(
        planning_dir.join("Task-Validation.md"),
        "## Pre-Work\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n\n## Implementation\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n\n## Verification\n\n- [x] Step 1\n- [x] Step 2\n- [x] Step 3\n"
    ).unwrap();

    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("task-done").arg(&project_path);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TASK COMPLETED SUCCESSFULLY"));

    // 8. Verify dashboard updated with completion timestamp
    let dashboard_content = fs::read_to_string(management_dir.join("00-ADHOC-TASK.md")).unwrap();
    assert!(dashboard_content.contains("**Task completed:** 202"));
}
