//! Plan Command Module
//!
//! Implements `nexus plan --init` for PROJECT GENESIS.
//! This is distinct from the Catalyst engine (which handles task-based document generation).
//!
//! DIFFERENTIATION RULE:
//! - Genesis: Creating a brand new project foundation (Docs 02-05)
//! - Catalyst: Surgical task execution (bug fixes, features)
//!
//! GUARDRAIL:
//! - Refuses to run if an active Task Capsule is detected in .nexus/gate-heuristics.json

use crate::config::NexusConfig;
use crate::genesis::GenesisEngine;
use crate::llm::{LlmClient, LlmProvider};
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};

/// Execute the `plan --init` command - PROJECT GENESIS
///
/// This command transforms a 01-Problem-and-Vision.md into a full foundational skeleton.
/// It is strictly for NEW projects and must not interfere with the existing Task workflow.
pub fn execute_init(project_path: &Path) -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║   🏛️  NEXUS PROJECT GENESIS - The God Move         ║".cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".cyan());
    println!();

    // Load project configuration
    let config_path = project_path.join("nexus.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    println!("{} {}", "📂 Project:".bold(), config.project.name.green());
    println!("{} {}", "📁 Obsidian Vault:".bold(), config.project.obsidian_path.dimmed());
    println!();

    // GUARDRAIL: Check for active Task Capsule
    println!("{} {}", "🛡️".bold().yellow(), "Checking for active Task Capsule...".bold());
    enforce_planning_phase(project_path)?;
    println!("  {} No active Task Capsule detected", "✓".green().bold());
    println!();

    // Verify 01-Problem-and-Vision.md exists
    let vision_path = Path::new(&config.project.obsidian_path).join("01-Problem-and-Vision.md");
    if !vision_path.exists() {
        anyhow::bail!(
            "Vision document not found: {}\n\
            Please create 01-Problem-and-Vision.md first using 'nexus init --project'",
            vision_path.display()
        );
    }

    println!("{} Vision document found", "✓".green().bold());
    println!();

    // Verify LLM is configured
    let llm_config = config.llm.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "LLM not configured. Add [llm] section to nexus.toml:\n\n\
            [llm]\n\
            provider = \"gemini\"\n\
            model = \"gemini-1.5-pro\"\n\
            enabled = true\n\n\
            Set your API key:\n\
            export GOGGLE_AI_STUDIO_API_KEY=\"your-key\""
        )
    })?;

    if !llm_config.enabled {
        anyhow::bail!("LLM is disabled. Set 'enabled = true' in [llm] section");
    }

    // Get API key from environment
    let api_key = match llm_config.provider.as_str() {
        "openrouter" => std::env::var("OPENROUTER_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("OPENROUTER_API_KEY not set"))
            })?,
        "claude" | "anthropic" => std::env::var("ANTHROPIC_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))
            })?,
        "gemini" | "google" => std::env::var("GOGGLE_AI_STUDIO_API_KEY")
            .or_else(|_| {
                llm_config
                    .api_key
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("GOGGLE_AI_STUDIO_API_KEY not set"))
            })?,
        _ => anyhow::bail!("Unknown LLM provider: {}", llm_config.provider),
    };

    // Create runtime for async operations
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        // Create LLM client
        let provider = LlmProvider::from_str(&llm_config.provider)
            .ok_or_else(|| anyhow::anyhow!("Invalid provider: {}", llm_config.provider))?;

        let llm_client = LlmClient::new(provider, api_key, llm_config.model.clone());

        // Create Genesis engine
        let obsidian_path = PathBuf::from(&config.project.obsidian_path);
        let engine = GenesisEngine::new(obsidian_path, llm_client)?;

        // Execute Genesis
        engine.execute_genesis().await
    })
}

/// Enforce that the project is in "Planning Phase" (no active Task Capsule)
///
/// # Arguments
/// * `project_path` - Path to the Git repository root (where nexus.toml lives)
///
/// # Returns
/// * `Ok(())` if in planning phase
/// * `Err` if an active Task Capsule is detected
fn enforce_planning_phase(project_path: &Path) -> Result<()> {
    let heuristics_path = project_path.join(".nexus/gate-heuristics.json");

    // If heuristics file doesn't exist, we're definitely in planning phase
    if !heuristics_path.exists() {
        return Ok(());
    }

    // Read and parse heuristics
    let heuristics_content = std::fs::read_to_string(&heuristics_path)
        .with_context(|| format!("Failed to read heuristics: {}", heuristics_path.display()))?;

    let heuristics: serde_json::Value = serde_json::from_str(&heuristics_content)
        .with_context(|| "Failed to parse heuristics JSON")?;

    // Check for active task capsule
    if let Some(active_capsule) = heuristics.get("active_capsule") {
        if !active_capsule.is_null() {
            anyhow::bail!(
                "GENESIS GUARDRAIL VIOLATION\n\
                \n\
                An active Task Capsule is detected:\n\
                  {}\n\
                \n\
                Genesis creates the Rules (Docs 02-05).\n\
                Tasks follow the Rules.\n\
                \n\
                You cannot run Project Genesis while a Task is active.\n\
                Use the task-based workflow instead:\n\
                  • 'nexus task start' - for bug fixes and features\n\
                  • 'catalyst generate' - for task-scoped planning\n\
                \n\
                To proceed with Genesis:\n\
                  1. Complete the active task: 'nexus task done'\n\
                  2. Then run: 'nexus plan --init'",
                active_capsule
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enforce_planning_phase_no_heuristics() {
        // Should pass when no heuristics file exists
        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test_enforce_planning_phase");

        assert!(enforce_planning_phase(&test_path).is_ok());
    }
}
