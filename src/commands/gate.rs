//! Gate Command - The Gatekeeper
//!
//! Validates planning documents and dashboard checkboxes before allowing project unlock.
//! Provides ADHD-friendly terminal output with context snippets and line numbers.

use crate::config::NexusConfig;
use crate::heuristics::load_heuristics;
use crate::planning::{
    validate_dashboard_checkboxes, validate_planning_document, ValidationIssue,
};
use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

/// Execute the gate command
/// Returns Ok(()) if gate passes, Err if validation fails or error occurs
pub fn execute(project_path: &Path) -> Result<()> {
    // Load project configuration
    let config_path = project_path.join("nexus.toml");
    let config_content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    // Check project state for context-aware validation
    if let Some(state) = &config.state {
        if state.is_unlocked {
            // Project is unlocked, check for active sprint
            if let Some(active_sprint) = &state.active_sprint {
                if active_sprint.status == "in_progress" {
                    return validate_sprint_phase(&config, &active_sprint.current);
                } else {
                    println!("{}", "ℹ️  No active sprint in progress.".yellow());
                    println!("   Project is unlocked but no sprint is currently active.");
                    println!("   To start a sprint, use: nexus sprint . <number>");
                    return Ok(());
                }
            } else {
                println!("{}", "ℹ️  Project is unlocked.".yellow());
                println!("   No active sprint found.");
                println!("   To start a sprint, use: nexus sprint . <number>");
                return Ok(());
            }
        }
    }

    // Default: Validate Init Phase (Planning + Dashboard)
    validate_init_phase(&config)
}

