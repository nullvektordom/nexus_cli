use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusSession {
    pub last_thought_signature: Option<String>,
    pub messages: Vec<SessionMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMessage {
    pub role: String,
    pub content: String,
}

impl NexusSession {
    pub fn load(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                last_thought_signature: None,
                messages: Vec::new(),
            });
        }
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).context("Failed to parse session file")
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content).context("Failed to write session file")
    }

    pub fn add_message(&mut self, role: String, content: String) {
        self.messages.push(SessionMessage { role, content });
        if self.messages.len() > 10 { // Keep a bit more than 5 to be safe, but user said 5
            self.messages.remove(0);
        }
    }
    
    pub fn get_last_5_messages(&self) -> Vec<SessionMessage> {
        let start = if self.messages.len() > 5 {
            self.messages.len() - 5
        } else {
            0
        };
        self.messages[start..].to_vec()
    }
}
