use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration structure for Nexus projects
/// Stores project metadata and is serialized to/from TOML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusConfig {
    pub project: ProjectConfig,
    pub structure: StructureConfig,
    pub gate: GateConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obsidian: Option<ObsidianConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<StateConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<TemplatesConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    /// Path to the Obsidian vault/project root
    pub obsidian_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureConfig {
    pub planning_dir: String,
    pub management_dir: String,
    pub sprint_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateConfig {
    pub heuristics_file: String,
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObsidianConfig {
    /// Path to the directory containing planning documents (01-PLANNING/)
    /// If not set, falls back to obsidian_path from [project]
    pub planning_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_sprint: Option<String>,
    pub is_unlocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesConfig {
    pub claude_template: String,
}

impl NexusConfig {
    /// Create a new NexusConfig with the given project name and obsidian path
    pub fn new(project_name: String, obsidian_path: String) -> Self {
        Self {
            project: ProjectConfig {
                name: project_name,
                version: "0.1.0".to_string(),
                obsidian_path: obsidian_path.clone(),
            },
            structure: StructureConfig {
                planning_dir: "01-PLANNING".to_string(),
                management_dir: "00-MANAGEMENT".to_string(),
                sprint_dir: "00-MANAGEMENT/Sprints".to_string(),
            },
            gate: GateConfig {
                heuristics_file: "Gate-Heuristics.json".to_string(),
                strict_mode: true,
            },
            // Default planning_path to obsidian_path
            obsidian: Some(ObsidianConfig {
                planning_path: PathBuf::from(&obsidian_path),
            }),
            state: Some(StateConfig {
                active_sprint: None,
                is_unlocked: false,
            }),
            templates: Some(TemplatesConfig {
                claude_template: "templates/CLAUDE.md.example".to_string(),
            }),
        }
    }

    /// Get the planning path, falling back to obsidian_path if not set
    pub fn get_planning_path(&self) -> PathBuf {
        self.obsidian
            .as_ref()
            .map(|o| o.planning_path.clone())
            .unwrap_or_else(|| PathBuf::from(&self.project.obsidian_path))
    }

    /// Get the vault/repo path (obsidian_path)
    pub fn get_repo_path(&self) -> PathBuf {
        PathBuf::from(&self.project.obsidian_path)
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
