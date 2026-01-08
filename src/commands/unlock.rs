//! Unlock Command - Generate CLAUDE.md from Planning Documents
//!
//! This command integrates gate checking, content extraction, template rendering,
//! and git initialization to unlock a fully-planned project.

use anyhow::{Context, Result};
use colored::Colorize;
use git2::{Repository, Signature};
use std::path::Path;

use crate::commands;
use crate::config::NexusConfig;
use crate::planning::parse_planning_documents;
use crate::templating::generate_claude_md;

/// Execute the unlock command
///
/// # Flow:
/// 1. Load config from `project_path`
/// 2. Run gate check - abort if planning incomplete
/// 3. Parse planning documents into context
/// 4. Generate CLAUDE.md via templating
/// 5. Initialize git repo and commit
///
/// # Arguments
/// * `project_path` - Path to the project directory (where nexus.toml lives)
///
/// # Returns
/// * `Ok(())` - Project successfully unlocked
/// * `Err` - Gate check failed or generation error
pub fn execute(project_path: &Path) -> Result<()> {
    println!("{}", "🔓 INITIATING UNLOCK SEQUENCE...".cyan().bold());
    println!();

    // Phase 1: Load Configuration
    println!("{}", "📋 Loading configuration...".cyan().bold());
    let config_path = project_path.join("nexus.toml");
    let config_content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from: {}", config_path.display()))?;
    let config: NexusConfig = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse config from: {}", config_path.display()))?;

    println!("  ✓ Config loaded: {}", config.project.name.green());
    println!();

    // Phase 2: Safety Check - Gate Integration
    println!("{}", "🚪 Running gate check...".cyan().bold());
    match commands::gate::execute(project_path) {
        Ok(()) => {
            println!();
            println!(
                "{}",
                "  ✓ Gate check passed - planning complete".green().bold()
            );
        }
        Err(e) => {
            eprintln!();
            eprintln!("{}", "🚫 UNLOCK ABORTED".red().bold());
            eprintln!("  Gate check failed. Fix planning issues before unlocking.");
            eprintln!();
            return Err(e).context("Gate check failed - planning incomplete");
        }
    }
    println!();

    // Phase 3: Resolve Planning Path
    let planning_dir = config.get_planning_path();

    if !planning_dir.exists() {
        anyhow::bail!(
            "Planning directory not found: {}",
            planning_dir.display()
        );
    }

    // Phase 4: Parse Planning Documents
    println!("{}", "🔧 Parsing planning documents...".cyan().bold());
    let context =
        parse_planning_documents(&planning_dir).context("Failed to parse planning documents")?;
    println!("  ✓ Planning documents parsed");
    println!("    📍 Project: {}", context.project_name.dimmed());
    println!();

    // Phase 5: Generate CLAUDE.md
    println!("{}", "📝 Generating CLAUDE.md...".cyan().bold());
    let repo_path = config.get_repo_path();

    // Use custom template if specified in config
    let template_path = config.templates.as_ref().and_then(|t| {
        let p = repo_path.join(&t.claude_template);
        if p.exists() { Some(p) } else { None }
    });

    generate_claude_md(&context, &repo_path, template_path.as_deref())
        .context("Failed to generate CLAUDE.md")?;

    let claude_path = repo_path.join("CLAUDE.md");
    println!("  ✓ CLAUDE.md generated");
    println!("    📍 {}", claude_path.display().to_string().dimmed());
    println!();

    // Phase 6: Git Initialization
    init_git_repo(&repo_path, &claude_path)?;
    println!();

    // Phase 7: Success Output
    println!("{}", "━".repeat(60).dimmed());
    println!("{}", "✅ PROJECT UNLOCKED".green().bold());
    println!("{}", "━".repeat(60).dimmed());
    println!();
    println!("📋 Summary:");
    println!("  • CLAUDE.md generated at: {}", claude_path.display());
    println!("  • Git repository initialized");
    println!("  • Initial commit created with planning docs");
    println!();
    println!("🚀 Next Steps:");
    println!("  1. Review CLAUDE.md in your repository");
    println!("  2. Share CLAUDE.md with your AI assistant (Claude, etc.)");
    println!("  3. Start development with clear context and constraints");
    println!();
    println!("{}", "━".repeat(60).dimmed());

    Ok(())
}

/// Initialize git repository and create initial commit
///
/// # Idempotency
/// - Skips init if .git already exists
/// - Skips commit if repo already has commits
///
/// # Arguments
/// * `repo_path` - Root directory of the repository
/// * `claude_path` - Path to CLAUDE.md for staging
fn init_git_repo(repo_path: &Path, claude_path: &Path) -> Result<()> {
    let git_dir = repo_path.join(".git");

    let repo = if git_dir.exists() {
        println!("{}", "🔧 Git repository already exists...".cyan().bold());
        Repository::open(repo_path).context("Failed to open existing git repository")?
    } else {
        println!("{}", "🔧 Initializing git repository...".cyan().bold());
        Repository::init(repo_path).context("Failed to initialize git repository")?
    };

    // Check if repo already has commits
    let has_commits = repo.head().is_ok();

    if has_commits {
        println!(
            "{}",
            "  ℹ Repository already has commits, skipping initial commit".yellow()
        );
        return Ok(());
    }

    // Stage CLAUDE.md and planning files
    let mut index = repo.index().context("Failed to get repository index")?;

    // Stage CLAUDE.md
    let claude_relative = claude_path
        .strip_prefix(repo_path)
        .context("Failed to get relative path for CLAUDE.md")?;
    index
        .add_path(claude_relative)
        .context("Failed to stage CLAUDE.md")?;

    // Stage planning directory if it exists in repo
    let planning_dir = repo_path.join("01-PLANNING");
    if planning_dir.exists() {
        index
            .add_all(["01-PLANNING"].iter(), git2::IndexAddOption::DEFAULT, None)
            .context("Failed to stage planning directory")?;
    }

    index.write().context("Failed to write index")?;

    println!("  ✓ Files staged for commit");

    // Create tree from index
    let tree_id = index.write_tree().context("Failed to write tree")?;
    let tree = repo.find_tree(tree_id).context("Failed to find tree")?;

    // Create commit
    let signature =
        Signature::now("Nexus CLI", "nexus@local").context("Failed to create git signature")?;

    let message = "Initial commit: Generated CLAUDE.md from planning docs\n\n\
                   Project planning complete and validated by nexus gate.\n\
                   CLAUDE.md provides permanent context for AI-assisted development.";

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[], // No parent commits (initial commit)
    )
    .context("Failed to create initial commit")?;

    println!("  ✓ Initial commit created");

    Ok(())
}
