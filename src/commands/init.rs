use crate::config::NexusConfig;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Execute the init command
/// Creates a new project folder with template files and nexus.toml
pub fn execute(project_name: &str, mode: &str) -> Result<(), String> {
    // Validate mode
    if mode != "sprint" && mode != "adhoc" {
        return Err(format!(
            "Invalid mode '{mode}'. Must be 'sprint' or 'adhoc'."
        ));
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
