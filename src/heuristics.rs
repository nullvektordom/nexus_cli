//! Heuristics Engine for Gate Command
//!
//! Parses and loads validation rules from Gate-Heuristics.json.
//! Defines the constraints used to validate planning documents.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

/// Management file configuration for dashboard validation
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ManagementFiles {
    /// Path to the dashboard file (e.g., "00-START-HERE.md")
    pub dashboard: String,
    /// Whether all checkboxes must be checked
    pub require_all_checked: bool,
}

/// Root heuristics configuration
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GateHeuristics {
    /// Minimum word count for each planning section
    pub min_section_length: u32,
    /// Headers that must be present in planning documents
    pub required_headers: Vec<String>,
    /// Strings that are forbidden (indicate incomplete planning)
    pub illegal_strings: Vec<String>,
    /// Management file validation rules
    pub management_files: ManagementFiles,
}

impl Default for GateHeuristics {
    /// Provides hardcoded fallback values matching Gate-Heuristics.json
    fn default() -> Self {
        Self {
            min_section_length: 50,
            required_headers: vec![
                "Problem".to_string(),
                "Vision".to_string(),
                "Scope".to_string(),
                "Boundaries".to_string(),
                "Tech Stack".to_string(),
                "Architecture".to_string(),
            ],
            illegal_strings: vec![
                "TODO".to_string(),
                "FIXME".to_string(),
                "TBD".to_string(),
                "...".to_string(),
                "insert here".to_string(),
                "fill me in".to_string(),
                "[ ]".to_string(),
            ],
            management_files: ManagementFiles {
                dashboard: "00-START-HERE.md".to_string(),
                require_all_checked: true,
            },
        }
    }
}

/// Loads heuristics from JSON file at the specified path
///
/// # Arguments
/// * `path` - Path to the Gate-Heuristics.json file
///
/// # Returns
/// * `Ok(GateHeuristics)` - Successfully parsed heuristics
/// * `Err` - File not found, invalid JSON, or deserialization error
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// use nexus_cli::heuristics::load_heuristics;
///
/// let heuristics = load_heuristics(Path::new("Gate-Heuristics.json"))?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn load_heuristics(path: &Path) -> Result<GateHeuristics> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open heuristics file: {}", path.display()))?;

    let heuristics: GateHeuristics = serde_json::from_reader(file)
        .with_context(|| format!("Failed to parse heuristics JSON: {}", path.display()))?;

    Ok(heuristics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_heuristics() {
        let heuristics = GateHeuristics::default();

        assert_eq!(heuristics.min_section_length, 50);
        assert_eq!(heuristics.required_headers.len(), 6);
        assert!(heuristics.required_headers.contains(&"Problem".to_string()));
        assert!(heuristics.required_headers.contains(&"Architecture".to_string()));
        assert_eq!(heuristics.illegal_strings.len(), 7);
        assert!(heuristics.illegal_strings.contains(&"TODO".to_string()));
        assert!(heuristics.illegal_strings.contains(&"[ ]".to_string()));
        assert_eq!(heuristics.management_files.dashboard, "00-START-HERE.md");
        assert!(heuristics.management_files.require_all_checked);
    }

    #[test]
    fn test_management_files_struct() {
        let mgmt = ManagementFiles {
            dashboard: "test.md".to_string(),
            require_all_checked: false,
        };

        assert_eq!(mgmt.dashboard, "test.md");
        assert!(!mgmt.require_all_checked);
    }

    #[test]
    fn test_load_heuristics_from_json() {
        let json_content = r#"{
  "min_section_length": 50,
  "required_headers": [
    "Problem",
    "Vision",
    "Scope",
    "Boundaries",
    "Tech Stack",
    "Architecture"
  ],
  "illegal_strings": [
    "TODO",
    "FIXME",
    "TBD",
    "...",
    "insert here",
    "fill me in",
    "[ ]"
  ],
  "management_files": {
    "dashboard": "00-START-HERE.md",
    "require_all_checked": true
  }
}"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(json_content.as_bytes())
            .expect("Failed to write to temp file");

        let heuristics = load_heuristics(temp_file.path()).expect("Failed to load heuristics");

        assert_eq!(heuristics.min_section_length, 50);
        assert_eq!(heuristics.required_headers.len(), 6);
        assert_eq!(heuristics.illegal_strings.len(), 7);
        assert_eq!(heuristics.management_files.dashboard, "00-START-HERE.md");
        assert!(heuristics.management_files.require_all_checked);
    }

    #[test]
    fn test_load_heuristics_invalid_json() {
        let invalid_json = r#"{ "invalid": json }"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(invalid_json.as_bytes())
            .expect("Failed to write to temp file");

        let result = load_heuristics(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_heuristics_missing_file() {
        let result = load_heuristics(Path::new("/nonexistent/file.json"));
        assert!(result.is_err());
    }
}
