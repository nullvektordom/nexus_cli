//! History Module - Project-Linked Conversation History
//!
//! Stores the last 5 turns of REPL conversation in the project's Obsidian folder.
//! This enables context persistence when switching between laptop and desktop.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Maximum number of conversation turns to store
pub const MAX_HISTORY_TURNS: usize = 5;

/// A single conversation turn (user input + assistant response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    /// Timestamp of the turn
    pub timestamp: String,
    /// User's input query
    pub user_input: String,
    /// Assistant's response (optional - may not be present for failed queries)
    pub assistant_response: Option<String>,
    /// Project ID this turn belongs to
    pub project_id: String,
}

/// Conversation history for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationHistory {
    /// List of conversation turns (most recent last)
    pub turns: Vec<ConversationTurn>,
    /// Project ID this history belongs to
    pub project_id: String,
}

impl ConversationHistory {
    /// Create a new empty conversation history
    pub fn new(project_id: String) -> Self {
        Self {
            turns: Vec::new(),
            project_id,
        }
    }

    /// Add a new turn to the history
    ///
    /// Automatically maintains the maximum of MAX_HISTORY_TURNS by removing oldest turns.
    pub fn add_turn(&mut self, user_input: String, assistant_response: Option<String>) {
        let turn = ConversationTurn {
            timestamp: chrono::Utc::now().to_rfc3339(),
            user_input,
            assistant_response,
            project_id: self.project_id.clone(),
        };

        self.turns.push(turn);

        // Keep only the last MAX_HISTORY_TURNS
        if self.turns.len() > MAX_HISTORY_TURNS {
            self.turns.drain(0..self.turns.len() - MAX_HISTORY_TURNS);
        }
    }

    /// Get the conversation context as a formatted string
    ///
    /// This can be included in LLM prompts to provide conversation continuity.
    pub fn get_context_string(&self) -> String {
        if self.turns.is_empty() {
            return String::new();
        }

        let mut context = String::from("[CONVERSATION HISTORY]\n\n");

        for (idx, turn) in self.turns.iter().enumerate() {
            context.push_str(&format!("--- Turn {} ---\n", idx + 1));
            context.push_str(&format!("User: {}\n", turn.user_input));

            if let Some(ref response) = turn.assistant_response {
                // Truncate long responses to first 200 chars
                let truncated = if response.len() > 200 {
                    format!("{}...", &response[..200])
                } else {
                    response.clone()
                };
                context.push_str(&format!("Assistant: {}\n", truncated));
            } else {
                context.push_str("Assistant: [No response]\n");
            }

            context.push('\n');
        }

        context
    }

    /// Load conversation history from the project's Obsidian folder
    ///
    /// # Arguments
    /// * `obsidian_root` - Path to the project's Obsidian root
    /// * `project_id` - Project identifier
    ///
    /// # Returns
    /// Loaded history or a new empty history if the file doesn't exist
    pub fn load(obsidian_root: &Path, project_id: &str) -> Result<Self> {
        let history_path = Self::get_history_path(obsidian_root);

        if !history_path.exists() {
            // Return new empty history if file doesn't exist
            return Ok(Self::new(project_id.to_string()));
        }

        let content = std::fs::read_to_string(&history_path)
            .with_context(|| format!("Failed to read history from: {}", history_path.display()))?;

        let mut history: ConversationHistory = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse history JSON from: {}", history_path.display()))?;

        // Update project_id in case it changed
        history.project_id = project_id.to_string();

        Ok(history)
    }

    /// Save conversation history to the project's Obsidian folder
    ///
    /// # Arguments
    /// * `obsidian_root` - Path to the project's Obsidian root
    pub fn save(&self, obsidian_root: &Path) -> Result<()> {
        let history_path = Self::get_history_path(obsidian_root);

        // Ensure parent directory exists
        if let Some(parent) = history_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize history to JSON")?;

        std::fs::write(&history_path, json)
            .with_context(|| format!("Failed to write history to: {}", history_path.display()))?;

        Ok(())
    }

    /// Get the path to the history file
    fn get_history_path(obsidian_root: &Path) -> PathBuf {
        obsidian_root
            .join("00-MANAGEMENT")
            .join(".nexus_history.json")
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.turns.clear();
    }

    /// Get the number of turns in history
    pub fn len(&self) -> usize {
        self.turns.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.turns.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_turn() {
        let mut history = ConversationHistory::new("test_project".to_string());

        history.add_turn("Hello".to_string(), Some("Hi there!".to_string()));
        assert_eq!(history.len(), 1);

        history.add_turn("How are you?".to_string(), Some("I'm good!".to_string()));
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_max_history_turns() {
        let mut history = ConversationHistory::new("test_project".to_string());

        // Add more than MAX_HISTORY_TURNS
        for i in 0..10 {
            history.add_turn(format!("Query {}", i), Some(format!("Response {}", i)));
        }

        // Should only keep the last MAX_HISTORY_TURNS
        assert_eq!(history.len(), MAX_HISTORY_TURNS);

        // Verify the oldest turns were removed
        assert!(history.turns[0].user_input.contains("Query 5"));
    }

    #[test]
    fn test_context_string() {
        let mut history = ConversationHistory::new("test_project".to_string());

        history.add_turn("Hello".to_string(), Some("Hi!".to_string()));
        history.add_turn("Goodbye".to_string(), None);

        let context = history.get_context_string();

        assert!(context.contains("[CONVERSATION HISTORY]"));
        assert!(context.contains("User: Hello"));
        assert!(context.contains("Assistant: Hi!"));
        assert!(context.contains("User: Goodbye"));
        assert!(context.contains("[No response]"));
    }
}
