//! State Management - Session Persistence for Nexus Shell
//!
//! Manages the state of the Nexus shell session, including:
//! - Active project ID
//! - Obsidian vault root
//! - Session ID for cross-computer handover
//! - Persistence to disk

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Default Obsidian vault root directory
pub const DEFAULT_OBSIDIAN_ROOT: &str = "/home/nullvektor/obsidian/execution_helper/Projects";

/// Default repos root directory
pub const DEFAULT_REPOS_ROOT: &str = "/home/nullvektor/repos";

/// Represents the persistent state of a Nexus shell session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusState {
    /// Unique identifier for this session
    pub session_id: String,

    /// Currently active project ID (None if no project selected)
    pub active_project_id: Option<String>,

    /// Root directory for Obsidian vault projects
    pub obsidian_vault_root: PathBuf,

    /// Root directory for code repositories
    #[serde(default = "default_repos_root")]
    pub repos_root: PathBuf,

    /// Timestamp when the session was created (ISO 8601)
    pub created_at: String,

    /// Timestamp when the session was last updated (ISO 8601)
    pub last_updated: String,
}

fn default_repos_root() -> PathBuf {
    PathBuf::from(DEFAULT_REPOS_ROOT)
}

impl NexusState {
    /// Create a new session state without an active project
    pub fn new() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            session_id: Uuid::new_v4().to_string(),
            active_project_id: None,
            obsidian_vault_root: PathBuf::from(DEFAULT_OBSIDIAN_ROOT),
            repos_root: PathBuf::from(DEFAULT_REPOS_ROOT),
            created_at: now.clone(),
            last_updated: now,
        }
    }

    /// Load session state from disk
    /// Returns a new state if the file doesn't exist
    pub fn load(state_file_path: &Path) -> Result<Self> {
        if state_file_path.exists() {
            let content = fs::read_to_string(state_file_path).with_context(|| {
                format!("Failed to read state file: {}", state_file_path.display())
            })?;

            let mut state: NexusState = serde_json::from_str(&content).with_context(|| {
                format!("Failed to parse state file: {}", state_file_path.display())
            })?;

            // Update the last_updated timestamp
            state.last_updated = chrono::Utc::now().to_rfc3339();

            Ok(state)
        } else {
            Ok(Self::new())
        }
    }

    /// Save session state to disk
    pub fn save(&self, state_file_path: &Path) -> Result<()> {
        // Ensure the parent directory exists
        if let Some(parent) = state_file_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create state directory: {}", parent.display())
            })?;
        }

        let content = serde_json::to_string_pretty(self).context("Failed to serialize state")?;

        fs::write(state_file_path, content).with_context(|| {
            format!("Failed to write state file: {}", state_file_path.display())
        })?;

        Ok(())
    }

    /// Update the `last_updated` timestamp
    pub fn touch(&mut self) {
        self.last_updated = chrono::Utc::now().to_rfc3339();
    }

    /// Set the active project
    pub fn set_active_project(&mut self, project_id: String) {
        self.active_project_id = Some(project_id);
        self.touch();
    }

    /// Clear the active project
    #[allow(dead_code)]
    pub fn clear_active_project(&mut self) {
        self.active_project_id = None;
        self.touch();
    }

    /// Get the path to the active project's repository
    pub fn get_active_repo_path(&self) -> Option<PathBuf> {
        self.active_project_id
            .as_ref()
            .map(|id| self.repos_root.join(id))
    }

    /// Get the path to the active project in Obsidian vault
    pub fn get_active_obsidian_path(&self) -> Option<PathBuf> {
        self.active_project_id
            .as_ref()
            .map(|id| self.obsidian_vault_root.join(id))
    }

    /// Get the path to the active project's session file
    #[allow(dead_code)]
    pub fn get_project_session_path(&self) -> Option<PathBuf> {
        self.get_active_obsidian_path().map(|p| {
            p.join("00-MANAGEMENT")
                .join(".nexus_state")
                .join("session.json")
        })
    }

    /// Validate that a project exists in both repos and obsidian vault
    pub fn validate_project(&self, project_id: &str) -> Result<()> {
        let repo_path = self.repos_root.join(project_id);
        let obsidian_path = self.obsidian_vault_root.join(project_id);

        if !repo_path.exists() {
            anyhow::bail!(
                "Project repository not found: {}\nExpected at: {}",
                project_id,
                repo_path.display()
            );
        }

        if !obsidian_path.exists() {
            anyhow::bail!(
                "Project Obsidian vault not found: {}\nExpected at: {}",
                project_id,
                obsidian_path.display()
            );
        }

        Ok(())
    }
}

impl Default for NexusState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_new_state() {
        let state = NexusState::new();

        assert!(state.active_project_id.is_none());
        assert_eq!(
            state.obsidian_vault_root,
            PathBuf::from(DEFAULT_OBSIDIAN_ROOT)
        );
        assert_eq!(state.repos_root, PathBuf::from(DEFAULT_REPOS_ROOT));
        assert!(!state.session_id.is_empty());
        assert!(!state.created_at.is_empty());
        assert_eq!(state.created_at, state.last_updated);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let state_file = temp_dir.path().join("session.json");

        // Create and save a state
        let mut original_state = NexusState::new();
        original_state.set_active_project("test_project".to_string());
        original_state.save(&state_file).unwrap();

        // Load the state
        let loaded_state = NexusState::load(&state_file).unwrap();

        assert_eq!(loaded_state.session_id, original_state.session_id);
        assert_eq!(
            loaded_state.active_project_id,
            Some("test_project".to_string())
        );
        assert_eq!(loaded_state.created_at, original_state.created_at);
    }

    #[test]
    fn test_set_and_clear_active_project() {
        let mut state = NexusState::new();
        assert!(state.active_project_id.is_none());

        state.set_active_project("my_project".to_string());
        assert_eq!(state.active_project_id, Some("my_project".to_string()));

        state.clear_active_project();
        assert!(state.active_project_id.is_none());
    }

    #[test]
    fn test_get_active_paths() {
        let mut state = NexusState::new();
        state.set_active_project("nexus_cli".to_string());

        assert_eq!(
            state.get_active_repo_path(),
            Some(PathBuf::from("/home/nullvektor/repos/nexus_cli"))
        );

        assert_eq!(
            state.get_active_obsidian_path(),
            Some(PathBuf::from(
                "/home/nullvektor/obsidian/execution_helper/Projects/nexus_cli"
            ))
        );
    }
}
