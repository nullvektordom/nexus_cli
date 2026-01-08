//! Genesis Engine
//!
//! Core engine for PROJECT GENESIS - generates foundational planning documents.

use crate::catalyst::engine::parse_vision_document;
use crate::genesis::{build_genesis_user_prompt, parse_genesis_response, get_genesis_system_prompt};
use crate::llm::LlmClient;
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::path::PathBuf;

/// Genesis Engine for creating foundational planning documents
pub struct GenesisEngine {
    /// Path to planning directory (01-PLANNING)
    planning_path: PathBuf,
    /// LLM client for generation
    llm_client: LlmClient,
}

impl GenesisEngine {
    /// Create a new Genesis engine
    pub fn new(planning_path: PathBuf, llm_client: LlmClient) -> Result<Self> {
        Ok(Self {
            planning_path,
            llm_client,
        })
    }

    /// Execute the full Genesis workflow
    ///
    /// This method:
    /// 1. Loads the vision document
    /// 2. Calls the LLM to generate all 4 documents
    /// 3. Presents each document for user review
    /// 4. Writes approved documents to the vault
    pub async fn execute_genesis(&self) -> Result<()> {
        println!("{}", "═══════════════════════════════════════".cyan());
        println!("{}", "  Phase 1: Reading Vision".cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();

        // Load vision document
        let vision_path = self.planning_path.join("01-Problem-and-Vision.md");
        let vision_content = fs::read_to_string(&vision_path)
            .with_context(|| format!("Failed to read vision document: {}", vision_path.display()))?;

        let vision = parse_vision_document(&vision_content)?;

        if !vision.is_complete() {
            anyhow::bail!(
                "Vision document is incomplete. Please fill out all sections:\n\
                • Problem Statement\n\
                • Vision\n\
                • Success Criteria (3 months)\n\
                • Anti-Vision (what this is NOT)"
            );
        }

        println!("{} Vision document loaded and validated", "✓".green().bold());
        println!();

        // Build prompt
        let user_prompt = build_genesis_user_prompt(&vision);
        let system_prompt = get_genesis_system_prompt();

        println!("{}", "═══════════════════════════════════════".cyan());
        println!("{}", "  Phase 2: LLM Generation".cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();

        // SAFETY GATE: Confirm LLM request
        let full_prompt = format!("SYSTEM:\n{}\n\nUSER:\n{}", system_prompt, user_prompt);
        let confirmed = crate::llm::confirm_llm_prompt(&full_prompt, "Project Genesis")?;
        if !confirmed {
            anyhow::bail!("User cancelled Genesis request");
        }

        println!("{}", "  Calling LLM for document synthesis...".dimmed());
        println!("{}", "  This may take 30-60 seconds...".dimmed());
        println!();

        // Call LLM
        let response = self
            .llm_client
            .complete_with_system(&system_prompt, &user_prompt)
            .await
            .with_context(|| {
                "Failed to generate documents via LLM. Possible causes:\n\
                • API key not set or invalid\n\
                • Network connectivity issues\n\
                • LLM provider API error\n\
                • Rate limiting\n\n\
                Check your API key and network connection."
            })?;

        // Debug: Show raw response
        println!("{}", "═══════════════════════════════════════".yellow());
        println!("{}", "  DEBUG: Raw LLM Response".yellow().bold());
        println!("{}", "═══════════════════════════════════════".yellow());
        println!();
        println!("{}", response);
        println!();
        println!("{}", "═══════════════════════════════════════".yellow());
        println!();

        // Parse response into documents
        let documents = parse_genesis_response(&response);

        if documents.len() != 3 {
            anyhow::bail!(
                "LLM did not generate all 3 documents. Got {} documents. \
                Response may need refinement.\n\n\
                Check the raw response above to see if the LLM used the correct ---NEXT_DOC--- separator.",
                documents.len()
            );
        }

        println!("{} Generated {} documents", "✓".green().bold(), documents.len());
        println!();

        // Phase 3: Review and commit
        println!("{}", "═══════════════════════════════════════".cyan());
        println!("{}", "  Phase 3: Review & Commit".cyan().bold());
        println!("{}", "═══════════════════════════════════════".cyan());
        println!();

        let mut approved_count = 0;

        for (filename, content) in documents {
            println!("{}", format!("📄 Reviewing: {}", filename).bold());
            println!("{}", "─".repeat(50).dimmed());
            println!();
            println!("{}", content);
            println!();
            println!("{}", "─".repeat(50).dimmed());
            println!();

            // Interactive review
            let options = vec!["Accept", "Skip", "Abort Genesis"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .items(&options)
                .default(0)
                .interact()?;

            match selection {
                0 => {
                    // Accept - write to file
                    let file_path = self.planning_path.join(&filename);
                    fs::write(&file_path, &content).with_context(|| {
                        format!("Failed to write document: {}", file_path.display())
                    })?;
                    println!("{} Saved: {}", "✓".green().bold(), filename.green());
                    approved_count += 1;
                }
                1 => {
                    // Skip
                    println!("{} Skipped: {}", "⊘".yellow().bold(), filename.yellow());
                }
                2 => {
                    // Abort
                    anyhow::bail!("Genesis aborted by user");
                }
                _ => unreachable!(),
            }

            println!();
        }

        // Final summary
        println!("{}", "═══════════════════════════════════════".green());
        println!("{}", "  Genesis Complete!".green().bold());
        println!("{}", "═══════════════════════════════════════".green());
        println!();
        println!(
            "  {} {} documents approved and saved",
            "✓".green().bold(),
            approved_count
        );
        println!();
        println!("{}", "📋 Next steps:".bold());
        println!("   1. Review generated documents in Obsidian");
        println!("   2. Ensure 01-Problem-and-Vision.md exists and is complete");
        println!("   3. Run {} to validate all planning documents", "nexus gate .".cyan());
        println!("   4. Run {} to generate CLAUDE.md and unlock project", "nexus unlock .".cyan());
        println!();

        Ok(())
    }
}
