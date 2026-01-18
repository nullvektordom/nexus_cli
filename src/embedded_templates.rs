//! Embedded templates for project initialization
//!
//! This module contains all template files embedded at compile time,
//! making them available regardless of where the binary is run from.

use std::path::Path;
use std::fs;
use std::io;

// Project templates
const TEMPLATE_00_START_HERE: &str = include_str!("../templates/project/00-START-HERE.md");
const TEMPLATE_01_PROBLEM_VISION: &str = include_str!("../templates/project/01-Problem-and-Vision.md");
const TEMPLATE_02_SCOPE_BOUNDARIES: &str = include_str!("../templates/project/02-Scope-and-Boundaries.md");
const TEMPLATE_03_TECH_STACK: &str = include_str!("../templates/project/03-Tech-Stack.md");
const TEMPLATE_04_ARCHITECTURE: &str = include_str!("../templates/project/04-Architecture.md");
const TEMPLATE_05_MVP_BREAKDOWN: &str = include_str!("../templates/project/05-MVP-Breakdown.md");
const TEMPLATE_06_PROJECT_UNLOCKED: &str = include_str!("../templates/project/06-PROJECT-UNLOCKED.md");

// Adhoc templates
const ADHOC_TASK_CAPTURE: &str = include_str!("../templates/adhoc/Task-Capture.md");
const ADHOC_TASK_APPROACH: &str = include_str!("../templates/adhoc/Task-Approach.md");
const ADHOC_TASK_VALIDATION: &str = include_str!("../templates/adhoc/Task-Validation.md");
const ADHOC_DASHBOARD: &str = include_str!("../templates/adhoc/00-ADHOC-TASK.md");

/// Template file definition
pub struct TemplateFile {
    pub relative_path: &'static str,
    pub content: &'static str,
}

/// Get all project template files
pub fn get_project_templates() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            relative_path: "00-MANAGEMENT/00-START-HERE.md",
            content: TEMPLATE_00_START_HERE,
        },
        TemplateFile {
            relative_path: "01-PLANNING/01-Problem-and-Vision.md",
            content: TEMPLATE_01_PROBLEM_VISION,
        },
        TemplateFile {
            relative_path: "01-PLANNING/02-Scope-and-Boundaries.md",
            content: TEMPLATE_02_SCOPE_BOUNDARIES,
        },
        TemplateFile {
            relative_path: "01-PLANNING/03-Tech-Stack.md",
            content: TEMPLATE_03_TECH_STACK,
        },
        TemplateFile {
            relative_path: "01-PLANNING/04-Architecture.md",
            content: TEMPLATE_04_ARCHITECTURE,
        },
        TemplateFile {
            relative_path: "01-PLANNING/05-MVP-Breakdown.md",
            content: TEMPLATE_05_MVP_BREAKDOWN,
        },
        TemplateFile {
            relative_path: "00-MANAGEMENT/06-PROJECT-UNLOCKED.md",
            content: TEMPLATE_06_PROJECT_UNLOCKED,
        },
    ]
}

/// Get all adhoc template files
pub fn get_adhoc_templates() -> Vec<TemplateFile> {
    vec![
        TemplateFile {
            relative_path: "adhoc-planning/Task-Capture.md",
            content: ADHOC_TASK_CAPTURE,
        },
        TemplateFile {
            relative_path: "adhoc-planning/Task-Approach.md",
            content: ADHOC_TASK_APPROACH,
        },
        TemplateFile {
            relative_path: "adhoc-planning/Task-Validation.md",
            content: ADHOC_TASK_VALIDATION,
        },
        TemplateFile {
            relative_path: "00-ADHOC-TASK.md",
            content: ADHOC_DASHBOARD,
        },
    ]
}

/// Write embedded templates to a destination directory
pub fn write_templates(templates: &[TemplateFile], dest_dir: &Path) -> io::Result<()> {
    for template in templates {
        let dest_path = dest_dir.join(template.relative_path);

        // Create parent directories if they don't exist
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the template content
        fs::write(&dest_path, template.content)?;
    }

    Ok(())
}
