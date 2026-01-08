use crate::config::NexusConfig;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Execute the init command
/// Creates a new project folder with template files and nexus.toml
pub fn execute(project_name: &str, mode: &str, is_full_project: bool) -> Result<(), String> {
    // Validate mode
    if mode != "sprint" && mode != "adhoc" {
        return Err(format!(
            "Invalid mode '{mode}'. Must be 'sprint' or 'adhoc'."
        ));
    }

    // If --project flag is set, use the God Move initialization
    if is_full_project {
        return init_full_project(project_name, mode);
    }
    let project_path = PathBuf::from(project_name);

    // Extract the folder name from the path (for use as project_name in config)
    let folder_name = project_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("Invalid project path: {project_name}"))?
        .to_string();

    // Check if folder already exists
    if project_path.exists() {
        return Err(format!(
            "Error: Project folder '{project_name}' already exists. Please choose a different name or remove the existing folder."
        ));
    }

    // Create the project folder
    fs::create_dir(&project_path)
        .map_err(|e| format!("Failed to create project folder '{project_name}': {e}"))?;

    println!("✓ Created project folder: {folder_name}");

    // Get absolute path early for config creation
    let absolute_path = project_path
        .canonicalize()
        .map_err(|e| format!("Failed to resolve absolute path: {e}"))?;

    // Branch based on mode
    if mode == "adhoc" {
        init_adhoc_project(&project_path, &folder_name, &absolute_path)?;
    } else {
        init_sprint_project(&project_path, &folder_name, &absolute_path)?;
    }

    Ok(())
}

/// Initialize a sprint-mode project (original behavior)
fn init_sprint_project(
    project_path: &Path,
    folder_name: &str,
    absolute_path: &Path,
) -> Result<(), String> {
    // Copy template files from templates/project
    let template_source = Path::new("templates/project");
    if !template_source.exists() {
        // Clean up the created folder on error
        let _ = fs::remove_dir(project_path);
        return Err(format!(
            "Error: Template directory not found at '{}'",
            template_source.display()
        ));
    }

    copy_dir_recursive(template_source, project_path).map_err(|e| {
        // Clean up the created folder on error
        let _ = fs::remove_dir_all(project_path);
        format!("Failed to copy template files: {e}")
    })?;

    println!("✓ Copied template files");

    // Create nexus.toml configuration (sprint mode - no tasks config)
    let config = NexusConfig::new(
        folder_name.to_string(),
        absolute_path.to_string_lossy().to_string(),
    );

    let config_toml = config
        .to_toml()
        .map_err(|e| format!("Failed to serialize config: {e}"))?;

    let config_path = project_path.join("nexus.toml");
    fs::write(&config_path, config_toml)
        .map_err(|e| format!("Failed to write nexus.toml: {e}"))?;

    println!("✓ Created nexus.toml");
    println!("\n✅ Project '{folder_name}' initialized successfully!");
    println!("   Location: {}", absolute_path.display());
    println!("\nNext steps:");
    println!("   1. cd {}", absolute_path.display());
    println!("   2. Open 00-START-HERE.md and follow the planning workflow");

    Ok(())
}

