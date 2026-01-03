//! Gate Command - The Gatekeeper
//!
//! Validates planning documents and dashboard checkboxes before allowing project unlock.
//! Provides ADHD-friendly terminal output with context snippets and line numbers.

use crate::config::NexusConfig;
use crate::heuristics::load_heuristics;
use crate::planning::{ValidationIssue, validate_dashboard_checkboxes, validate_planning_document};
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
        let planning_files = std::fs::read_dir(&planning_dir)
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
