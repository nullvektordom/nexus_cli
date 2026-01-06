/// Core catalyst engine for document generation

use anyhow::{Context, Result};
use colored::Colorize;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::catalyst::generator::{DocumentType, GenerationContext, VisionData};
use crate::catalyst::prompts::PromptTemplate;
use crate::catalyst::validation::validate_generated_document;
use crate::llm::LlmClient;

/// Report of document generation results
#[derive(Debug, Clone)]
pub struct GenerationReport {
    pub successes: Vec<DocumentType>,
    pub failures: Vec<(DocumentType, String)>,
}

impl GenerationReport {
    pub fn new() -> Self {
        Self {
            successes: Vec::new(),
            failures: Vec::new(),
        }
    }

    pub fn mark_success(&mut self, doc_type: DocumentType) {
        self.successes.push(doc_type);
    }

    pub fn mark_failure(&mut self, doc_type: DocumentType, error: String) {
        self.failures.push((doc_type, error));
    }

    pub fn is_complete_success(&self) -> bool {
        self.failures.is_empty() && self.successes.len() == 4
    }

    pub fn print_summary(&self) {
        println!();
        println!("{}", "═══════════════════════════════════════".cyan());
        println!("{}", "  Generation Report".cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();

        if !self.successes.is_empty() {
            println!("{}", "✓ Successfully Generated:".green().bold());
            for doc in &self.successes {
                println!("  • {}", doc.filename().green());
            }
            println!();
        }

        if !self.failures.is_empty() {
            println!("{}", "✗ Failed:".red().bold());
            for (doc, error) in &self.failures {
                println!("  • {}: {}", doc.filename().red(), error.dimmed());
            }
            println!();
        }

        if self.is_complete_success() {
            println!("{}", "🎉 All documents generated successfully!".green().bold());
        } else {
            println!(
                "{}",
                format!(
                    "⚠ Generated {}/{} documents",
                    self.successes.len(),
                    self.successes.len() + self.failures.len()
                )
                .yellow()
            );
        }
        println!();
    }
}

/// Main engine for generating planning documents
pub struct CatalystEngine {
    /// Project identifier
    project_id: String,
    /// Path to Obsidian vault
    obsidian_path: PathBuf,
    /// LLM client for generation
    llm_client: LlmClient,
}

impl CatalystEngine {
    /// Create a new catalyst engine
    pub fn new(project_id: String, obsidian_path: PathBuf, llm_client: LlmClient) -> Result<Self> {
        Ok(Self {
            project_id,
            obsidian_path,
            llm_client,
        })
    }

    /// Generate the scope document (02-Scope-and-Boundaries.md)
    pub async fn generate_scope(&self) -> Result<()> {
        println!("{}", "🔮 Generating Scope and Boundaries document...".cyan());

        // Load vision document
        let vision = self.load_vision_document()?;

        if !vision.is_complete() {
            anyhow::bail!(
                "Vision document is incomplete. Please fill out all sections in 01-Problem-and-Vision.md"
            );
        }

        // Build generation context
        let context = GenerationContext::new(vision);

        // Generate document
        let content = self
            .generate_document(DocumentType::Scope, &context)
            .await?;

        // Validate
        let output_path = self
            .obsidian_path
            .join(DocumentType::Scope.filename());
        
        println!("{}", "✓ Document generated, validating...".green());
        
        // Save to temporary location for validation
        fs::write(&output_path, &content)
            .context("Failed to write scope document")?;

        let is_valid = validate_generated_document(
            DocumentType::Scope,
            &content,
            &output_path,
        )?;

        if !is_valid {
            println!(
                "{}",
                "⚠ Generated document failed validation. Saving as draft...".yellow()
            );
            let draft_path = self
                .obsidian_path
                .join("02-Scope-and-Boundaries.draft.md");
            fs::write(&draft_path, &content)
                .context("Failed to write draft")?;
            anyhow::bail!(
                "Generated document failed validation. Review the draft at: {}",
                draft_path.display()
            );
        }

        println!(
            "{}",
            format!(
                "✓ Scope document generated successfully: {}",
                output_path.display()
            )
            .green()
        );

        Ok(())
    }

