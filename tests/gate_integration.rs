use assert_cmd::cargo::cargo_bin_cmd;
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
    let content = r"# Problem
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
";
    fs::write(path, content).unwrap();
}

/// Helper to create a planning doc with issues
fn create_invalid_planning_doc(path: &PathBuf) {
    let content = r"# Problem
TODO: Fill this in later

# Vision
Not enough words here.
";
    fs::write(path, content).unwrap();
}

/// Helper to create a valid dashboard
fn create_valid_dashboard(path: &PathBuf) {
    let content = r"# Dashboard
- [x] Task 1 completed
- [x] Task 2 completed
- [x] Task 3 completed
";
    fs::write(path, content).unwrap();
}

/// Helper to create a dashboard with unchecked items
fn create_invalid_dashboard(path: &PathBuf) {
    let content = r"# Dashboard
- [x] Task 1 completed
- [ ] Task 2 not done
- [x] Task 3 completed
";
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
    let mut cmd = cargo_bin_cmd!("nexus");
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
    let mut cmd = cargo_bin_cmd!("nexus");
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
    let mut cmd = cargo_bin_cmd!("nexus");
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
    let mut cmd = cargo_bin_cmd!("nexus");
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
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(project_path);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read config"));
}

#[test]
fn test_gate_adhoc_mode_fails_with_empty_planning() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("repo");
    let vault_path = temp_dir.path().join("vault");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&vault_path).unwrap();

    // Create adhoc mode config
    let vault_path_str = vault_path.to_str().unwrap();
    let config_content = format!(
        r#"[project]
name = "test_adhoc"
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

    // Create minimal adhoc structure with empty planning docs
    let management_dir = vault_path.join("00-MANAGEMENT");
    let planning_dir = management_dir.join("adhoc-planning");
    fs::create_dir_all(&planning_dir).unwrap();

    // Create empty Task-Capture.md
    fs::write(
        planning_dir.join("Task-Capture.md"),
        "# Task Capture\n\n## Source\n\nTODO\n\n## Problem Statement\n\nTODO\n\n## Context\n\nTODO\n\n## Definition of Done\n\nTODO\n"
    ).unwrap();

    // Create empty Task-Approach.md
    fs::write(
        planning_dir.join("Task-Approach.md"),
        "# Implementation Approach\n\n## Analysis\n\nTODO\n\n## Proposed Solution\n\nTODO\n\n## Risks\n\nTODO\n\n## Files to Modify\n\nTODO\n"
    ).unwrap();

    // Create empty Task-Validation.md
    fs::write(
        planning_dir.join("Task-Validation.md"),
        "# Task Validation Checklist\n\n## Pre-Work\n\n- [ ] TODO\n\n## Implementation\n\n- [ ] TODO\n\n## Verification\n\n- [ ] TODO\n"
    ).unwrap();

    // Create dashboard with unchecked boxes
    fs::write(
        management_dir.join("00-ADHOC-TASK.md"),
        "# Adhoc Task Dashboard\n\n## Planning Phase\n- [ ] Task-Capture.md filled (min 100 words)\n- [ ] Task-Approach.md filled (min 150 words)\n- [ ] Task-Validation.md checklist created\n\n## Execution Phase\n- [ ] Implementation complete\n- [ ] All Task-Validation.md boxes checked\n- [ ] Ready for review/merge\n"
    ).unwrap();

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(&project_path);

    // Should fail because planning is incomplete
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("ADHOC MODE: TASK PLANNING"))
        .stdout(predicate::str::contains("SCANNING ADHOC DASHBOARD"))
        .stdout(predicate::str::contains("SCANNING ADHOC PLANNING DOCUMENTS"))
        .stdout(predicate::str::contains("Task-Capture.md"))
        .stdout(predicate::str::contains("GATE CLOSED"));
}