/// Validate the initialization phase (Planning Docs + Dashboard)
fn validate_init_phase(config: &NexusConfig) -> Result<()> {
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

    // Load heuristics
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

    println!("{}", "🚪 INITIATING GATE SEQUENCE...".bold().cyan());
    println!();

    let mut all_passed = true;

    // Validate Dashboard (00-START-HERE.md)
    println!("{}", "📋 SCANNING DASHBOARD...".bold());
    let dashboard_path = vault_path
        .join(&config.structure.management_dir)
        .join("00-START-HERE.md");

    // DEFENSIVE: Check if dashboard exists
    if !dashboard_path.exists() {
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
    } else {
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
    }

    println!();

    // Validate Planning Documents (01-PLANNING/*.md)
    println!("{}", "📝 SCANNING PLANNING DOCUMENTS...".bold());
    let planning_dir = vault_path.join(&config.structure.planning_dir);

    if !planning_dir.exists() {
        all_passed = false;
        println!(
            "  {} Planning directory not found: {}",
            "✗".red().bold(),
            planning_dir.display()
        );
    } else {
        // Scan all markdown files in planning directory
        let planning_files = fs::read_dir(&planning_dir)
            .with_context(|| {
                format!(
                    "Failed to read planning directory: {}",
                    planning_dir.display()
                )
            })?
            .filter_map(|entry| entry.ok())
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
            for entry in planning_files {
                let file_path = entry.path();

                // DEFENSIVE: Extract file name safely
                let file_name = match file_path.file_name() {
                    Some(name) => name.to_string_lossy().to_string(),
                    None => {
                        // This should never happen for files from read_dir, but be defensive
                        all_passed = false;
                        println!(
                            "  {} Skipping file with invalid path: {}",
                            "⚠".yellow().bold(),
                            file_path.display()
                        );
                        continue;
                    }
                };

                // DEFENSIVE: Check for symlink loops before reading
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

                match validate_planning_document(&file_path, &heuristics) {
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
                        // Provide specific error messages for common issues
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

/// Validate the Sprint Phase
///
/// Checks:
/// - Tasks.md (all items checked)
/// - Sprint-Context.md (existence)
fn validate_sprint_phase(config: &NexusConfig, current_sprint: &str) -> Result<()> {
    let vault_path = config.get_repo_path();
    // Use configured sprint directory or fallback to standard location
    let sprint_dir_rel = if config.structure.sprint_dir.is_empty() {
        // Fallback if not configured (though it should be)
        Path::new(&config.structure.management_dir).join("sprints")
    } else {
        Path::new(&config.structure.sprint_dir).to_path_buf()
    };

    let sprints_dir = vault_path.join(sprint_dir_rel);

    println!("{}", "🚪 INITIATING SPRINT GATE SEQUENCE...".bold().cyan());
    println!("{}", format!("   Target: {}", current_sprint).cyan());
    println!();

    // Find the sprint folder (e.g., "sprint-4-title" matching "sprint-4")
    // We match "sprint-4-" to avoid matching "sprint-40" when looking for "sprint-4"
    let target_prefix = format!("{}-", current_sprint);

    let sprint_folder = if sprints_dir.exists() {
        fs::read_dir(&sprints_dir)
            .with_context(|| {
                format!(
                    "Failed to read sprints directory: {}",
                    sprints_dir.display()
                )
            })?
            .filter_map(|e| e.ok())
            .find(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                // Match "sprint-4-*" OR exact "sprint-4" (unlikely but possible)
                name == current_sprint || name.starts_with(&target_prefix)
            })
            .map(|e| e.path())
    } else {
        None
    };

    let sprint_folder = match sprint_folder {
        Some(path) => path,
        None => {
            anyhow::bail!(
                "Sprint folder not found for: {}\n  \
                 Expected a folder starting with {} in {}",
                current_sprint,
                current_sprint,
                sprints_dir.display()
            );
        }
    };

    println!(
        "   Found sprint workspace: {}",
        sprint_folder
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .dimmed()
    );
    println!();

    let mut all_passed = true;

    // 1. Validate Tasks.md
    println!("{}", "📋 SCANNING SPRINT TASKS...".bold());
    let tasks_path = sprint_folder.join("Tasks.md");

    if !tasks_path.exists() {
        all_passed = false;
        println!(
            "  {} Tasks file not found: {}",
            "✗".red().bold(),
            tasks_path.display()
        );
    } else {
        match validate_dashboard_checkboxes(&tasks_path) {
            Ok(result) => {
                if result.passed {
                    println!(
                        "  {} All tasks completed",
                        "✓".green().bold()
                    );
                } else {
                    all_passed = false;
                    println!("  {} Unchecked tasks found:", "✗".red().bold());
                    print_validation_issues(&result.issues, &tasks_path);
                }
            }
            Err(e) => {
                all_passed = false;
                println!(
                    "  {} Failed to read tasks file: {}",
                    "✗".red().bold(),
                    e
                );
            }
        }
    }

    println!();

    // 2. Validate Sprint-Context.md
    println!("{}", "📝 SCANNING SPRINT CONTEXT...".bold());
    let context_path = sprint_folder.join("Sprint-Context.md");

    if !context_path.exists() {
        all_passed = false;
        println!(
            "  {} Sprint Context file not found: {}",
            "✗".red().bold(),
            context_path.display()
        );
    } else {
        // Basic check: is it empty?
        match fs::metadata(&context_path) {
            Ok(metadata) => {
                if metadata.len() == 0 {
                    all_passed = false;
                    println!(
                        "  {} Sprint Context file is empty",
                        "✗".red().bold()
                    );
                } else {
                    println!(
                        "  {} Sprint Context present",
                        "✓".green().bold()
                    );
                }
            }
            Err(e) => {
                all_passed = false;
                println!(
                    "  {} Cannot access Sprint Context: {}",
                    "✗".red().bold(),
                    e
                );
            }
        }
    }

    println!();
    println!("{}", "━".repeat(60).dimmed());

    // Final verdict
    if all_passed {
        println!();
        println!("{}", "✅ SPRINT COMPLETE".green().bold());
        println!(
            "{}",
            "   Gate is open. All sprint tasks verified.".green()
        );
        println!();
        Ok(())
    } else {
        println!();
        println!("{}", "🚫 SPRINT INCOMPLETE".red().bold());
        println!("{}", "   Complete all tasks before finishing the sprint.".red());
        println!();
        anyhow::bail!("Sprint validation failed")
    }
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
