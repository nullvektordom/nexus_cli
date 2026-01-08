//! Genesis Prompt Templates
//!
//! These prompts are specifically designed for PROJECT GENESIS.
//! They differ from Catalyst prompts in their architectural focus.

use crate::catalyst::generator::VisionData;
use crate::schema::planning::PLANNING_DOCUMENTS;

/// The internal Genesis prompt (dynamically generated from schema)
pub fn get_genesis_system_prompt() -> String {
    let mut prompt = String::from(r#"You are the Nexus Catalyst. You are performing a PROJECT GENESIS. I will provide a 'Problem & Vision' (Doc 01). You must synthesize the complete foundational skeleton.

DIFFERENTIATION RULE: Do not treat this as a bug fix or a feature addition. This is the creation of a new system.

REQUIRED OUTPUT (Separated by ---NEXT_DOC---):
"#);

    // Skip the first document (Vision) as it is the input
    let docs_to_generate: Vec<_> = PLANNING_DOCUMENTS.iter().skip(1).collect();

    for (i, doc) in docs_to_generate.iter().enumerate() {
        prompt.push_str(&format!("\n{}. {}: {}\n", i + 1, doc.filename, doc.template_guidance));
    }

    prompt.push_str(r#"
CRITICAL REQUIREMENTS:
- Output ONLY the documents separated by ---NEXT_DOC---
- Be specific and concrete - NO placeholders like TODO, TBD, [fill], [describe]
- Each document must have substantial content (minimum 100 words per section)
- Focus on architectural synthesis and system-level design
- Justify all technical decisions based on the Vision's constraints

DOCUMENT STRUCTURE:
"#);

    for (i, doc) in docs_to_generate.iter().enumerate() {
        prompt.push_str(&format!("\n{}:\n", doc.filename));
        for header in doc.required_headers {
            prompt.push_str(&format!("## {}\n[Content...]\n\n", header));
        }
        if i < docs_to_generate.len() - 1 {
            prompt.push_str("---NEXT_DOC---\n");
        }
    }

    prompt
}

/// Build the user prompt for Genesis
pub fn build_genesis_user_prompt(vision: &VisionData) -> String {
    let docs_count = PLANNING_DOCUMENTS.len() - 1;
    let last_doc_index = PLANNING_DOCUMENTS.len();

    format!(
        r#"Based on this vision, generate the complete foundational skeleton (Docs 02-{:02}):

# Vision Document (01-Problem-and-Vision.md)

## Problem Statement
{}

## Vision
{}

## Success Criteria (3 months)
{}

## Anti-Vision (what this is NOT)
{}

---

Generate all {} planning documents now, separated by ---NEXT_DOC---. Remember:
- This is PROJECT GENESIS, not a task or feature
- Focus on architectural synthesis
- Be specific and concrete
- No placeholders or TODOs
- Justify all technical decisions
- Use EXACTLY the headers specified in the document structure above"#,
        last_doc_index,
        vision.problem, vision.solution, vision.success_criteria, vision.anti_vision,
        docs_count
    )
}

/// Parse the LLM response into individual documents
///
/// # Arguments
/// * `response` - The raw LLM response containing all documents
///
/// # Returns
/// * A vector of (filename, content) tuples
pub fn parse_genesis_response(response: &str) -> Vec<(String, String)> {
    let documents: Vec<&str> = response.split("---NEXT_DOC---").collect();

    // Skip the first document (Vision)
    let expected_docs: Vec<_> = PLANNING_DOCUMENTS.iter().skip(1).collect();

    documents
        .iter()
        .enumerate()
        .filter_map(|(idx, content)| {
            if idx < expected_docs.len() {
                let mut clean_content = content.trim().to_string();

                // Remove filename header if present (e.g., "03-Tech-Stack.md:")
                // LLMs often include this when they see it in the prompt structure
                let filename = expected_docs[idx].filename;
                if clean_content.starts_with(filename) {
                    // Remove the filename line
                    if let Some(newline_pos) = clean_content.find('\n') {
                        clean_content = clean_content[newline_pos + 1..].trim().to_string();
                    }
                }

                if !clean_content.is_empty() {
                    Some((filename.to_string(), clean_content))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_genesis_user_prompt() {
        let vision = VisionData {
            problem: "Test problem".to_string(),
            solution: "Test solution".to_string(),
            success_criteria: "Test criteria".to_string(),
            anti_vision: "Test anti-vision".to_string(),
        };

        let prompt = build_genesis_user_prompt(&vision);
        assert!(prompt.contains("Test problem"));
        assert!(prompt.contains("PROJECT GENESIS"));
    }

    #[test]
    fn test_parse_genesis_response() {
        let response = r#"# Doc 1
Content 1
---NEXT_DOC---
# Doc 2
Content 2
---NEXT_DOC---
# Doc 3
Content 3
---NEXT_DOC---
# Doc 4
Content 4"#;

        let docs = parse_genesis_response(response);
        assert_eq!(docs.len(), 4);
        assert_eq!(docs[0].0, "02-Scope-and-Boundaries.md");
        assert_eq!(docs[1].0, "03-Tech-Stack.md");
        assert_eq!(docs[2].0, "04-Architecture.md");
        assert_eq!(docs[3].0, "05-MVP-Breakdown.md");
        assert!(docs[0].1.contains("Doc 1"));
    }
}
