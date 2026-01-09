//! Git Operations for Sprint Management
//!
//! Provides utilities for creating and managing sprint branches using git2.

use anyhow::{Context, Result, bail};
use git2::{BranchType, Repository};
use std::path::Path;

/// Create a new sprint branch and switch to it
///
/// # Arguments
/// * `repo_path` - Path to the git repository
/// * `sprint_number` - Sprint number (e.g., 4)
/// * `sprint_name` - Sprint slug (e.g., "the-sprint-orchestrator")
///
/// # Returns
/// * `Ok(())` - Branch created and checked out successfully
/// * `Err` - If repo is dirty, branch already exists, or git operation fails
///
/// # Requirements
/// - Repository must have a clean working directory (no uncommitted changes)
/// - Branch name must not already exist
/// - Creates branch from current HEAD
/// - Automatically switches to the new branch
pub fn create_sprint_branch(repo_path: &Path, sprint_number: u32, sprint_name: &str) -> Result<()> {
    // Open repository
    let repo = Repository::open(repo_path)
        .with_context(|| format!("Failed to open git repository at: {}", repo_path.display()))?;

    // Check if working directory is clean
    ensure_clean_working_directory(&repo)?;

    // Generate branch name following common convention: feature/sprint-N-description
    let branch_name = format!("feature/sprint-{sprint_number}-{sprint_name}");

    // Check if branch already exists
    if branch_exists(&repo, &branch_name)? {
        bail!(
            "Branch '{branch_name}' already exists. Please delete it first or use a different sprint."
        );
    }

    // Get current HEAD commit
    let head = repo.head().context("Failed to get HEAD reference")?;
    let head_commit = head
        .peel_to_commit()
        .context("Failed to peel HEAD to commit")?;

    // Create new branch
    repo.branch(&branch_name, &head_commit, false)
        .with_context(|| format!("Failed to create branch '{branch_name}'"))?;

    // Checkout the new branch
    checkout_branch(&repo, &branch_name)?;

    Ok(())
}

/// Check if the working directory is clean (no uncommitted changes to tracked files)
/// Untracked files are allowed - we only care about modifications to existing files
fn ensure_clean_working_directory(repo: &Repository) -> Result<()> {
    let statuses = repo
        .statuses(None)
        .context("Failed to get repository status")?;

    let mut dirty_files = Vec::new();
    for entry in statuses.iter() {
        let status = entry.status();
        // Ignore untracked files (WT_NEW) - only fail on modifications to tracked files
        if status.is_wt_modified()
            || status.is_wt_deleted()
            || status.is_wt_renamed()
            || status.is_wt_typechange()
            || status.is_index_modified()
            || status.is_index_deleted()
            || status.is_index_renamed()
            || status.is_index_new()
        {
            if let Some(path) = entry.path() {
                dirty_files.push(path.to_string());
            }
        }
    }

    if !dirty_files.is_empty() {
        bail!(
            "Working directory is not clean. Please commit or stash your changes first.\nModified files:\n  {}",
            dirty_files.join("\n  ")
        );
    }

    Ok(())
}

/// Check if a branch exists
fn branch_exists(repo: &Repository, branch_name: &str) -> Result<bool> {
    match repo.find_branch(branch_name, BranchType::Local) {
        Ok(_) => Ok(true),
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(false),
        Err(e) => Err(e).context("Failed to check if branch exists"),
    }
}

/// Checkout a branch by name
fn checkout_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    // Find the branch
    let branch = repo
        .find_branch(branch_name, BranchType::Local)
        .with_context(|| format!("Failed to find branch '{branch_name}'"))?;

    // Get the reference
    let branch_ref = branch
        .get()
        .name()
        .context("Failed to get branch reference name")?;

    // Set HEAD to the branch
    repo.set_head(branch_ref)
        .with_context(|| format!("Failed to set HEAD to '{branch_name}'"))?;

    // Checkout the files
    repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))
        .context("Failed to checkout branch files")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Helper to create a test git repository with an initial commit
    fn create_test_repo() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize repo
        let repo = Repository::init(&repo_path).unwrap();

        // Create initial commit
        let signature = git2::Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();

            // Create a test file
            fs::write(repo_path.join("README.md"), "# Test Project\n").unwrap();
            index.add_path(Path::new("README.md")).unwrap();
            index.write().unwrap();
            index.write_tree().unwrap()
        };

        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )
        .unwrap();

        (temp_dir, repo_path)
    }

    #[test]
    fn test_create_sprint_branch_success() {
        let (_temp, repo_path) = create_test_repo();

        let result = create_sprint_branch(&repo_path, 4, "the-sprint-orchestrator");
        assert!(result.is_ok(), "Should create branch successfully");

        // Verify branch exists
        let repo = Repository::open(&repo_path).unwrap();
        let branch = repo.find_branch("feature/sprint-4-the-sprint-orchestrator", BranchType::Local);
        assert!(branch.is_ok(), "Branch should exist");

        // Verify we're on the new branch
        let head = repo.head().unwrap();
        assert!(head.is_branch(), "HEAD should be a branch");
        assert_eq!(
            head.shorthand(),
            Some("feature/sprint-4-the-sprint-orchestrator"),
            "Should be on new branch"
        );
    }

    #[test]
    fn test_create_sprint_branch_already_exists() {
        let (_temp, repo_path) = create_test_repo();

        // Create branch first time
        create_sprint_branch(&repo_path, 2, "test-branch").unwrap();

        // Try to create same branch again
        let result = create_sprint_branch(&repo_path, 2, "test-branch");
        assert!(result.is_err(), "Should fail when branch already exists");
        assert!(
            result.unwrap_err().to_string().contains("already exists"),
            "Error should mention branch already exists"
        );
    }

    #[test]
    fn test_create_sprint_branch_dirty_working_directory() {
        let (_temp, repo_path) = create_test_repo();

        // Modify a tracked file (not just add untracked)
        fs::write(repo_path.join("README.md"), "# Modified\n").unwrap();

        let result = create_sprint_branch(&repo_path, 3, "test-dirty");
        assert!(result.is_err(), "Should fail with dirty working directory");
        assert!(
            result.unwrap_err().to_string().contains("not clean"),
            "Error should mention dirty working directory"
        );
    }

    #[test]
    fn test_ensure_clean_working_directory() {
        let (_temp, repo_path) = create_test_repo();
        let repo = Repository::open(&repo_path).unwrap();

        // Clean repo should pass
        let result = ensure_clean_working_directory(&repo);
        assert!(result.is_ok(), "Clean repo should pass");

        // Add untracked file - should still pass (untracked files are allowed)
        fs::write(repo_path.join("untracked.txt"), "untracked").unwrap();
        let result = ensure_clean_working_directory(&repo);
        assert!(result.is_ok(), "Untracked files should be allowed");

        // Modify tracked file - should fail
        fs::write(repo_path.join("README.md"), "# Modified\n").unwrap();
        let result = ensure_clean_working_directory(&repo);
        assert!(result.is_err(), "Modified tracked files should fail");
    }

    #[test]
    fn test_branch_exists() {
        let (_temp, repo_path) = create_test_repo();
        let repo = Repository::open(&repo_path).unwrap();

        // Check non-existent branch
        assert!(
            !branch_exists(&repo, "nonexistent").unwrap(),
            "Non-existent branch should return false"
        );

        // Create a branch
        create_sprint_branch(&repo_path, 1, "test").unwrap();

        // Check existing branch
        assert!(
            branch_exists(&repo, "feature/sprint-1-test").unwrap(),
            "Existing branch should return true"
        );
    }
}
