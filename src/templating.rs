//! Templating Module for CLAUDE.md Generation
//!
//! Uses Tera to render CLAUDE.md from extracted planning context.

#![allow(clippy::similar_names)] // context/content are domain-appropriate names

use crate::planning::PlanningContext;
use anyhow::{Context, Result};
use std::path::Path;
use tera::Tera;

/// Generate CLAUDE.md from planning context using Tera template
///
/// # Arguments
/// * `context` - Extracted planning data to populate template
/// * `project_root` - Root directory where CLAUDE.md will be written
/// * `template_path` - Optional path to custom template (defaults to embedded template)
///
/// # Returns
/// * `Ok(())` - CLAUDE.md successfully generated
/// * `Err` - Template rendering or file write failed
pub fn generate_claude_md(
    context: &PlanningContext,
    project_root: &Path,
    template_path: Option<&Path>,
) -> Result<()> {
    // Load template
    let template_content = if let Some(path) = template_path {
        std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read template from: {}", path.display()))?
    } else {
        // Use default embedded template
        include_str!("../templates/claude_template.tera").to_string()
    };

    // Create Tera instance and add template
    let mut tera = Tera::default();
    tera.add_raw_template("claude", &template_content)
        .context("Failed to parse Tera template")?;

    // Create context for Tera
    let mut tera_context = tera::Context::new();
    tera_context.insert("project_name", &context.project_name);
    tera_context.insert("problem_statement", &context.problem_statement);
    tera_context.insert("vision", &context.vision);
    tera_context.insert("problem_details", &context.problem_details);
    tera_context.insert("mvp_scope", &context.mvp_scope);
    tera_context.insert("anti_scope", &context.anti_scope);
    tera_context.insert("tech_constraints", &context.tech_constraints);
    tera_context.insert("tech_stack", &context.tech_stack);
    tera_context.insert("stack_justification", &context.stack_justification);
    tera_context.insert("tech_exclusions", &context.tech_exclusions);
    tera_context.insert("dependencies", &context.dependencies);
    tera_context.insert("folder_structure", &context.folder_structure);
    tera_context.insert("data_model", &context.data_model);
    tera_context.insert("user_flow", &context.user_flow);
    tera_context.insert("technical_decisions", &context.technical_decisions);
    tera_context.insert("mvp_breakdown", &context.mvp_breakdown);
    tera_context.insert("generation_date", &context.generation_date);

    // Render template
    let rendered = tera
        .render("claude", &tera_context)
        .context("Failed to render Tera template")?;

    // Write to CLAUDE.md
    let claude_path = project_root.join("CLAUDE.md");
    std::fs::write(&claude_path, rendered)
        .with_context(|| format!("Failed to write CLAUDE.md to: {}", claude_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_claude_md() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();

        let mut context = PlanningContext::new("TestProject".to_string());
        context.problem_statement = "Solve a test problem".to_string();
        context.vision = "Build something great".to_string();
        context.tech_stack = "Rust, Tera".to_string();

        let result = generate_claude_md(&context, project_root, None);
        assert!(result.is_ok(), "Failed to generate CLAUDE.md");

        let claude_path = project_root.join("CLAUDE.md");
        assert!(claude_path.exists(), "CLAUDE.md was not created");

        let content = std::fs::read_to_string(&claude_path).unwrap();
        assert!(
            content.contains("TestProject"),
            "Content missing project name"
        );
        assert!(
            content.contains("Solve a test problem"),
            "Content missing problem statement"
        );
    }

    #[test]
    fn test_generate_with_custom_template() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();

        // Create custom template
        let template_content = "# {{ project_name }}\n\nProblem: {{ problem_statement }}";
        let template_path = temp_dir.path().join("custom.tera");
        std::fs::write(&template_path, template_content).unwrap();

        let context = PlanningContext::new("CustomProject".to_string());

        let result = generate_claude_md(&context, project_root, Some(&template_path));
        assert!(result.is_ok(), "Failed with custom template");

        let claude_path = project_root.join("CLAUDE.md");
        let content = std::fs::read_to_string(&claude_path).unwrap();
        assert!(content.contains("# CustomProject"));
    }
}
