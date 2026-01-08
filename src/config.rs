use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Forward declarations of config structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasksConfig {
    /// Task mode: "sprint" (default) or "adhoc"
    #[serde(default = "default_task_mode")]
    pub mode: String,
    /// Path to adhoc planning directory (relative to management_dir)
    #[serde(default = "default_adhoc_planning_dir")]
    pub adhoc_planning_dir: String,
    /// Path to adhoc dashboard (relative to management_dir)
    #[serde(default = "default_adhoc_dashboard")]
    pub adhoc_dashboard: String,
}

fn default_task_mode() -> String {
    "sprint".to_string()
}

fn default_adhoc_planning_dir() -> String {
    "adhoc-planning".to_string()
}

fn default_adhoc_dashboard() -> String {
    "00-ADHOC-TASK.md".to_string()
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brain: Option<BrainConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<LlmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalyst: Option<CatalystConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<TasksConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    /// Path to the Obsidian vault/project root
    pub obsidian_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_field_names)] // _dir suffix is descriptive in this domain
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
    /// If not set, falls back to `obsidian_path` from [project]
    pub planning_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_sprint: Option<ActiveSprintConfig>,
    pub is_unlocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSprintConfig {
    /// Current sprint identifier (e.g., "sprint-4")
    pub current: String,
    /// Sprint status: "`in_progress`" or "approved"
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesConfig {
    pub claude_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainConfig {
    /// Qdrant gRPC URL (e.g., "<http://100.64.0.1:6334>" for Tailscale)
    pub qdrant_url: String,
    /// Whether the brain is enabled
    #[serde(default = "default_brain_enabled")]
    pub enabled: bool,
}

fn default_brain_enabled() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// LLM provider: "openrouter" (default), "claude", or "gemini"
    #[serde(default = "default_provider")]
    pub provider: String,
    /// API key for the LLM provider (set via environment variable recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Model to use (e.g., "deepseek/deepseek-r1", "claude-3-5-sonnet-20241022", "gemini-1.5-pro")
    #[serde(default = "default_model")]
    pub model: String,
    /// Whether LLM integration is enabled
    #[serde(default = "default_llm_enabled")]
    pub enabled: bool,
}

fn default_provider() -> String {
    "gemini".to_string()
}

fn default_model() -> String {
    "gemini-3-pro".to_string()
}

fn default_llm_enabled() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalystConfig {
    /// Whether catalyst document generation is enabled
    #[serde(default = "default_catalyst_enabled")]
    pub enabled: bool,
    /// Whether to show reasoning process from models like `DeepSeek` R1
    #[serde(default)]
    pub show_reasoning: bool,
    /// Maximum number of retry attempts if validation fails
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
}

fn default_catalyst_enabled() -> bool {
    true
}

fn default_max_retries() -> u32 {
    3
}

impl NexusConfig {
    /// Create a new `NexusConfig` with the given project name and obsidian path
    #[allow(clippy::needless_pass_by_value)] // Builder pattern, obsidian_path is cloned
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
            brain: None,    // Brain is disabled by default, configure in nexus.toml
            llm: None,      // LLM is disabled by default, configure in nexus.toml
            catalyst: None, // Catalyst uses defaults if not configured
            tasks: None,    // Tasks defaults to sprint mode if not configured
        }
    }

    /// Get the planning path, falling back to `obsidian_path` if not set
    pub fn get_planning_path(&self) -> PathBuf {
        self.obsidian
            .as_ref().map_or_else(|| PathBuf::from(&self.project.obsidian_path), |o| o.planning_path.clone())
    }

    /// Get the vault/repo path (`obsidian_path`)
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

    /// Check if the project is in adhoc task mode
    pub fn is_adhoc_mode(&self) -> bool {
        self.tasks
            .as_ref()
            .is_some_and(|t| t.mode == "adhoc")
    }

    /// Get the full path to the adhoc planning directory
    pub fn get_adhoc_planning_path(&self) -> PathBuf {
        let planning_dir = self.tasks
            .as_ref()
            .map_or_else(|| "adhoc-planning".to_string(), |t| t.adhoc_planning_dir.clone());

        self.get_repo_path()
            .join(&self.structure.management_dir)
            .join(planning_dir)
    }

    /// Get the full path to the adhoc dashboard
    pub fn get_adhoc_dashboard_path(&self) -> PathBuf {
        let dashboard = self.tasks
            .as_ref()
            .map_or_else(|| "00-ADHOC-TASK.md".to_string(), |t| t.adhoc_dashboard.clone());

        self.get_repo_path()
            .join(&self.structure.management_dir)
            .join(dashboard)
    }

    /// Get the stable heuristics file path (.nexus/gate-heuristics.json in project root)
    /// This is the new standard location that prevents "Moment 22" deadlocks
    pub fn get_stable_heuristics_path(&self) -> PathBuf {
        self.get_repo_path().join(".nexus/gate-heuristics.json")
    }

    /// Get the legacy heuristics file path (from config)
    /// Used for backward compatibility
    #[allow(dead_code)] // Reserved for future migration tooling
    pub fn get_legacy_heuristics_path(&self) -> PathBuf {
        self.get_repo_path().join(&self.gate.heuristics_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tasks_config_defaults() {
        let toml_without_tasks = r#"
[project]
name = "test"
version = "0.1.0"
obsidian_path = "/test"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true
        "#;

        let config = NexusConfig::from_toml(toml_without_tasks).unwrap();
        assert!(!config.is_adhoc_mode());
    }

    #[test]
    fn test_tasks_config_sprint_mode() {
        let toml_sprint_mode = r#"
[project]
name = "test"
version = "0.1.0"
obsidian_path = "/test"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[tasks]
mode = "sprint"
        "#;

        let config = NexusConfig::from_toml(toml_sprint_mode).unwrap();
        assert!(!config.is_adhoc_mode());
    }

    #[test]
    fn test_tasks_config_adhoc_mode() {
        let toml_adhoc_mode = r#"
[project]
name = "test"
version = "0.1.0"
obsidian_path = "/test"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[tasks]
mode = "adhoc"
        "#;

        let config = NexusConfig::from_toml(toml_adhoc_mode).unwrap();
        assert!(config.is_adhoc_mode());
    }

    #[test]
    fn test_adhoc_path_resolution() {
        let toml_adhoc = r#"
[project]
name = "test"
version = "0.1.0"
obsidian_path = "/test/vault"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[tasks]
mode = "adhoc"
adhoc_planning_dir = "my-adhoc-planning"
adhoc_dashboard = "MY-TASK.md"
        "#;

        let config = NexusConfig::from_toml(toml_adhoc).unwrap();

        let planning_path = config.get_adhoc_planning_path();
        assert_eq!(
            planning_path,
            PathBuf::from("/test/vault/00-MANAGEMENT/my-adhoc-planning")
        );

        let dashboard_path = config.get_adhoc_dashboard_path();
        assert_eq!(
            dashboard_path,
            PathBuf::from("/test/vault/00-MANAGEMENT/MY-TASK.md")
        );
    }

    #[test]
    fn test_adhoc_path_defaults() {
        let toml_adhoc = r#"
[project]
name = "test"
version = "0.1.0"
obsidian_path = "/test"

[structure]
planning_dir = "01-PLANNING"
management_dir = "00-MANAGEMENT"
sprint_dir = "00-MANAGEMENT/Sprints"

[gate]
heuristics_file = "Gate-Heuristics.json"
strict_mode = true

[tasks]
mode = "adhoc"
        "#;

        let config = NexusConfig::from_toml(toml_adhoc).unwrap();

        let planning_path = config.get_adhoc_planning_path();
        assert_eq!(
            planning_path,
            PathBuf::from("/test/00-MANAGEMENT/adhoc-planning")
        );

        let dashboard_path = config.get_adhoc_dashboard_path();
        assert_eq!(
            dashboard_path,
            PathBuf::from("/test/00-MANAGEMENT/00-ADHOC-TASK.md")
        );
    }
}

