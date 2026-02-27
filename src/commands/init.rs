use crate::config::NexusConfig;
use crate::embedded_templates;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Parsed adhoc task path components
struct AdhocTaskPath {
    /// Project name (e.g., "svea_skog")
    project: String,
    /// Task type (e.g., "feature", "bug")
    task_type: String,
    /// Task name (e.g., "my-task")
    task_name: String,
}

impl AdhocTaskPath {
    /// Parse a path string like "project/type/name" into components
    fn parse(path: &str) -> Result<Self, String> {
        let clean = path.trim_matches('"').replace('\\', "/");
        let parts: Vec<&str> = clean.split('/').filter(|s| !s.is_empty()).collect();

        if parts.len() != 3 {
            return Err(format!(
                "Invalid adhoc task path '{path}'.\n\
                 Expected format: <project>/<type>/<name>\n\
                 Example: svea_skog/feature/implement-api"
            ));
        }

        // Validate parts are not empty
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                let names = ["project", "type", "name"];
                return Err(format!("Empty {} in path '{path}'", names[i]));
            }
        }

        Ok(Self {
            project: parts[0].to_string(),
            task_type: parts[1].to_string(),
            task_name: parts[2].to_string(),
        })
    }
}

/// Returns the user's home directory cross-platform
fn home_dir() -> Result<PathBuf, String> {
    dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())
}

/// Prompt user with a yes/no question, returns true for yes
fn prompt_yes_no(question: &str) -> Result<bool, String> {
    print!("{question} [y/N]: ");
    io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {e}"))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {e}"))?;

    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
}

/// Check if a directory contains a git repository
fn is_git_repo(path: &Path) -> bool {
    path.join(".git").is_dir()
}

/// Execute the init command
/// Creates a new project folder with template files and nexus.toml
///
/// # Arguments
/// * `project_name` - Name of the project to create (for adhoc: "project/type/name")
/// * `mode` - "sprint" or "adhoc"
/// * `is_full_project` - If true, creates full project structure (God Move)
/// * `base_dir` - Optional base directory to create project in (defaults to current dir)
/// * `obsidian_root` - Optional Obsidian vault root directory (defaults to ~/obsidian/work)
pub fn execute(
    project_name: &str,
    mode: &str,
    is_full_project: bool,
    base_dir: Option<&Path>,
    obsidian_root: Option<&Path>,
) -> Result<(), String> {
    // Validate mode
    if mode != "sprint" && mode != "adhoc" {
        return Err(format!(
            "Invalid mode '{mode}'. Must be 'sprint' or 'adhoc'."
        ));
    }

    // If --project flag is set, use the God Move initialization
    if is_full_project {
        return init_full_project(project_name, mode, base_dir, obsidian_root);
    }

    // Adhoc mode has completely different logic - branch early
    if mode == "adhoc" {
        return init_adhoc_task(project_name);
    }

    // Sprint mode: create folder in current directory
    // Construct project path: base_dir/project_name or just project_name
    let project_path = if let Some(base) = base_dir {
        base.join(project_name)
    } else {
        PathBuf::from(project_name)
    };

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

    init_sprint_project(&project_path, &folder_name, &absolute_path)?;

    Ok(())
}