    /// Generate the tech stack document (03-Tech-Stack.md)
    pub async fn generate_tech_stack(&self) -> Result<()> {
        println!("{}", "🔮 Generating Tech Stack document...".cyan());

        // Load vision and scope
        let vision = self.load_vision_document()?;
        let scope = self.load_scope_document()?;

        let context = GenerationContext::new(vision).with_scope(scope);

        // Generate document
        let content = self
            .generate_document(DocumentType::TechStack, &context)
            .await?;

        // Save and validate
        let output_path = self
            .obsidian_path
            .join(DocumentType::TechStack.filename());

        println!("{}", "✓ Document generated, validating...".green());

        fs::write(&output_path, &content)
            .context("Failed to write tech stack document")?;

        let is_valid = validate_generated_document(
            DocumentType::TechStack,
            &content,
            &output_path,
        )?;

        if !is_valid {
            println!(
                "{}",
                "⚠ Generated document failed validation. Saving as draft...".yellow()
            );
            let draft_path = self
                .obsidian_path
                .join("03-Tech-Stack.draft.md");
            fs::write(&draft_path, &content)
                .context("Failed to write draft")?;
            anyhow::bail!(
                "Generated document failed validation. Review the draft at: {}",
                draft_path.display()
            );
        }

        println!(
            "{}",
            format!(
                "✓ Tech Stack document generated successfully: {}",
                output_path.display()
            )
            .green()
        );

        Ok(())
    }

    /// Generate the architecture document (04-Architecture.md)
    pub async fn generate_architecture(&self) -> Result<()> {
        println!("{}", "🔮 Generating Architecture document...".cyan());

        // Load vision, scope, and tech stack
        let vision = self.load_vision_document()?;
        let scope = self.load_scope_document()?;
        let tech_stack = self.load_tech_stack_document()?;

        let context = GenerationContext::new(vision)
            .with_scope(scope)
            .with_tech_stack(tech_stack);

        // Generate document
        let content = self
            .generate_document(DocumentType::Architecture, &context)
            .await?;

        // Save and validate
        let output_path = self
            .obsidian_path
            .join(DocumentType::Architecture.filename());

        println!("{}", "✓ Document generated, validating...".green());

        fs::write(&output_path, &content)
            .context("Failed to write architecture document")?;

        let is_valid = validate_generated_document(
            DocumentType::Architecture,
            &content,
            &output_path,
        )?;

        if !is_valid {
            println!(
                "{}",
                "⚠ Generated document failed validation. Saving as draft...".yellow()
            );
            let draft_path = self
                .obsidian_path
                .join("04-Architecture.draft.md");
            fs::write(&draft_path, &content)
                .context("Failed to write draft")?;
            anyhow::bail!(
                "Generated document failed validation. Review the draft at: {}",
                draft_path.display()
            );
        }

        println!(
            "{}",
            format!(
                "✓ Architecture document generated successfully: {}",
                output_path.display()
            )
            .green()
        );

        Ok(())
    }

    /// Generate the MVP breakdown document (05-MVP-Breakdown.md)
    pub async fn generate_mvp_breakdown(&self) -> Result<()> {
        println!("{}", "🔮 Generating MVP Breakdown document...".cyan());

        // Load all previous documents
        let vision = self.load_vision_document()?;
        let scope = self.load_scope_document()?;
        let tech_stack = self.load_tech_stack_document()?;
        let architecture = self.load_architecture_document()?;

        let context = GenerationContext::new(vision)
            .with_scope(scope)
            .with_tech_stack(tech_stack)
            .with_architecture(architecture);

        // Generate document
        let content = self
            .generate_document(DocumentType::MvpBreakdown, &context)
            .await?;

        // Save and validate
        let output_path = self
            .obsidian_path
            .join(DocumentType::MvpBreakdown.filename());

        println!("{}", "✓ Document generated, validating...".green());

        fs::write(&output_path, &content)
            .context("Failed to write MVP breakdown document")?;

        let is_valid = validate_generated_document(
            DocumentType::MvpBreakdown,
            &content,
            &output_path,
        )?;

        if !is_valid {
            println!(
                "{}",
                "⚠ Generated document failed validation. Saving as draft...".yellow()
            );
            let draft_path = self
                .obsidian_path
                .join("05-MVP-Breakdown.draft.md");
            fs::write(&draft_path, &content)
                .context("Failed to write draft")?;
            anyhow::bail!(
                "Generated document failed validation. Review the draft at: {}",
                draft_path.display()
            );
        }

        println!(
            "{}",
            format!(
                "✓ MVP Breakdown document generated successfully: {}",
                output_path.display()
            )
            .green()
        );

        Ok(())
    }

