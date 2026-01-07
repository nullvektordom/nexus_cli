use crate::config::NexusConfig;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Execute the init command
/// Creates a new project folder with template files and nexus.toml
pub fn execute(project_name: &str) -> Result<(), String> {
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

    // Copy template files
    let template_source = Path::new("templates/project");
    if !template_source.exists() {
        // Clean up the created folder on error
        let _ = fs::remove_dir(&project_path);
        return Err(format!(
            "Error: Template directory not found at '{}'",
            template_source.display()
        ));
    }

    copy_dir_recursive(template_source, &project_path).map_err(|e| {
        // Clean up the created folder on error
        let _ = fs::remove_dir_all(&project_path);
        format!("Failed to copy template files: {e}")
    })?;

    println!("✓ Copied template files");

    // Create nexus.toml configuration
    let absolute_path = project_path
        .canonicalize()
        .map_err(|e| format!("Failed to resolve absolute path: {e}"))?;

    let config = NexusConfig::new(
        folder_name.clone(),
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