/// Initialize an adhoc-mode project
fn init_adhoc_project(
    project_path: &Path,
    folder_name: &str,
    absolute_path: &Path,
) -> Result<(), String> {
    // For adhoc mode, we need an Obsidian vault path
    // Default to user's home/obsidian/work/project_name
    let home_dir = std::env::var("HOME")
        .map_err(|_| "Could not determine HOME directory".to_string())?;

    let obsidian_vault_path = PathBuf::from(&home_dir)
        .join("obsidian")
        .join("work")
        .join(folder_name);

    // Create Obsidian vault directory structure if it doesn't exist
    fs::create_dir_all(&obsidian_vault_path)
        .map_err(|e| format!("Failed to create Obsidian vault directory: {e}"))?;

    let management_dir = obsidian_vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir)
        .map_err(|e| format!("Failed to create 00-MANAGEMENT directory: {e}"))?;

    let planning_dir = management_dir.join("adhoc-planning");
    fs::create_dir(&planning_dir)
        .map_err(|e| format!("Failed to create adhoc-planning directory: {e}"))?;

    println!("✓ Created Obsidian vault structure at: {}", obsidian_vault_path.display());

    // Copy adhoc planning templates to Obsidian vault
    let template_source = Path::new("templates/adhoc");
    if !template_source.exists() {
        let _ = fs::remove_dir_all(project_path);
        let _ = fs::remove_dir_all(&obsidian_vault_path);
        return Err(format!(
            "Error: Adhoc template directory not found at '{}'",
            template_source.display()
        ));
    }

    // Copy Task-Capture.md
    fs::copy(
        template_source.join("Task-Capture.md"),
        planning_dir.join("Task-Capture.md"),
    )
    .map_err(|e| format!("Failed to copy Task-Capture.md: {e}"))?;

    // Copy Task-Approach.md
    fs::copy(
        template_source.join("Task-Approach.md"),
        planning_dir.join("Task-Approach.md"),
    )
    .map_err(|e| format!("Failed to copy Task-Approach.md: {e}"))?;

    // Copy Task-Validation.md
    fs::copy(
        template_source.join("Task-Validation.md"),
        planning_dir.join("Task-Validation.md"),
    )
    .map_err(|e| format!("Failed to copy Task-Validation.md: {e}"))?;

    println!("✓ Copied planning templates to Obsidian vault");

    // Copy dashboard to 00-MANAGEMENT/
    fs::copy(
        template_source.join("00-ADHOC-TASK.md"),
        management_dir.join("00-ADHOC-TASK.md"),
    )
    .map_err(|e| format!("Failed to copy dashboard: {e}"))?;

    println!("✓ Created task dashboard in Obsidian vault");

    // Create nexus.toml in the repo pointing to Obsidian vault
    let mut config = NexusConfig::new(
        folder_name.to_string(),
        obsidian_vault_path.to_string_lossy().to_string(), // Point to Obsidian vault
    );

    // Set adhoc mode
    config.tasks = Some(crate::config::TasksConfig {
        mode: "adhoc".to_string(),
        adhoc_planning_dir: "adhoc-planning".to_string(),
        adhoc_dashboard: "00-ADHOC-TASK.md".to_string(),
    });

    let config_toml = config
        .to_toml()
        .map_err(|e| format!("Failed to serialize config: {e}"))?;

    let config_path = project_path.join("nexus.toml");
    fs::write(&config_path, config_toml)
        .map_err(|e| format!("Failed to write nexus.toml: {e}"))?;

    println!("✓ Created nexus.toml in repo (points to Obsidian vault)");
    println!("\n✅ Adhoc task '{folder_name}' initialized successfully!");
    println!("   Repo location: {}", absolute_path.display());
    println!("   Obsidian vault: {}", obsidian_vault_path.display());
    println!("\nNext steps:");
    println!("   1. cd {}", absolute_path.display());
    println!("   2. Fill out planning documents in Obsidian vault:");
    println!("      {}/00-MANAGEMENT/adhoc-planning/", obsidian_vault_path.display());
    println!("   3. Run 'nexus gate .' to validate planning");
    println!("   4. Run 'nexus task start' to begin implementation");

    Ok(())
}