    /// Generate all planning documents sequentially (02-05)
    pub async fn generate_all(&self) -> Result<GenerationReport> {
        println!("{}", "🚀 Starting sequential document generation...".cyan().bold());
        println!();

        let mut report = GenerationReport::new();

        // Step 1: Generate scope
        println!("{}", "Step 1/4: Scope and Boundaries".bold());
        match self.generate_scope().await {
            Ok(_) => {
                report.mark_success(DocumentType::Scope);
                println!();
            }
            Err(e) => {
                report.mark_failure(DocumentType::Scope, e.to_string());
                println!("{}", format!("✗ Failed: {}", e).red());
                return Ok(report);
            }
        }

        // Step 2: Generate tech stack
        println!("{}", "Step 2/4: Tech Stack".bold());
        match self.generate_tech_stack().await {
            Ok(_) => {
                report.mark_success(DocumentType::TechStack);
                println!();
            }
            Err(e) => {
                report.mark_failure(DocumentType::TechStack, e.to_string());
                println!("{}", format!("✗ Failed: {}", e).red());
                return Ok(report);
            }
        }

        // Step 3: Generate architecture
        println!("{}", "Step 3/4: Architecture".bold());
        match self.generate_architecture().await {
            Ok(_) => {
                report.mark_success(DocumentType::Architecture);
                println!();
            }
            Err(e) => {
                report.mark_failure(DocumentType::Architecture, e.to_string());
                println!("{}", format!("✗ Failed: {}", e).red());
                return Ok(report);
            }
        }

        // Step 4: Generate MVP breakdown
        println!("{}", "Step 4/4: MVP Breakdown".bold());
        match self.generate_mvp_breakdown().await {
            Ok(_) => {
                report.mark_success(DocumentType::MvpBreakdown);
                println!();
            }
            Err(e) => {
                report.mark_failure(DocumentType::MvpBreakdown, e.to_string());
                println!("{}", format!("✗ Failed: {}", e).red());
                return Ok(report);
            }
        }

        Ok(report)
    }

    /// Load and parse the vision document
    fn load_vision_document(&self) -> Result<VisionData> {
        let vision_path = self.obsidian_path.join("01-Problem-and-Vision.md");

        if !vision_path.exists() {
            anyhow::bail!(
                "Vision document not found: {}. Please create it first.",
                vision_path.display()
            );
        }

        let content = fs::read_to_string(&vision_path)
            .context("Failed to read vision document")?;

        parse_vision_document(&content)
    }

    /// Load and parse the scope document
    fn load_scope_document(&self) -> Result<crate::catalyst::generator::ScopeData> {
        let scope_path = self.obsidian_path.join("02-Scope-and-Boundaries.md");

        if !scope_path.exists() {
            anyhow::bail!(
                "Scope document not found. Generate it first with 'catalyst scope'"
            );
        }

        let content = fs::read_to_string(&scope_path)
            .context("Failed to read scope document")?;

        parse_scope_document(&content)
    }

    /// Load and parse the tech stack document
    fn load_tech_stack_document(&self) -> Result<crate::catalyst::generator::TechStackData> {
        let tech_path = self.obsidian_path.join("03-Tech-Stack.md");

        if !tech_path.exists() {
            anyhow::bail!(
                "Tech Stack document not found. Generate it first with 'catalyst stack'"
            );
        }

        let content = fs::read_to_string(&tech_path)
            .context("Failed to read tech stack document")?;

        parse_tech_stack_document(&content)
    }