/// Initialize a sprint-mode project (original behavior)
fn init_sprint_project(
    project_path: &Path,
    folder_name: &str,
    absolute_path: &Path,
) -> Result<(), String> {
    // Write embedded template files to project directory
    let templates = embedded_templates::get_project_templates();
    embedded_templates::write_templates(&templates, project_path).map_err(|e| {
        // Clean up the created folder on error
        let _ = fs::remove_dir_all(project_path);
        format!("Failed to write template files: {e}")
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

/// Initialize an adhoc task within an existing project
///
/// Expected path format: <project>/<type>/<name>
/// Example: svea_skog/feature/implement-api
///
/// This will:
/// 1. Check ~/obsidian/work/<project>/ exists (prompt to create if not)
/// 2. Check ~/repos/<project>/ exists and is a git repo
/// 3. Create ~/obsidian/work/<project>/<type>/<name>/ with templates
/// 4. Create nexus.toml in ~/repos/<project>/
fn init_adhoc_task(task_path: &str) -> Result<(), String> {
    use colored::Colorize;

    // Parse the task path
    let parsed = AdhocTaskPath::parse(task_path)?;

    let home = home_dir()?;

    // Define base paths
    let obsidian_root = home.join("obsidian").join("work");
    let repos_root = home.join("repos");

    // Project-level paths
    let obsidian_project_path = obsidian_root.join(&parsed.project);
    let repo_path = repos_root.join(&parsed.project);

    // Task-level path (where templates will be created)
    let task_folder_path = obsidian_project_path
        .join(&parsed.task_type)
        .join(&parsed.task_name);

    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║   📋 NEXUS ADHOC TASK INITIALIZATION                  ║".cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════╝".cyan()
    );
    println!();

    // Step 1: Check if Obsidian project folder exists
    println!("{} Checking Obsidian project folder...", "1/4".cyan().bold());
    if !obsidian_project_path.exists() {
        println!(
            "   {} Project folder not found: {}",
            "⚠".yellow(),
            obsidian_project_path.display()
        );

        if prompt_yes_no("   Create project folder?")? {
            fs::create_dir_all(&obsidian_project_path)
                .map_err(|e| format!("Failed to create project folder: {e}"))?;
            println!(
                "   {} Created: {}",
                "✓".green(),
                obsidian_project_path.display()
            );
        } else {
            return Err(format!(
                "Obsidian project folder does not exist: {}\n\
                 Create it manually or run with a valid project name.",
                obsidian_project_path.display()
            ));
        }
    } else {
        println!(
            "   {} Found: {}",
            "✓".green(),
            obsidian_project_path.display()
        );
    }

    // Step 2: Check if repo exists and is a git repository
    println!();
    println!("{} Checking repository...", "2/4".cyan().bold());
    if !repo_path.exists() {
        let mkdir_cmd = if cfg!(windows) {
            format!("mkdir \"{}\"", repo_path.display())
        } else {
            format!("mkdir -p \"{}\"", repo_path.display())
        };
        return Err(format!(
            "Repository folder not found: {}\n\
             \n\
             The adhoc task requires an existing git repository.\n\
             Either:\n\
             • Create the repository: {} && cd \"{}\" && git init\n\
             • Or use 'nexus init {} --project' to create a full project",
            repo_path.display(),
            mkdir_cmd,
            repo_path.display(),
            parsed.project
        ));
    }

    if !is_git_repo(&repo_path) {
        return Err(format!(
            "Directory exists but is not a git repository: {}\n\
             \n\
             Initialize git: cd \"{}\" && git init",
            repo_path.display(),
            repo_path.display()
        ));
    }

    println!("   {} Git repo found: {}", "✓".green(), repo_path.display());

    // Step 3: Check if task folder already exists
    println!();
    println!("{} Creating task folder...", "3/4".cyan().bold());
    if task_folder_path.exists() {
        return Err(format!(
            "Task folder already exists: {}\n\
             Choose a different task name or remove the existing folder.",
            task_folder_path.display()
        ));
    }

    // Create task folder structure
    fs::create_dir_all(&task_folder_path)
        .map_err(|e| format!("Failed to create task folder: {e}"))?;

    let management_dir = task_folder_path.join("00-MANAGEMENT");
    fs::create_dir_all(&management_dir)
        .map_err(|e| format!("Failed to create 00-MANAGEMENT directory: {e}"))?;

    let planning_dir = management_dir.join("adhoc-planning");
    fs::create_dir_all(&planning_dir)
        .map_err(|e| format!("Failed to create adhoc-planning directory: {e}"))?;

    println!(
        "   {} Created: {}",
        "✓".green(),
        task_folder_path.display()
    );

    // Write embedded adhoc templates
    let templates = embedded_templates::get_adhoc_templates();
    embedded_templates::write_templates(&templates, &management_dir).map_err(|e| {
        // Clean up on error
        let _ = fs::remove_dir_all(&task_folder_path);
        format!("Failed to write adhoc templates: {e}")
    })?;

    println!("   {} Copied planning templates", "✓".green());

    // Step 4: Create nexus.toml in the repo
    println!();
    println!("{} Creating nexus.toml...", "4/4".cyan().bold());

    let mut config = NexusConfig::new(
        parsed.task_name.clone(),
        task_folder_path.to_string_lossy().to_string(),
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

    let config_path = repo_path.join("nexus.toml");
    fs::write(&config_path, config_toml)
        .map_err(|e| format!("Failed to write nexus.toml: {e}"))?;

    println!(
        "   {} Created: {}",
        "✓".green(),
        config_path.display()
    );

    // Success message
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════╗".green()
    );
    println!(
        "{}",
        "║       ✅ ADHOC TASK INITIALIZED SUCCESSFULLY!         ║".green()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════╝".green()
    );
    println!();
    println!("{}", "🎯 Task Details:".bold());
    println!("   {} {}", "Project:".bold(), parsed.project);
    println!("   {} {}", "Type:".bold(), parsed.task_type);
    println!("   {} {}", "Task:".bold(), parsed.task_name);
    println!();
    println!("   {} {}", "Repository:".bold(), repo_path.display());
    println!("   {} {}", "Task Folder:".bold(), task_folder_path.display());
    println!();
    println!("{}", "📋 Next steps:".bold());
    println!("   1. cd {}", repo_path.display());
    println!("   2. Fill out planning documents in Obsidian:");
    println!(
        "      {}/00-MANAGEMENT/adhoc-planning/",
        task_folder_path.display()
    );
    println!("   3. Run {} to validate planning", "nexus gate .".cyan());
    println!("   4. Run {} to begin implementation", "nexus task start".cyan());

    Ok(())
}

/// THE "GOD MOVE" - Initialize a complete project from scratch
/// This prevents the "Moment 22" deadlock by setting up everything correctly from day one
fn init_full_project(
    project_name: &str,
    _mode: &str,
    base_dir: Option<&Path>,
    obsidian_root: Option<&Path>,
) -> Result<(), String> {
    use colored::Colorize;

    println!("{}", "╔═══════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║   🏗️  NEXUS PROJECT BOOTSTRAP - THE GOD MOVE          ║".cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".cyan());
    println!();

    // Step 1: Determine project root directory
    let current_dir = if let Some(base) = base_dir {
        base.join(project_name)
    } else {
        std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {e}"))?
    };

    // Create the project directory if it doesn't exist (when using base_dir)
    if base_dir.is_some() && !current_dir.exists() {
        fs::create_dir_all(&current_dir)
            .map_err(|e| format!("Failed to create project directory: {e}"))?;
    }

    println!("{} {}", "📂 Project Root:".bold(), current_dir.display());

    // Step 2: Ask for Obsidian vault location
    println!();
    println!("{}", "📝 Obsidian Vault Configuration".bold());
    println!("   Where should the planning documents be stored?");

    let default_vault = if let Some(vault_root) = obsidian_root {
        vault_root.join(project_name)
    } else {
        home_dir()?
            .join("obsidian")
            .join("work")
            .join(project_name)
    };

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
    git2::Repository::init(&current_dir)
        .map_err(|e| format!("Failed to initialize Git repository: {e}"))?;

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
    copy_project_templates(&vault_path)?;

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

    // Smart .gitignore handling - preserve existing entries and only add missing ones
    let gitignore_path = current_dir.join(".gitignore");
    let required_entries = vec![
        "# Nexus CLI",
        ".nexus_history.json",
        ".nexus_session.json",
        ".env",
        ".nexus/",
        "target/",
        "",
        "# IDE",
        ".vscode/",
        ".idea/",
        "",
        "# OS",
        ".DS_Store",
        "Thumbs.db",
    ];

    let mut final_content = String::new();
    let mut existing_entries = std::collections::HashSet::new();

    // If .gitignore exists, read and preserve its content
    if gitignore_path.exists() {
        if let Ok(existing_content) = fs::read_to_string(&gitignore_path) {
            final_content = existing_content.clone();
            // Track what's already in the file (normalized, trimmed lines)
            for line in existing_content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    existing_entries.insert(trimmed.to_string());
                }
            }
        }
    }

    // Add missing required entries
    let mut added_entries = Vec::new();
    for entry in &required_entries {
        let trimmed = entry.trim();
        // Skip empty lines and comments for the "already exists" check
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            if !existing_entries.contains(trimmed) {
                added_entries.push(*entry);
            }
        } else if entry.starts_with('#') && !existing_entries.contains(trimmed) {
            // Add missing section headers
            added_entries.push(*entry);
        }
    }

    // Append missing entries if any
    if !added_entries.is_empty() {
        if !final_content.is_empty() && !final_content.ends_with('\n') {
            final_content.push('\n');
        }
        if !final_content.is_empty() {
            final_content.push('\n');
        }
        for entry in &added_entries {
            final_content.push_str(entry);
            final_content.push('\n');
        }
    }

    // If file didn't exist, create with full template
    if !gitignore_path.exists() {
        final_content = required_entries.join("\n") + "\n";
    }

    fs::write(&gitignore_path, final_content)
        .map_err(|e| format!("Failed to create .gitignore: {e}"))?;

    if gitignore_path.metadata().map(|m| m.len()).unwrap_or(0) > 0 {
        if added_entries.is_empty() {
            println!("    {} .gitignore already up to date", "✓".green());
        } else {
            println!("    {} .gitignore updated with {} new entries", "✓".green(), added_entries.len());
        }
    } else {
        println!("    {} .gitignore created", "✓".green());
    }

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

/// Copy project templates to the Obsidian vault
fn copy_project_templates(vault_path: &Path) -> Result<(), String> {
    use colored::Colorize;

    // Write embedded templates to vault
    let templates = embedded_templates::get_project_templates();
    embedded_templates::write_templates(&templates, vault_path)
        .map_err(|e| format!("Failed to write templates: {e}"))?;

    // List the files that were created for user feedback
    let planning_files = vec![
        "01-Problem-and-Vision.md",
        "02-Scope-and-Boundaries.md",
        "03-Tech-Stack.md",
        "04-Architecture.md",
        "05-MVP-Breakdown.md",
    ];

    for file in planning_files {
        println!("    {} {}", "✓".green(), file);
    }

    let management_files = vec![
        "00-START-HERE.md",
        "06-PROJECT-UNLOCKED.md",
    ];

    for file in management_files {
        println!("    {} {}", "✓".green(), file);
    }

    println!("    {} 00-MANAGEMENT/ directory", "✓".green());

    // Create empty directories for decisions and dev-sessions
    let decisions_dir = vault_path.join("decisions");
    fs::create_dir_all(&decisions_dir)
        .map_err(|e| format!("Failed to create decisions directory: {e}"))?;
    println!("    {} decisions/ directory", "✓".green());

    let dev_sessions_dir = vault_path.join("dev-sessions");
    fs::create_dir_all(&dev_sessions_dir)
        .map_err(|e| format!("Failed to create dev-sessions directory: {e}"))?;
    println!("    {} dev-sessions/ directory", "✓".green());

    Ok(())
}
