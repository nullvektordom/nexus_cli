/// Document validation for generated planning documents

use anyhow::{Context, Result};
use std::path::Path;

use crate::catalyst::generator::DocumentType;
use crate::planning::validate_planning_document_with_headers;

/// Validate a generated document against its expected structure
pub fn validate_generated_document(
    doc_type: DocumentType,
    content: &str,
    path: &Path,
) -> Result<bool> {
    let (required_headers, min_word_count) = get_validation_requirements(doc_type);

    let illegal_strings: Vec<String> = vec![
        "TODO".to_string(),
        "TBD".to_string(),
        "[fill]".to_string(),
        "[describe]".to_string(),
        "[your".to_string(),
        "[add".to_string(),
    ];

    let validation = validate_planning_document_with_headers(
        path,
        &required_headers,
        min_word_count,
        &illegal_strings,
    )
    .context("Failed to validate generated document")?;

    if !validation.passed {
        // Log validation errors for debugging
        eprintln!("Validation failed for {:?}:", doc_type);
        for issue in &validation.issues {
            eprintln!("  - {:?}", issue);
        }
    }

    Ok(validation.passed)
}

/// Get validation requirements for each document type
fn get_validation_requirements(doc_type: DocumentType) -> (Vec<String>, usize) {
    match doc_type {
        DocumentType::Scope => (
            vec![
                "MVP (Minimum Viable Product):".to_string(),
                "Version 2 (NOT NOW - just document):".to_string(),
                "Never (things I will NOT build):".to_string(),
                "Tech constraints:".to_string(),
            ],
            50, // minimum word count per section
        ),
        DocumentType::TechStack => (
            vec![
                "Language:".to_string(),
                "Framework/Library:".to_string(),
                "Database (if needed):".to_string(),
                "Justification:".to_string(),
            ],
            30,
        ),
        DocumentType::Architecture => (
            vec![
                "Folder structure:".to_string(),
                "Data model (main entities):".to_string(),
                "Flow (user journey):".to_string(),
                "Critical technical decisions:".to_string(),
            ],
            30,
        ),
        DocumentType::MvpBreakdown => (
            vec![
                "Sprint 0:".to_string(),
            ],
            30,
        ),
    }
}

/// Check if a document contains placeholder text that should be replaced
pub fn contains_placeholders(content: &str) -> bool {
    let placeholders = [
        "TODO",
        "TBD",
        "[fill",
        "[describe",
        "[your",
        "[add",
        "...",
        "etc.",
    ];

    for placeholder in &placeholders {
        if content.contains(placeholder) {
            return true;
        }
    }

    false
}

/// Extract section content from markdown by header
pub fn extract_section(content: &str, header: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_section = false;
    let mut section_content = Vec::new();

    for line in lines {
        if line.contains(header) {
            in_section = true;
            continue;
        }

        if in_section {
            // Stop at next header (## or #)
            if line.starts_with("##") || line.starts_with("# ") {
                break;
            }
            section_content.push(line);
        }
    }

    if section_content.is_empty() {
        None
    } else {
        Some(section_content.join("\n").trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_placeholders() {
        assert!(contains_placeholders("This has a TODO item"));
        assert!(contains_placeholders("This is TBD"));
        assert!(contains_placeholders("[fill this in]"));
        assert!(!contains_placeholders("This is complete content"));
    }

    #[test]
    fn test_extract_section() {
        let content = r#"
## MVP (Minimum Viable Product):

This is the MVP content.
It has multiple lines.

## Version 2:

This is version 2 content.
"#;

        let mvp = extract_section(content, "MVP (Minimum Viable Product):");
        assert!(mvp.is_some());
        assert!(mvp.unwrap().contains("This is the MVP content"));

        let v2 = extract_section(content, "Version 2:");
        assert!(v2.is_some());
        assert!(v2.unwrap().contains("This is version 2 content"));
    }
}
