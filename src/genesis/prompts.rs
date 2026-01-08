//! Genesis Prompt Templates
//!
//! These prompts are specifically designed for PROJECT GENESIS.
//! They differ from Catalyst prompts in their architectural focus.

use crate::catalyst::generator::VisionData;

/// The internal Genesis prompt (hardcoded as per spec)
pub const GENESIS_SYSTEM_PROMPT: &str = r#"You are the Nexus Catalyst. You are performing a PROJECT GENESIS. I will provide a 'Problem & Vision' (Doc 01). You must synthesize the complete foundational skeleton.

DIFFERENTIATION RULE: Do not treat this as a bug fix or a feature addition. This is the creation of a new system.

REQUIRED OUTPUT (Separated by ---NEXT_DOC---):

1. 02-Scope-and-Boundaries: Define the 'Fence'. Establish hard limits on what the system will NOT do to protect the MVP.

2. 03-Architecture-Logic: Map the system components, data schemas, and internal orchestration logic.

3. 04-Tech-Stack-Standard: Select specific Rust crates and justify them based on the Vision's constraints (e.g., why axum over actix, or sqlx over diesel).

4. 05-MVP-Roadmap: Build a 3-phase strategic path with clear 'Definitions of Done' for each milestone.

CRITICAL REQUIREMENTS:
- Output ONLY the four documents separated by ---NEXT_DOC---
- Be specific and concrete - NO placeholders like TODO, TBD, [fill], [describe]
- Each document must have substantial content (minimum 100 words per section)
- Focus on architectural synthesis and system-level design
- Justify all technical decisions based on the Vision's constraints

DOCUMENT STRUCTURE:

02-Scope-and-Boundaries.md:
## MVP (Minimum Viable Product):
[List of MVP features as bullet points]

## Version 2 (NOT NOW - just document):
[Future features to defer]

## Never (things I will NOT build):
[Features that would dilute focus]

## Tech constraints:
[Technical, time, or resource constraints]

---NEXT_DOC---

03-Architecture-Logic.md:
## System Architecture:
[High-level architecture description]

## Core Components:
[Key components and their responsibilities]

## Data Flow:
[How data moves through the system]

## Key Design Decisions:
[Important architectural decisions and rationale]

---NEXT_DOC---

04-Tech-Stack-Standard.md:
## Language:
[Primary programming language]

## Framework/Library:
[Framework or library]

## Database (if needed):
[Database choice or None]

## Justification:
[Justification for choices based on Vision constraints]

---NEXT_DOC---

05-MVP-Roadmap.md:
## Phase 1: Foundation
[Setup and core infrastructure tasks]

**Exit criteria:** [Measurable goal]

## Phase 2: Core Features
[Main feature implementation tasks]

**Exit criteria:** [Measurable goal]

## Phase 3: Polish & Release
[Refinement and release preparation tasks]

**Exit criteria:** [Measurable goal]"#;

/// Build the user prompt for Genesis
pub fn build_genesis_user_prompt(vision: &VisionData) -> String {
    format!(
        r#"Based on this vision, generate the complete foundational skeleton (Docs 02-05):

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

Generate all four planning documents now, separated by ---NEXT_DOC---. Remember:
- This is PROJECT GENESIS, not a task or feature
- Focus on architectural synthesis
- Be specific and concrete
- No placeholders or TODOs
- Justify all technical decisions"#,
        vision.problem, vision.solution, vision.success_criteria, vision.anti_vision
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
    let documents = response.split("---NEXT_DOC---");

    let filenames = vec![
        "02-Scope-and-Boundaries.md",
        "03-Architecture-Logic.md",
        "04-Tech-Stack-Standard.md",
        "05-MVP-Roadmap.md",
    ];

    documents
        .enumerate()
        .filter_map(|(idx, doc)| {
            if idx < filenames.len() {
                let content = doc.trim().to_string();
                if !content.is_empty() {
                    Some((filenames[idx].to_string(), content))
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
        assert!(docs[0].1.contains("Doc 1"));
    }
}