    /// Load and parse the architecture document
    fn load_architecture_document(&self) -> Result<crate::catalyst::generator::ArchitectureData> {
        let arch_path = self.obsidian_path.join("04-Architecture.md");

        if !arch_path.exists() {
            anyhow::bail!(
                "Architecture document not found. Generate it first with 'catalyst arch'"
            );
        }

        let content = fs::read_to_string(&arch_path)
            .context("Failed to read architecture document")?;

        parse_architecture_document(&content)
    }

    /// Generate a document using the LLM
    async fn generate_document(
        &self,
        doc_type: DocumentType,
        context: &GenerationContext,
    ) -> Result<String> {
        let prompt = match doc_type {
            DocumentType::Scope => PromptTemplate::for_scope(context),
            DocumentType::TechStack => PromptTemplate::for_tech_stack(context),
            DocumentType::Architecture => PromptTemplate::for_architecture(context),
            DocumentType::MvpBreakdown => PromptTemplate::for_mvp_breakdown(context),
        };

        println!("{}", "  Calling LLM...".dimmed());

        // Call LLM with system and user prompts
        let response = self
            .llm_client
            .complete_with_system(prompt.system_prompt(), prompt.user_prompt())
            .await
            .context("Failed to generate document via LLM")?;

        // Clean up response (remove any markdown code fences if present)
        let cleaned = clean_llm_response(&response);

        Ok(cleaned)
    }
}

/// Parse vision document into structured data
fn parse_vision_document(content: &str) -> Result<VisionData> {
    let sections = extract_sections(content);

    Ok(VisionData {
        problem: sections
            .get("My problem (personal):")
            .cloned()
            .unwrap_or_default(),
        solution: sections
            .get("Solution in ONE SENTENCE:")
            .cloned()
            .unwrap_or_default(),
        success_criteria: sections
            .get("Success criteria (3 months):")
            .cloned()
            .unwrap_or_default(),
        anti_vision: sections
            .get("Anti-vision (what this project is NOT):")
            .cloned()
            .unwrap_or_default(),
    })
}

/// Parse scope document into structured data
fn parse_scope_document(content: &str) -> Result<crate::catalyst::generator::ScopeData> {
    let sections = extract_sections(content);

    // Extract MVP features as list
    let mvp_text = sections
        .get("MVP (Minimum Viable Product):")
        .cloned()
        .unwrap_or_default();
    let mvp_features = extract_list_items(&mvp_text);

    // Extract Version 2 features
    let v2_text = sections
        .get("Version 2 (NOT NOW - just document):")
        .cloned()
        .unwrap_or_default();
    let version2_features = extract_list_items(&v2_text);

    // Extract Never features
    let never_text = sections
        .get("Never (things I will NOT build):")
        .cloned()
        .unwrap_or_default();
    let never_features = extract_list_items(&never_text);

    Ok(crate::catalyst::generator::ScopeData {
        mvp_features,
        version2_features,
        never_features,
        constraints: sections
            .get("Tech constraints:")
            .cloned()
            .unwrap_or_default(),
    })
}

/// Extract sections from markdown content
fn extract_sections(content: &str) -> HashMap<String, String> {
    let parser = Parser::new(content);
    let mut sections: HashMap<String, String> = HashMap::new();

    let mut current_header: Option<String> = None;
    let mut current_content: Vec<String> = Vec::new();
    let mut in_header = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                // Save previous section
                if let Some(header) = current_header.take() {
                    sections.insert(header, current_content.join(" ").trim().to_string());
                    current_content.clear();
                }
                in_header = true;
            }
            Event::End(TagEnd::Heading(_)) => {
                in_header = false;
            }
            Event::Text(text) => {
                if in_header {
                    current_header = Some(text.to_string());
                } else if current_header.is_some() {
                    current_content.push(text.to_string());
                }
            }
            Event::Code(code) => {
                if !in_header && current_header.is_some() {
                    current_content.push(format!("`{}`", code));
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if !in_header && current_header.is_some() {
                    current_content.push("\n".to_string());
                }
            }
            _ => {}
        }
    }

    // Save final section
    if let Some(header) = current_header {
        sections.insert(header, current_content.join(" ").trim().to_string());
    }

    sections
}

