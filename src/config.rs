use serde::{Deserialize, Serialize};

/// Configuration structure for Nexus projects
/// Stores project metadata and is serialized to/from TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusConfig {
    /// Name of the project
    pub project_name: String,

    /// Path to the War Room (Obsidian vault project folder)
    pub obsidian_path: String,

    /// Path to the code repository (optional, set later during unlock)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_path: Option<String>,

    /// Timestamp when the project was initialized
    pub created_at: String,

    /// Current active sprint number (None if no sprint is active)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_sprint: Option<u32>,
}

impl NexusConfig {
    /// Create a new NexusConfig with the given project name and obsidian path
    pub fn new(project_name: String, obsidian_path: String) -> Self {
        let created_at = chrono::Utc::now().to_rfc3339();

        Self {
            project_name,
            obsidian_path,
            repo_path: None,
            created_at,
            current_sprint: None,
        }
    }

    /// Serialize the config to a TOML string
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Deserialize the config from a TOML string
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
}