#[test]
fn test_gate_adhoc_mode_passes_with_complete_planning() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("repo");
    let vault_path = temp_dir.path().join("vault");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&vault_path).unwrap();

    // Create adhoc mode config
    let vault_path_str = vault_path.to_str().unwrap();
    let config_content = format!(
        r#"[project]
name = "test_adhoc"
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

    // Create minimal adhoc structure with complete planning docs
    let management_dir = vault_path.join("00-MANAGEMENT");
    let planning_dir = management_dir.join("adhoc-planning");
    fs::create_dir_all(&planning_dir).unwrap();

    // Create complete Task-Capture.md (100+ words per section)
    fs::write(
        planning_dir.join("Task-Capture.md"),
        "## Source\n\nThis task comes from a user-reported bug in the authentication system that was identified during our quarterly security code review process. The issue has been prioritized for immediate resolution because it affects multiple users across different deployment environments and could potentially lead to serious security vulnerabilities if not addressed promptly and correctly. The bug was originally reported in our issue tracking system under ticket number 12345 and has been independently confirmed by three separate users who experienced identical problematic behavior across different operating systems, network configurations, and deployment scenarios. This is a critical security issue that requires immediate attention and resolution to protect our users and maintain system integrity.\n\n## Problem Statement\n\nThe authentication system is currently failing to properly validate and process user tokens when they contain special characters or non-ASCII unicode sequences, leading to unauthorized access attempts being incorrectly granted access to protected resources. This is a serious and potentially dangerous security flaw that affects approximately fifteen percent of our total user base, particularly those users who have special characters, international characters, or unicode symbols in their usernames, email addresses, or other identity fields. The root cause appears to be an improper character escaping and encoding routine in the token validation middleware component that was inadvertently introduced in version two point three point zero of our authentication library during a recent refactoring effort.\n\n## Context\n\nThe authentication system is located in the source auth directory and consists of several interconnected and interdependent modules including token validation services, session management components, user credential verification systems, and authorization policy enforcement mechanisms. The bug specifically affects the TokenValidator class defined in source auth validator rust file which is centrally responsible for parsing, validating, and verifying JWT tokens throughout the entire application. This class is used extensively by all authenticated API endpoints and any failure or vulnerability here has significant cascading security effects throughout the entire application architecture. The token validation logic interacts deeply with the database persistence layer for session state lookups and the encryption cryptography module for digital signature verification operations.\n\n## Definition of Done\n\n- [ ] Token validation system properly handles all special characters including unicode sequences in all token fields including username, email, display name, and custom application claims without causing authentication failures, security vulnerabilities, or data corruption issues throughout the system\n- [ ] All existing comprehensive unit test suites pass successfully without any modifications required and extensive new test cases are added to thoroughly cover all special character scenarios, edge cases, and unicode handling in the token validation logic across all supported character encodings\n- [ ] Full integration test suites confirm that the security fix works correctly across all supported authentication flows including initial user login, token refresh operations, password reset workflows, and multi-factor authentication scenarios without introducing any regressions or breaking changes to existing functionality\n"
    ).unwrap();

    // Create complete Task-Approach.md (150+ words per section)
    fs::write(
        planning_dir.join("Task-Approach.md"),
        "## Analysis\n\nAfter conducting a thorough and comprehensive analysis of the entire codebase and authentication system architecture, I have successfully identified that the root cause of the critical token validation bug lies specifically in the improper and insufficient URL encoding handling of special characters and unicode sequences before they are processed by the JWT token validation library component. The current implementation unfortunately uses a basic and overly simplistic string replacement approach that does not properly account for all possible special characters that are defined in the comprehensive URL encoding specification standards. Additionally, the token parsing logic does not properly handle important edge cases where special characters appear at the beginning or end of a token field, leading to unexpected truncation errors and data loss. The validation middleware component also lacks proper and comprehensive error handling for malformed or corrupted tokens, which causes the system to fail open rather than fail closed when encountering unexpected or malicious input.\n\n## Proposed Solution\n\nThe proposed comprehensive solution involves completely refactoring the TokenValidator class to use the industry-standard and well-tested URL encoding library which provides comprehensive and complete support for all special characters that are defined in the official RFC 3986 specification document. We will systematically replace the current manual string manipulation code with proper calls to the library's standardized encode and decode functions, ensuring consistent and predictable behavior across all token operations throughout the system. The solution will include adding a new comprehensive validation layer that thoroughly checks for malformed or corrupted tokens before they reach the main validation logic, preventing any possibility of the system failing open and allowing unauthorized access. We will implement comprehensive error handling with specific error codes for different validation failure scenarios, making it significantly easier to debug and diagnose issues in production environments. The implementation will also include extensive documentation updates and code comments explaining the rationale behind each decision and providing guidance for future maintainers.\n\n## Risks\n\n- [x] The main and most significant risk with this implementation approach is the potential for inadvertently introducing breaking changes that could negatively affect existing authenticated users who currently have valid active tokens stored in their browser sessions or mobile applications. If we are not extremely careful with maintaining backward compatibility with the existing token format, we could inadvertently invalidate all existing user sessions forcing a mass logout event which would significantly impact user experience and generate numerous support tickets from confused users. There is also a significant risk that our test coverage might not catch all possible edge cases, particularly with less common special characters or unusual character combinations that might not appear in our standard test data sets. Another serious concern is the performance impact of adding additional validation layers and processing steps which could affect system responsiveness and user experience. We must also consider the deployment strategy and coordinate with operations teams to ensure smooth rollout.\n\n## Files to Modify\n\nThe following source code files will need to be carefully modified and updated to implement this critical security fix: src/auth/validator.rs file contains the main token validation logic and parsing routines, src/auth/middleware.rs file contains the authentication middleware component that calls the validator, src/auth/tokens.rs file contains token generation and parsing utilities and helper functions, src/auth/mod.rs file contains module exports and public API definitions, tests/auth/validator_tests.rs file contains comprehensive unit tests for the validator component, tests/auth/integration_tests.rs file contains end-to-end authentication flow integration tests, src/utils/encoding.rs file contains URL encoding utilities and helper functions, and docs/authentication.md file contains the API documentation for the authentication system. Each of these files plays a critical and important role in the overall authentication flow and requires careful modification to ensure we do not introduce regressions or break existing functionality that users depend on. We will also need to update configuration files and deployment scripts to ensure consistent behavior across all environments.\n"
    ).unwrap();

    // Create complete Task-Validation.md with all required headers
    fs::write(
        planning_dir.join("Task-Validation.md"),
        "## Pre-Work\n\n- [x] Read existing token validation code in src/auth/validator.rs and understand current implementation approach and limitations\n- [x] Review RFC 3986 URL encoding specification to ensure compliance with industry standards for special character handling\n- [x] Identify all special characters that need to be handled in token fields including username, email, and custom claims\n- [x] Set up test environment with tokens containing various special characters to verify current behavior and test fixes\n- [x] Review existing test coverage for token validation to identify gaps that need to be filled\n\n## Implementation\n\n- [x] Implement URL encoding function using standard library that properly handles all special characters defined in RFC 3986\n- [x] Refactor TokenValidator class to use new URL encoding function for all token parsing and validation operations\n- [x] Add comprehensive error handling with specific error codes for different validation failure scenarios\n- [x] Implement backward compatibility layer to handle tokens generated by previous implementation without breaking existing sessions\n- [x] Add extensive logging to track validation failures and help with debugging in production environments\n\n## Verification\n\n- [x] Run all existing unit tests to ensure no regressions were introduced in the refactored validation logic\n- [x] Add new unit tests covering special character scenarios including edge cases and unusual character combinations\n- [x] Run integration tests to verify the fix works correctly with the entire authentication flow from login to session management\n- [x] Perform manual testing with real tokens containing special characters to verify correct behavior in realistic scenarios\n- [x] Conduct security review with senior engineers to ensure no new vulnerabilities were introduced by the changes\n"
    ).unwrap();

    // Create dashboard with all Planning Phase boxes checked
    fs::write(
        management_dir.join("00-ADHOC-TASK.md"),
        "# Adhoc Task Dashboard\n\n## Planning Phase\n- [x] Task-Capture.md filled (min 100 words)\n- [x] Task-Approach.md filled (min 150 words)\n- [x] Task-Validation.md checklist created\n\n## Execution Phase\n- [ ] Implementation complete\n- [ ] All Task-Validation.md boxes checked\n- [ ] Ready for review/merge\n"
    ).unwrap();

    // Run gate command
    let mut cmd = cargo_bin_cmd!("nexus");
    cmd.arg("gate").arg(&project_path);

    // Should pass because planning is complete
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("ADHOC MODE: TASK PLANNING"))
        .stdout(predicate::str::contains("MISSION READY"))
        .stdout(predicate::str::contains("Gate is open"));
}