/// Extract list items from text (lines starting with -, *, or numbers)
fn extract_list_items(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('-') || trimmed.starts_with('*') {
                Some(
                    trimmed
                        .trim_start_matches('-')
                        .trim_start_matches('*')
                        .trim()
                        .to_string(),
                )
            } else if trimmed.chars().next().map_or(false, |c| c.is_numeric()) {
                // Handle numbered lists (e.g., "1. Item")
                trimmed
                    .split_once('.')
                    .map(|(_, rest)| rest.trim().to_string())
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}

/// Clean LLM response by removing markdown code fences and extra whitespace
fn clean_llm_response(response: &str) -> String {
    let mut cleaned = response.trim().to_string();

    // Remove markdown code fences if present
    if cleaned.starts_with("```markdown") {
        cleaned = cleaned
            .strip_prefix("```markdown")
            .unwrap_or(&cleaned)
            .to_string();
    }
    if cleaned.starts_with("```") {
        cleaned = cleaned.strip_prefix("```").unwrap_or(&cleaned).to_string();
    }
    if cleaned.ends_with("```") {
        cleaned = cleaned.strip_suffix("```").unwrap_or(&cleaned).to_string();
    }

    cleaned.trim().to_string()
}

/// Parse tech stack document into structured data
fn parse_tech_stack_document(content: &str) -> Result<crate::catalyst::generator::TechStackData> {
    let sections = extract_sections(content);

    Ok(crate::catalyst::generator::TechStackData {
        language: sections
            .get("Language:")
            .cloned()
            .unwrap_or_default(),
        framework: sections
            .get("Framework/Library:")
            .cloned()
            .unwrap_or_default(),
        database: sections.get("Database (if needed):").cloned(),
        justification: sections
            .get("Justification:")
            .cloned()
            .unwrap_or_default(),
    })
}

/// Parse architecture document into structured data
fn parse_architecture_document(
    content: &str,
) -> Result<crate::catalyst::generator::ArchitectureData> {
    let sections = extract_sections(content);

    Ok(crate::catalyst::generator::ArchitectureData {
        folder_structure: sections
            .get("Folder structure:")
            .cloned()
            .unwrap_or_default(),
        data_model: sections
            .get("Data model (main entities):")
            .cloned()
            .unwrap_or_default(),
        user_flow: sections
            .get("Flow (user journey):")
            .cloned()
            .unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vision_document() {
        let content = r#"
# My problem (personal):

I need to manage my projects better.

## Solution in ONE SENTENCE:

A CLI tool for project planning.

## Success criteria (3 months):

Successfully plan 5 projects.

## Anti-vision (what this project is NOT):

Not a full project management suite.
"#;

        let vision = parse_vision_document(content).unwrap();
        assert!(vision.problem.contains("manage my projects"));
        assert!(vision.solution.contains("CLI tool"));
        assert!(vision.success_criteria.contains("5 projects"));
        assert!(vision.anti_vision.contains("Not a full"));
    }

    #[test]
    fn test_extract_list_items() {
        let text = r#"
- Feature one
- Feature two
* Feature three
1. Feature four
2. Feature five
"#;

        let items = extract_list_items(text);
        assert_eq!(items.len(), 5);
        assert_eq!(items[0], "Feature one");
        assert_eq!(items[3], "Feature four");
    }

    #[test]
    fn test_clean_llm_response() {
        let response = "```markdown\n# Header\nContent\n```";
        let cleaned = clean_llm_response(response);
        assert_eq!(cleaned, "# Header\nContent");

        let response2 = "```\n# Header\n```";
        let cleaned2 = clean_llm_response(response2);
        assert_eq!(cleaned2, "# Header");
    }
}