/// Recursively copy a directory and its contents
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    // Create destination directory if it doesn't exist
    if !dst.exists() {
        fs::create_dir(dst)?;
    }

    // Iterate over entries in the source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if file_type.is_dir() {
            // Recursively copy subdirectory
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            // Copy file
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// THE "GOD MOVE" - Initialize a complete project from scratch
/// This prevents the "Moment 22" deadlock by setting up everything correctly from day one
fn init_full_project(project_name: &str, _mode: &str) -> Result<(), String> {
    use colored::Colorize;

    println!("{}", "╔═══════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║   🏗️  NEXUS PROJECT BOOTSTRAP - THE GOD MOVE       ║".cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".cyan());
    println!();

    // Step 1: Get current directory as project root
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {e}"))?;

    println!("{} {}", "📂 Project Root:".bold(), current_dir.display());

    // Step 2: Ask for Obsidian vault location
    println!();
    println!("{}", "📝 Obsidian Vault Configuration".bold());
    println!("   Where should the planning documents be stored?");

    let home_dir = std::env::var("HOME")
        .map_err(|_| "Could not determine HOME directory".to_string())?;

    let default_vault = PathBuf::from(&home_dir)
        .join("obsidian")
        .join("work")
        .join(project_name);

    println!("   Default: {}", default_vault.display().to_string().dimmed());
    println!("   Press Enter to use default, or type custom path:");

    let mut vault_input = String::new();
    io::stdin()
        .read_line(&mut vault_input)
        .map_err(|e| format!("Failed to read input: {e}"))?;

    let vault_path = if vault_input.trim().is_empty() {
        default_vault
    } else {
        PathBuf::from(vault_input.trim())
    };

    println!();
    println!("{} Initializing Git repository...", "1/5".cyan().bold());

    // Initialize Git repository in current directory
    let git_status = std::process::Command::new("git")
        .args(["init"])
        .current_dir(&current_dir)
        .output()
        .map_err(|e| format!("Failed to execute git init: {e}"))?;

    if !git_status.status.success() {
        return Err("Failed to initialize Git repository".to_string());
    }

    println!("    {} Git repository initialized", "✓".green());

    println!();
    println!("{} Creating .nexus/ directory...", "2/5".cyan().bold());

    // Create .nexus directory for local configs
    let nexus_dir = current_dir.join(".nexus");
    fs::create_dir_all(&nexus_dir)
        .map_err(|e| format!("Failed to create .nexus directory: {e}"))?;

    println!("    {} .nexus/ directory created", "✓".green());

    // Create bootstrap heuristics
    let heuristics_path = nexus_dir.join("gate-heuristics.json");
    crate::heuristics::create_bootstrap_heuristics(&heuristics_path)
        .map_err(|e| format!("Failed to create bootstrap heuristics: {e}"))?;

    println!("    {} Bootstrap heuristics created", "✓".green());

    println!();
    println!("{} Scaffolding Obsidian vault...", "3/5".cyan().bold());

    // Create Obsidian vault structure
    fs::create_dir_all(&vault_path)
        .map_err(|e| format!("Failed to create Obsidian vault: {e}"))?;

    println!("    {} Vault directory: {}", "✓".green(), vault_path.display());

    // Create planning documents (01-05)
    create_planning_documents(&vault_path)?;

    println!();
    println!("{} Creating nexus.toml...", "4/5".cyan().bold());

    // Create nexus.toml
    let config = NexusConfig::new(
        project_name.to_string(),
        vault_path.to_string_lossy().to_string(),
    );

    let mut config_toml = config
        .to_toml()
        .map_err(|e| format!("Failed to serialize config: {e}"))?;

    // Override heuristics_file path to use stable location
    config_toml = config_toml.replace(
        "heuristics_file = \"Gate-Heuristics.json\"",
        "heuristics_file = \".nexus/gate-heuristics.json\""
    );

    let config_path = current_dir.join("nexus.toml");
    fs::write(&config_path, config_toml)
        .map_err(|e| format!("Failed to write nexus.toml: {e}"))?;

    println!("    {} nexus.toml created", "✓".green());

    println!();
    println!("{} Creating .gitignore...", "5/5".cyan().bold());

    // Create .gitignore
    let gitignore_content = "# Nexus CLI
.nexus_history.json
.nexus_session.json

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db
";

    fs::write(current_dir.join(".gitignore"), gitignore_content)
        .map_err(|e| format!("Failed to create .gitignore: {e}"))?;

    println!("    {} .gitignore created", "✓".green());

    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════╗".green());
    println!("{}", "║       ✅ PROJECT INITIALIZED SUCCESSFULLY!           ║".green());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".green());
    println!();
    println!("{}", "🎯 Your project is ready!".bold());
    println!();
    println!("   {} {}", "Git Repo:".bold(), current_dir.display());
    println!("   {} {}", "Obsidian Vault:".bold(), vault_path.display());
    println!("   {} .nexus/gate-heuristics.json", "Heuristics:".bold());
    println!();
    println!("{}", "📋 Next steps:".bold());
    println!("   1. Open {} in Obsidian", vault_path.display());
    println!("   2. Fill out {}", "01-Problem-and-Vision.md".cyan());
    println!("   3. Run {} to verify planning", "nexus gate .".cyan());
    println!("   4. Run {} to generate CLAUDE.md", "nexus unlock .".cyan());
    println!();

    Ok(())
}

/// Create the 01-05 planning documents in the Obsidian vault
fn create_planning_documents(vault_path: &Path) -> Result<(), String> {
    use colored::Colorize;

    // Create 01-PLANNING directory
    let planning_dir = vault_path;
    fs::create_dir_all(planning_dir)
        .map_err(|e| format!("Failed to create planning directory: {e}"))?;

    // Document 1: Problem and Vision
    let doc1 = r#"# Problem and Vision

## Problem
*What problem are we solving? Who experiences this problem? Be specific about the pain points.*



## Vision
*What does success look like? What's the ideal future state after solving this problem?*



---
*Generated by Nexus CLI - Edit this document to define your project vision*
*Note: You can add additional sections like Target Users or Success Metrics if needed*
"#;

    fs::write(planning_dir.join("01-Problem-and-Vision.md"), doc1)
        .map_err(|e| format!("Failed to write 01-Problem-and-Vision.md: {e}"))?;

    println!("    {} 01-Problem-and-Vision.md", "✓".green());

    // Document 2: Scope and Boundaries
    let doc2 = r#"# Scope and Boundaries

## Scope
*What will we build? List specific features and capabilities included in the MVP.*



## Boundaries
*What will we NOT build? List features that are explicitly excluded or deferred.*



---
*Generated by Nexus CLI - Define clear boundaries for your project*
*Note: Focus on MVP scope - what's the minimum needed to validate the core value*
"#;

    fs::write(planning_dir.join("02-Scope-and-Boundaries.md"), doc2)
        .map_err(|e| format!("Failed to write 02-Scope-and-Boundaries.md: {e}"))?;

    println!("    {} 02-Scope-and-Boundaries.md", "✓".green());

    // Document 3: Tech Stack
    let doc3 = r#"# Tech Stack

## Tech Stack
*Complete technology stack including:*
*- Primary programming language and version*
*- Frameworks and libraries*
*- Database and data storage*
*- Deployment platform*
*- Development tools*
*Justify choices based on your project's constraints and vision.*



---
*Generated by Nexus CLI - Define your technical stack and justify choices*
"#;

    fs::write(planning_dir.join("03-Tech-Stack.md"), doc3)
        .map_err(|e| format!("Failed to write 03-Tech-Stack.md: {e}"))?;

    println!("    {} 03-Tech-Stack.md", "✓".green());

    // Document 4: Architecture
    let doc4 = r#"# Architecture

## Architecture
*System architecture including:*
*- High-level system design*
*- Core components and their responsibilities*
*- Data flow and communication patterns*
*- Key design decisions and rationale*
*- Technology integration points*



---
*Generated by Nexus CLI - Document your technical architecture*
"#;

    fs::write(planning_dir.join("04-Architecture.md"), doc4)
        .map_err(|e| format!("Failed to write 04-Architecture.md: {e}"))?;

    println!("    {} 04-Architecture.md", "✓".green());

    // Create 00-MANAGEMENT directory for task tracking
    let management_dir = vault_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir)
        .map_err(|e| format!("Failed to create 00-MANAGEMENT directory: {e}"))?;

    println!("    {} 00-MANAGEMENT/ directory", "✓".green());

    Ok(())
}
