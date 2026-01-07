//! Gate Command - The Gatekeeper
//!
//! Validates planning documents and dashboard checkboxes before allowing project unlock.
//! Provides ADHD-friendly terminal output with context snippets and line numbers.

#![allow(clippy::too_many_lines)] // Complex validation logic requires detailed checks
#![allow(clippy::if_not_else)] // Validation logic is clearer with existence checks first

use crate::config::NexusConfig;
use crate::heuristics::load_heuristics;
use crate::planning::{
    ValidationIssue, validate_dashboard_checkboxes, validate_planning_document_with_headers,
};
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

/// Execute the gate command
/// Returns Ok(()) if gate passes, Err if validation fails or error occurs
pub fn execute(project_path: &Path) -> Result<()> {
    // Load project configuration
    let config_path = project_path.join("nexus.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    // Resolve obsidian vault path
    let vault_path = config.get_repo_path();

    // DEFENSIVE: Verify vault exists and is accessible
    if !vault_path.exists() {
        anyhow::bail!(
            "Obsidian vault not found at: {}\n  \
             Check the 'obsidian_path' in your nexus.toml configuration.",
            vault_path.display()
        );
    }

    if !vault_path.is_dir() {
        anyhow::bail!(
            "Obsidian vault path is not a directory: {}\n  \
             Expected a directory, found a file.",
            vault_path.display()
        );
    }

    // DEFENSIVE: Check if vault is a symlink loop
    match vault_path.metadata() {
        Ok(_) => {} // Successfully accessed metadata, not a loop
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            anyhow::bail!(
                "Vault path exists but metadata unavailable (possible symlink loop): {}",
                vault_path.display()
            );
        }
        Err(e) => {
            anyhow::bail!(
                "Failed to access vault metadata: {} - {}",
                vault_path.display(),
                e
            );
        }
    }

    println!("{}", "🚪 INITIATING GATE SEQUENCE...".bold().cyan());
    println!();

    // Check lifecycle state and mode
    let is_unlocked = config
        .state
        .as_ref()
        .is_some_and(|s| s.is_unlocked);
    let phase_label = if config.is_adhoc_mode() {
        "ADHOC MODE: TASK PLANNING"
    } else if is_unlocked {
        "PHASE 2: ACTIVE SPRINT"
    } else {
        "PHASE 1: PLANNING"
    };
    println!("{} {}", "📍 Lifecycle Phase:".bold(), phase_label.yellow());
    println!();

    // Mode and phase-specific validation
    let all_passed = if config.is_adhoc_mode() {
        // ADHOC MODE: Validate adhoc planning documents
        validate_adhoc_planning(&vault_path, &config)?
    } else {
        // Sprint mode - load heuristics file
        let heuristics_path = vault_path.join(&config.gate.heuristics_file);

        // DEFENSIVE: Check if heuristics file exists
        if !heuristics_path.exists() {
            anyhow::bail!(
                "Heuristics file not found: {}\n  \
                 Check the 'heuristics_file' path in your nexus.toml configuration.",
                heuristics_path.display()
            );
        }

        let heuristics = load_heuristics(&heuristics_path).with_context(|| {
            format!(
                "Failed to parse heuristics file: {}",
                heuristics_path.display()
            )
        })?;

        if is_unlocked {
            // PHASE 2: Active Sprint Validation
            validate_active_sprint(&vault_path, &config)?
        } else {
            // PHASE 1: Planning Document Validation
            validate_planning_phase(&vault_path, &config, &heuristics)?
        }
    };

    println!();
    println!("{}", "━".repeat(60).dimmed());

    // Final verdict
    if all_passed {
        println!();
        println!("{}", "✅ MISSION READY".green().bold());
        println!(
            "{}",
            "   Gate is open. All validation checks passed.".green()
        );
        println!();
        Ok(())
    } else {
        println!();
        println!("{}", "🚫 GATE CLOSED".red().bold());
        println!("{}", "   Fix the issues above before proceeding.".red());
        println!();
        anyhow::bail!("Validation failed")
    }
}

