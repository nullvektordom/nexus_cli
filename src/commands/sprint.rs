//! Sprint Command Implementation
//!
//! Creates a new sprint branch and scaffolds the Obsidian workspace.

use crate::config::{ActiveSprintConfig, NexusConfig};
use crate::git_ops::create_sprint_branch;
use crate::planning::parse_mvp_sprints;
use crate::scaffolding::scaffold_sprint_folder;
use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::Path;

/// Execute the sprint command
///
/// # Arguments
/// * `project_path` - Path to the project directory (where nexus.toml lives)
/// * `sprint_number` - Sprint number to activate
///
/// # Returns
/// * `Ok(())` - Sprint created successfully
/// * `Err` - If config load, parsing, or creation fails
pub fn execute(project_path: &Path, sprint_number: u32) -> Result<()> {
    println!("{}", "🚀 Sprint Orchestrator".bright_cyan().bold());
    println!();

    // Load config from project path
    let config_path = project_path.join("nexus.toml");
    let config_content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read nexus.toml from: {}", project_path.display()))?;

    let mut config = NexusConfig::from_toml(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    // Resolve planning path (where Obsidian vault lives)
    let planning_path = config.get_planning_path();

    println!(
        "{}",
        format!("📂 Project: {}", config.project.name)
            .bright_white()
            .bold()
    );
    println!(
        "{}",
        format!("📁 Planning path: {}", planning_path.display()).bright_black()
    );
    println!();

    // Check active sprint status - enforce sequencing
    if let Some(state) = &config.state
        && let Some(active_sprint) = &state.active_sprint
        && active_sprint.status != "approved"
    {
        println!(
            "{}",
            "❌ SPRINT BLOCKED: Previous sprint not approved"
                .bright_red()
                .bold()
        );
        println!();
        println!(
            "{}",
            format!("  Current active sprint: {}", active_sprint.current).bright_yellow()
        );
        println!(
            "{}",
            format!("  Status: {}", active_sprint.status).bright_yellow()
        );
        println!();
        println!(
            "{}",
            "You must complete and approve the current sprint before starting a new one.".white()
        );
        bail!(
            "Cannot start Sprint {sprint_number} until previous sprint is approved. Please complete the current sprint first."
        );
    }

    // Parse sprint data from MVP breakdown
    println!("{}", "📖 Parsing MVP breakdown...".bright_blue());
    let mvp_path = planning_path
        .join("01-PLANNING")
        .join("05-MVP-Breakdown.md");

    if !mvp_path.exists() {
        bail!(
            "MVP breakdown file not found: {}\nPlease ensure planning documents are complete.",
            mvp_path.display()
        );
    }

    let sprints =
        parse_mvp_sprints(&mvp_path).context("Failed to parse sprints from MVP breakdown")?;

    // Find the requested sprint
    let sprint_data = sprints
        .iter()
        .find(|s| s.number == sprint_number)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Sprint {} not found in MVP breakdown. Available sprints: {}",
                sprint_number,
                sprints
                    .iter()
                    .map(|s| s.number.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    println!(
        "{}",
        format!(
            "  ✓ Found Sprint {}: {}",
            sprint_data.number, sprint_data.title
        )
        .green()
    );
    println!();

    // Create Git branch
    println!("{}", "🌿 Creating git branch...".bright_blue());
    let repo_path = config.get_repo_path();

    create_sprint_branch(&repo_path, sprint_data.number, &sprint_data.name)
        .context("Failed to create sprint branch")?;

    println!(
        "{}",
        format!(
            "  ✓ Branch created: sprint-{}-{}",
            sprint_data.number, sprint_data.name
        )
        .green()
    );
    println!();

    // Scaffold Obsidian folders
    println!("{}", "📁 Scaffolding sprint workspace...".bright_blue());

    scaffold_sprint_folder(&planning_path, sprint_data)
        .context("Failed to scaffold sprint folder")?;

    println!(
        "{}",
        format!(
            "  ✓ Created: 00-MANAGEMENT/sprints/sprint-{}-{}/",
            sprint_data.number, sprint_data.name
        )
        .green()
    );
    println!("{}", "  ✓ Tasks.md".green());
    println!("{}", "  ✓ Sprint-Context.md".green());
    println!("{}", "  ✓ approvals/".green());
    println!("{}", "  ✓ sessions/".green());
    println!();

    // Update nexus.toml with active sprint
    println!("{}", "💾 Updating nexus.toml...".bright_blue());

    if let Some(ref mut state) = config.state {
        state.active_sprint = Some(ActiveSprintConfig {
            current: format!("sprint-{}", sprint_data.number),
            status: "in_progress".to_string(),
        });
    }

    let updated_toml = config.to_toml().context("Failed to serialize config")?;
    fs::write(&config_path, updated_toml)
        .with_context(|| format!("Failed to write config to: {}", config_path.display()))?;

    println!("{}", "  ✓ Active sprint updated".green());
    println!();

    // Success message
    println!("{}", "✅ SPRINT READY".bright_green().bold());
    println!();
    println!(
        "{}",
        format!(
            "Sprint {} is now active: {}",
            sprint_data.number, sprint_data.title
        )
        .bright_white()
    );
    println!();
    println!("{}", "Next steps:".bright_cyan());
    println!(
        "  1. Review tasks in: {}",
        format!(
            "00-MANAGEMENT/sprints/sprint-{}-{}/Tasks.md",
            sprint_data.number, sprint_data.name
        )
        .bright_yellow()
    );
    println!(
        "  2. Check scope boundaries in: {}",
        format!(
            "00-MANAGEMENT/sprints/sprint-{}-{}/Sprint-Context.md",
            sprint_data.number, sprint_data.name
        )
        .bright_yellow()
    );
    println!("  3. Start implementing the tasks!");
    println!();

    Ok(())
}
