//! Task Command Module
//!
//! Implements `task start` and `task done` commands for ad-hoc task management.
//! Enforces gate validation before starting tasks and validation completeness before marking tasks done.

use crate::commands::gate;
use crate::config::NexusConfig;
use crate::planning::{update_dashboard_execution_complete, update_dashboard_planning_complete, validate_all_checkboxes_checked};
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::Path;

/// Execute the `task start` command
///
/// This is a PRIVILEGED COMMAND that bypasses strict mode gate checks.
/// It ensures heuristics exist (creating bootstrap if needed) before running gate validation.
pub fn execute_start(project_path: &Path) -> Result<()> {
    // Load project configuration
    let config_path = project_path.join("nexus.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    // Verify adhoc mode
    if !config.is_adhoc_mode() {
        anyhow::bail!("Task commands are only available in adhoc mode");
    }

    println!("{} {}", "🚀".bold().cyan(), "INITIATING TASK START SEQUENCE...".bold());
    println!();

    // PRIVILEGED: Ensure heuristics exist before gate check
    // This prevents the "Moment 22" deadlock
    let stable_heuristics_path = config.get_stable_heuristics_path();
    if !stable_heuristics_path.exists() {
        println!("{} {}", "📦".bold().cyan(), "Bootstrap: Creating heuristics file...".bold());
        crate::heuristics::create_bootstrap_heuristics(&stable_heuristics_path)
            .context("Failed to create bootstrap heuristics")?;
        println!(
            "  {} Created: {}",
            "✓".green().bold(),
            stable_heuristics_path.display().to_string().dimmed()
        );
        println!();
    }

    // Run gate validation internally
    println!("{} {}", "🔒".bold().yellow(), "Running gate validation...".bold());
    match gate::execute(project_path) {
        Ok(_) => println!("  {} Gate validation passed", "✓".green().bold()),
        Err(e) => {
            println!("  {} Gate validation failed: {}", "✗".red().bold(), e);
            anyhow::bail!("Cannot start task without passing gate validation");
        }
    }

    println!();

    // Update dashboard with planning completion timestamp
    let dashboard_path = config.get_adhoc_dashboard_path();
    match update_dashboard_planning_complete(&dashboard_path) {
        Ok(_) => println!("  {} Updated dashboard with planning completion timestamp", "✓".green().bold()),
        Err(e) => {
            println!("  {} Failed to update dashboard: {}", "✗".red().bold(), e);
            anyhow::bail!("Failed to update dashboard");
        }
    }

    println!();
    println!("{} {}", "✅".bold().green(), "TASK STARTED SUCCESSFULLY".bold());
    println!("  You may now begin implementation");
    Ok(())
}

/// Execute the `task done` command
pub fn execute_done(project_path: &Path) -> Result<()> {
    // Load project configuration
    let config_path = project_path.join("nexus.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    // Verify adhoc mode
    if !config.is_adhoc_mode() {
        anyhow::bail!("Task commands are only available in adhoc mode");
    }

    println!("{} {}", "🏁".bold().cyan(), "INITIATING TASK COMPLETION SEQUENCE...".bold());
    println!();

    // Verify task was started
    let dashboard_path = config.get_adhoc_dashboard_path();
    if !dashboard_path.exists() {
        anyhow::bail!("Dashboard not found");
    }

    let dashboard_content = std::fs::read_to_string(&dashboard_path)?;
    if !dashboard_content.contains("Planning completed:") {
        anyhow::bail!("Task has not been started yet");
    }

    println!("  {} Task was properly started", "✓".green().bold());

    // Validate Task-Validation.md
    let validation_path = config.get_adhoc_planning_path().join("Task-Validation.md");
    println!("{} {}", "✅".bold().yellow(), "Validating task completion...".bold());
    match validate_all_checkboxes_checked(&validation_path) {
        Ok(true) => println!("  {} All validation checkboxes completed", "✓".green().bold()),
        Ok(false) => {
            println!("  {} Validation incomplete", "✗".red().bold());
            anyhow::bail!("Cannot mark task done with incomplete validation");
        }
        Err(e) => {
            println!("  {} Validation failed: {}", "✗".red().bold(), e);
            anyhow::bail!("Validation check failed");
        }
    }

    // Update dashboard with task completion timestamp
    match update_dashboard_execution_complete(&dashboard_path) {
        Ok(_) => println!("  {} Updated dashboard with task completion timestamp", "✓".green().bold()),
        Err(e) => {
            println!("  {} Failed to update dashboard: {}", "✗".red().bold(), e);
            anyhow::bail!("Failed to update dashboard");
        }
    }

    println!();
    println!("{} {}", "🎉".bold().green(), "TASK COMPLETED SUCCESSFULLY".bold());
    println!("  Ready for review and merge");
    Ok(())
}