/// Validates planning documents in Phase 1 (Locked) with per-file specific headers
fn validate_planning_phase(
    vault_path: &Path,
    config: &NexusConfig,
    heuristics: &crate::heuristics::GateHeuristics,
) -> Result<bool> {
    let mut all_passed = true;

    // Validate Dashboard (00-START-HERE.md)
    println!("{}", "📋 SCANNING DASHBOARD...".bold());
    let dashboard_path = vault_path
        .join(&config.structure.management_dir)
        .join("00-START-HERE.md");

    // DEFENSIVE: Check if dashboard exists
    if dashboard_path.exists() {
        // DEFENSIVE: Check dashboard file permissions
        match dashboard_path.metadata() {
            Ok(_) => match validate_dashboard_checkboxes(&dashboard_path) {
                Ok(result) => {
                    if result.passed {
                        println!(
                            "  {} Dashboard clean - all tasks completed",
                            "✓".green().bold()
                        );
                    } else {
                        all_passed = false;
                        println!("  {} Dashboard has unchecked items:", "✗".red().bold());
                        print_validation_issues(&result.issues, &dashboard_path);
                    }
                }
                Err(e) => {
                    all_passed = false;
                    let error_msg = if e.to_string().contains("invalid utf-8")
                        || e.to_string().contains("stream did not contain valid UTF-8")
                    {
                        "File contains invalid UTF-8 or binary data".to_string()
                    } else {
                        e.to_string()
                    };
                    println!(
                        "  {} Failed to read dashboard: {}",
                        "✗".red().bold(),
                        error_msg
                    );
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                all_passed = false;
                println!("  {} Dashboard file permission denied", "✗".red().bold());
            }
            Err(e) => {
                all_passed = false;
                println!("  {} Cannot access dashboard: {}", "✗".red().bold(), e);
            }
        }
    } else {
        all_passed = false;
        println!(
            "  {} Dashboard file not found: {}",
            "✗".red().bold(),
            dashboard_path.display()
        );
        println!(
            "     Expected file: 00-START-HERE.md in {}",
            config.structure.management_dir
        );
    }

    println!();

    // Validate Planning Documents (01-PLANNING/*.md) with per-file headers
    println!("{}", "📝 SCANNING PLANNING DOCUMENTS...".bold());
    let planning_dir = vault_path.join(&config.structure.planning_dir);

    if planning_dir.exists() {
        // Use heuristics for validation parameters
        let min_word_count = heuristics.min_section_length as usize;
        let illegal_strings: Vec<String> = heuristics.illegal_strings.clone();

        // File-specific validation rules for structured projects
        // Only use specific file rules if heuristics has required headers
        let use_specific_file_rules = !heuristics.required_headers.is_empty();

        let file_rules: Vec<(&str, Vec<String>)> = if use_specific_file_rules {
            vec![
                (
                    "01-Problem-and-Vision.md",
                    vec!["Problem".to_string(), "Vision".to_string()],
                ),
                (
                    "02-Scope-and-Boundaries.md",
                    vec!["Scope".to_string(), "Boundaries".to_string()],
                ),
                (
                    "03-Tech-Stack.md",
                    vec!["Tech Stack".to_string()],
                ),
                (
                    "04-Architecture.md",
                    vec!["Architecture".to_string()],
                ),
            ]
        } else {
            vec![] // No specific file rules if heuristics doesn't require headers
        };

        // Check if structured files exist - if any exist, require all
        let structured_files_exist = use_specific_file_rules && file_rules.iter().any(|(name, _)| {
            planning_dir.join(name).exists()
        });

        if structured_files_exist {
            // Validate structured planning files
            for (file_name, required_headers) in file_rules {
                let file_path = planning_dir.join(file_name);

                if !file_path.exists() {
                    all_passed = false;
                    println!(
                        "  {} {} - File not found",
                        "✗".red().bold(),
                        file_name
                    );
                    continue;
                }

                // DEFENSIVE: Check for symlink loops and permissions
                match file_path.metadata() {
                    Ok(metadata) => {
                        // Check file size to warn about very large files
                        if metadata.len() > 100_000_000 {
                            // 100MB
                            println!(
                                "  {} {} - File very large ({}MB), may take time to process",
                                "⚠".yellow().bold(),
                                file_name,
                                metadata.len() / 1_000_000
                            );
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                        all_passed = false;
                        println!("  {} {} - Permission denied", "✗".red().bold(), file_name);
                        continue;
                    }
                    Err(e) => {
                        all_passed = false;
                        println!(
                            "  {} {} - Cannot access file: {}",
                            "✗".red().bold(),
                            file_name,
                            e
                        );
                        continue;
                    }
                }

                // Validate with specific headers for this file
                match validate_planning_document_with_headers(
                    &file_path,
                    &required_headers,
                    min_word_count,
                    &illegal_strings,
                ) {
                    Ok(result) => {
                        if result.passed {
                            println!("  {} {}", "✓".green().bold(), file_name);
                        } else {
                            all_passed = false;
                            println!("  {} {}", "✗".red().bold(), file_name);
                            print_validation_issues(&result.issues, &file_path);
                        }
                    }
                    Err(e) => {
                        all_passed = false;
                        let error_msg = if e.to_string().contains("invalid utf-8")
                            || e.to_string().contains("stream did not contain valid UTF-8")
                        {
                            "File contains invalid UTF-8 or binary data".to_string()
                        } else if e.to_string().contains("Permission denied") {
                            "Permission denied".to_string()
                        } else {
                            e.to_string()
                        };
                        println!("  {} {} - {}", "✗".red().bold(), file_name, error_msg);
                    }
                }
            }
        } else {
            // Fallback: validate any .md files found (backward compatibility)
            let planning_files = std::fs::read_dir(&planning_dir)
                .with_context(|| {
                    format!(
                        "Failed to read planning directory: {}",
                        planning_dir.display()
                    )
                })?
                .filter_map(std::result::Result::ok)
                .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("md"))
                .collect::<Vec<_>>();

            if planning_files.is_empty() {
                all_passed = false;
                println!(
                    "  {} No planning documents found in {}",
                    "✗".red().bold(),
                    planning_dir.display()
                );
            } else {
                // Use generic validation with all required headers from heuristics
                for entry in planning_files {
                    let file_path = entry.path();
                    let file_name = if let Some(name) = file_path.file_name() { name.to_string_lossy().to_string() } else {
                        all_passed = false;
                        println!(
                            "  {} Skipping file with invalid path: {}",
                            "⚠".yellow().bold(),
                            file_path.display()
                        );
                        continue;
                    };

                    // DEFENSIVE: Check for symlink loops before reading
                    match file_path.metadata() {
                        Ok(metadata) => {
                            if metadata.len() > 100_000_000 {
                                println!(
                                    "  {} {} - File very large ({}MB), may take time to process",
                                    "⚠".yellow().bold(),
                                    file_name,
                                    metadata.len() / 1_000_000
                                );
                            }
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                            all_passed = false;
                            println!("  {} {} - Permission denied", "✗".red().bold(), file_name);
                            continue;
                        }
                        Err(e) => {
                            all_passed = false;
                            println!(
                                "  {} {} - Cannot access file: {}",
                                "✗".red().bold(),
                                file_name,
                                e
                            );
                            continue;
                        }
                    }

                    // Use generic validation with heuristics
                    match validate_planning_document_with_headers(
                        &file_path,
                        &heuristics.required_headers,
                        min_word_count,
                        &illegal_strings,
                    ) {
                        Ok(result) => {
                            if result.passed {
                                println!("  {} {}", "✓".green().bold(), file_name);
                            } else {
                                all_passed = false;
                                println!("  {} {}", "✗".red().bold(), file_name);
                                print_validation_issues(&result.issues, &file_path);
                            }
                        }
                        Err(e) => {
                            all_passed = false;
                            let error_msg = if e.to_string().contains("invalid utf-8")
                                || e.to_string().contains("stream did not contain valid UTF-8")
                            {
                                "File contains invalid UTF-8 or binary data".to_string()
                            } else if e.to_string().contains("Permission denied") {
                                "Permission denied".to_string()
                            } else {
                                e.to_string()
                            };
                            println!("  {} {} - {}", "✗".red().bold(), file_name, error_msg);
                        }
                    }
                }
            }
        }
    } else {
        all_passed = false;
        println!(
            "  {} Planning directory not found: {}",
            "✗".red().bold(),
            planning_dir.display()
        );
    }

    Ok(all_passed)
}

/// Validates adhoc planning documents for ad-hoc task mode
fn validate_adhoc_planning(_vault_path: &Path, config: &NexusConfig) -> Result<bool> {
    let mut all_passed = true;

    // Load adhoc heuristics - embedded in binary for portability
    const ADHOC_HEURISTICS_JSON: &str = include_str!("../../templates/adhoc/adhoc-gate-heuristics.json");
    let heuristics: crate::heuristics::GateHeuristics = serde_json::from_str(ADHOC_HEURISTICS_JSON)
        .context("Failed to parse embedded adhoc heuristics")?;

    // Validate Dashboard (00-ADHOC-TASK.md) - Planning Phase only
    println!("{}", "📋 SCANNING ADHOC DASHBOARD...".bold());
    let dashboard_path = config.get_adhoc_dashboard_path();

    if dashboard_path.exists() {
        // Read dashboard content and filter for Planning Phase checkboxes only
        match std::fs::read_to_string(&dashboard_path) {
            Ok(content) => {
                let mut planning_phase_section = false;
                let mut unchecked_planning_items = Vec::new();
                let mut line_num = 0;

                for line in content.lines() {
                    line_num += 1;
                    let trimmed = line.trim();

                    // Track which section we're in
                    if trimmed.starts_with("## Planning Phase") {
                        planning_phase_section = true;
                    } else if trimmed.starts_with("## Execution Phase") || trimmed.starts_with("## ") {
                        planning_phase_section = false;
                    }

                    // Check for unchecked checkboxes in Planning Phase
                    if planning_phase_section && trimmed.starts_with("- [ ]") {
                        let task_text = trimmed.strip_prefix("- [ ]").unwrap_or("").trim();
                        unchecked_planning_items.push((line_num, task_text.to_string()));
                    }
                }

                if unchecked_planning_items.is_empty() {
                    println!(
                        "  {} Dashboard planning phase complete",
                        "✓".green().bold()
                    );
                } else {
                    all_passed = false;
                    println!("  {} Dashboard planning phase incomplete:", "✗".red().bold());
                    for (line, task) in unchecked_planning_items {
                        println!("      ▸ Unchecked task at line {line}");
                        println!("         Task: {task}");
                    }
                    println!("      📍 File: {}", dashboard_path.display());
                }
            }
            Err(e) => {
                all_passed = false;
                println!(
                    "  {} Failed to read dashboard: {}",
                    "✗".red().bold(),
                    e
                );
            }
        }
    } else {
        all_passed = false;
        println!(
            "  {} Dashboard not found: {}",
            "✗".red().bold(),
            dashboard_path.display()
        );
    }

    println!();

    // Validate Planning Documents
    println!("{}", "📝 SCANNING ADHOC PLANNING DOCUMENTS...".bold());
    let planning_dir = config.get_adhoc_planning_path();

    if !planning_dir.exists() {
        all_passed = false;
        println!(
            "  {} Planning directory not found: {}",
            "✗".red().bold(),
            planning_dir.display()
        );
        return Ok(all_passed);
    }

    let min_word_count = heuristics.min_section_length as usize;
    let illegal_strings: Vec<String> = heuristics.illegal_strings.clone();

    // Validate Task-Capture.md
    let capture_path = planning_dir.join("Task-Capture.md");
    if capture_path.exists() {
        let required_headers = vec![
            "Source".to_string(),
            "Problem Statement".to_string(),
            "Context".to_string(),
            "Definition of Done".to_string(),
        ];

        match validate_planning_document_with_headers(
            &capture_path,
            &required_headers,
            100, // Min 100 words total
            &illegal_strings,
        ) {
            Ok(result) => {
                if result.passed {
                    println!("  {} Task-Capture.md", "✓".green().bold());
                } else {
                    all_passed = false;
                    println!("  {} Task-Capture.md", "✗".red().bold());
                    print_validation_issues(&result.issues, &capture_path);
                }
            }
            Err(e) => {
                all_passed = false;
                println!("  {} Task-Capture.md - {}", "✗".red().bold(), e);
            }
        }
    } else {
        all_passed = false;
        println!(
            "  {} Task-Capture.md not found",
            "✗".red().bold()
        );
    }

    // Validate Task-Approach.md
    let approach_path = planning_dir.join("Task-Approach.md");
    if approach_path.exists() {
        let required_headers = vec![
            "Analysis".to_string(),
            "Proposed Solution".to_string(),
            "Risks".to_string(),
            "Files to Modify".to_string(),
        ];

        match validate_planning_document_with_headers(
            &approach_path,
            &required_headers,
            150, // Min 150 words total
            &illegal_strings,
        ) {
            Ok(result) => {
                if result.passed {
                    println!("  {} Task-Approach.md", "✓".green().bold());
                } else {
                    all_passed = false;
                    println!("  {} Task-Approach.md", "✗".red().bold());
                    print_validation_issues(&result.issues, &approach_path);
                }
            }
            Err(e) => {
                all_passed = false;
                println!("  {} Task-Approach.md - {}", "✗".red().bold(), e);
            }
        }

        // Additional risk mitigation check
        let content = std::fs::read_to_string(&approach_path).unwrap_or_default();
        if !content.contains("## Risks") || (!content.contains("- [ ]") && !content.contains("- [x]")) {
            all_passed = false;
            println!(
                "  {} Task-Approach.md missing risk mitigation checkboxes",
                "✗".red().bold()
            );
        }
    } else {
        all_passed = false;
        println!(
            "  {} Task-Approach.md not found",
            "✗".red().bold()
        );
    }

    // Validate Task-Validation.md (structure only, don't require checkboxes checked)
    let validation_path = planning_dir.join("Task-Validation.md");
    if validation_path.exists() {
        let required_headers = vec![
            "Pre-Work".to_string(),
            "Implementation".to_string(),
            "Verification".to_string(),
        ];

        match validate_planning_document_with_headers(
            &validation_path,
            &required_headers,
            min_word_count, // Use default min word count
            &illegal_strings,
        ) {
            Ok(result) => {
                if result.passed {
                    println!("  {} Task-Validation.md", "✓".green().bold());
                } else {
                    all_passed = false;
                    println!("  {} Task-Validation.md", "✗".red().bold());
                    print_validation_issues(&result.issues, &validation_path);
                }
            }
            Err(e) => {
                all_passed = false;
                println!("  {} Task-Validation.md - {}", "✗".red().bold(), e);
            }
        }

        // Verify checkbox structure
        let content = std::fs::read_to_string(&validation_path).unwrap_or_default();
        let sections = ["Pre-Work", "Implementation", "Verification"];
        for section in sections {
            if !content.contains(&format!("## {section}")) {
                all_passed = false;
                println!(
                    "  {} Task-Validation.md missing {} section",
                    "✗".red().bold(),
                    section
                );
            }
        }
    } else {
        all_passed = false;
        println!(
            "  {} Task-Validation.md not found",
            "✗".red().bold()
        );
    }

    Ok(all_passed)
}

/// Validates active sprint folder in Phase 2 (Active Sprint)
fn validate_active_sprint(vault_path: &Path, config: &NexusConfig) -> Result<bool> {
    let mut all_passed = true;

    println!("{}", "🎯 SCANNING ACTIVE SPRINT...".bold());

    // Get active sprint from config
    let active_sprint = config
        .state
        .as_ref()
        .and_then(|s| s.active_sprint.as_ref())
        .map(|a| a.current.as_str())
        .ok_or_else(|| {
            anyhow::anyhow!("Active sprint not configured in nexus.toml. Expected state.active_sprint.current")
        })?;
    println!("  {} Sprint: {}", "📌".dimmed(), active_sprint.cyan());

    // Build sprint folder path
    let sprint_folder = vault_path
        .join(&config.structure.sprint_dir)
        .join(active_sprint);

    if !sprint_folder.exists() {
        all_passed = false;
        println!(
            "  {} Sprint folder not found: {}",
            "✗".red().bold(),
            sprint_folder.display()
        );
        return Ok(all_passed);
    }

    // Validate Tasks.md (must have zero unchecked boxes)
    let tasks_path = sprint_folder.join("Tasks.md");
    if tasks_path.exists() {
        match validate_dashboard_checkboxes(&tasks_path) {
            Ok(result) => {
                if result.passed {
                    println!(
                        "  {} Tasks.md - All tasks completed",
                        "✓".green().bold()
                    );
                } else {
                    all_passed = false;
                    println!(
                        "  {} Tasks.md - Has unchecked items:",
                        "✗".red().bold()
                    );
                    print_validation_issues(&result.issues, &tasks_path);
                }
            }
            Err(e) => {
                all_passed = false;
                println!(
                    "  {} Failed to read Tasks.md: {}",
                    "✗".red().bold(),
                    e
                );
            }
        }
    } else {
        all_passed = false;
        println!(
            "  {} Tasks.md not found in sprint folder",
            "✗".red().bold()
        );
    }

    // Validate Sprint-Context.md (must exist and contain content)
    let context_path = sprint_folder.join("Sprint-Context.md");
    if context_path.exists() {
        match std::fs::read_to_string(&context_path) {
            Ok(content) => {
                let trimmed = content.trim();
                if trimmed.is_empty() {
                    all_passed = false;
                    println!(
                        "  {} Sprint-Context.md is empty",
                        "✗".red().bold()
                    );
                } else {
                    println!(
                        "  {} Sprint-Context.md - Contains content ({} bytes)",
                        "✓".green().bold(),
                        trimmed.len()
                    );
                }
            }
            Err(e) => {
                all_passed = false;
                println!(
                    "  {} Failed to read Sprint-Context.md: {}",
                    "✗".red().bold(),
                    e
                );
            }
        }
    } else {
        all_passed = false;
        println!(
            "  {} Sprint-Context.md not found in sprint folder",
            "✗".red().bold()
        );
    }

    Ok(all_passed)
}

/// Print validation issues with ADHD-friendly context
fn print_validation_issues(issues: &[ValidationIssue], file_path: &Path) {
    for issue in issues {
        match issue {
            ValidationIssue::SectionTooShort {
                header,
                word_count,
                required,
            } => {
                println!(
                    "      {} Section '{}' too short: {} words (need {})",
                    "▸".yellow(),
                    header.bold(),
                    word_count.to_string().yellow(),
                    required.to_string().green()
                );
            }
            ValidationIssue::MissingHeader { header } => {
                println!(
                    "      {} Missing required header: {}",
                    "▸".yellow(),
                    header.bold()
                );
            }
            ValidationIssue::IllegalString {
                string,
                context,
                line_estimate,
            } => {
                println!(
                    "      {} Illegal string '{}' at line ~{}",
                    "▸".yellow(),
                    string.bold().red(),
                    line_estimate.to_string().cyan()
                );
                println!("         Context: {}", context.dimmed());
            }
            ValidationIssue::UncheckedCheckbox {
                context,
                line_estimate,
            } => {
                println!(
                    "      {} Unchecked task at line ~{}",
                    "▸".yellow(),
                    line_estimate.to_string().cyan()
                );
                println!("         Task: {}", context.dimmed());
            }
        }
    }
    println!(
        "      {} File: {}",
        "📍".to_string().dimmed(),
        file_path.display().to_string().dimmed()
    );
}
