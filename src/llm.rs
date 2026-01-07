//! LLM Provider Module - Claude and Gemini API Integration
//!
//! Provides a unified interface for interacting with LLM providers (Claude and Gemini).
//! Handles API authentication, request formatting, and response parsing.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// LLM Provider types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmProvider {
    OpenRouter,
    Claude,
    Gemini,
}

impl LlmProvider {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openrouter" => Some(LlmProvider::OpenRouter),
            "claude" | "anthropic" => Some(LlmProvider::Claude),
            "gemini" | "google" => Some(LlmProvider::Gemini),
            _ => None,
        }
    }
}

/// LLM Client for making API requests
pub struct LlmClient {
    provider: LlmProvider,
    api_key: String,
    model: String,
    http_client: reqwest::Client,
}

impl LlmClient {
    /// Create a new LLM client
    ///
    /// # Arguments
    /// * `provider` - The LLM provider to use
    /// * `api_key` - API key for authentication
    /// * `model` - Model identifier (e.g., "claude-3-5-sonnet-20241022")
    pub fn new(provider: LlmProvider, api_key: String, model: String) -> Self {
        Self {
            provider,
            api_key,
            model,
            http_client: reqwest::Client::new(),
        }
    }

    /// Send a prompt to the LLM and get a response
    ///
    /// # Arguments
    /// * `prompt` - The prompt to send to the LLM
    ///
    /// # Returns
    /// The LLM's text response
    pub async fn complete(&self, prompt: &str) -> Result<String> {
        match self.provider {
            LlmProvider::OpenRouter => self.complete_openrouter(prompt).await,
            LlmProvider::Claude => self.complete_claude(prompt).await,
            LlmProvider::Gemini => {
                let (text, _) = self.complete_gemini_with_system("", prompt).await?;
                Ok(text)
            }
        }
    }

    /// Send a prompt with system message to the LLM and get a response
    ///
    /// # Arguments
    /// * `system_prompt` - The system prompt to set context
    /// * `user_prompt` - The user prompt
    ///
    /// # Returns
    /// The LLM's text response
    pub async fn complete_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String> {
        match self.provider {
            LlmProvider::OpenRouter => {
                self.complete_openrouter_with_system(system_prompt, user_prompt)
                    .await
            }
            LlmProvider::Claude => {
                self.complete_claude_with_system(system_prompt, user_prompt)
                    .await
            }
            LlmProvider::Gemini => {
                let (text, _) = self.complete_gemini_with_system(system_prompt, user_prompt)
                    .await?;
                Ok(text)
            }
        }
    }

    /// Send a prompt with system message to `OpenRouter` API
    async fn complete_openrouter_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String> {
        #[derive(Serialize)]
        struct OpenRouterRequest {
            model: String,
            messages: Vec<OpenRouterMessage>,
            max_tokens: Option<u32>,
        }

        #[derive(Serialize)]
        struct OpenRouterMessage {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct OpenRouterResponse {
            choices: Vec<OpenRouterChoice>,
        }

        #[derive(Deserialize)]
        struct OpenRouterChoice {
            message: OpenRouterResponseMessage,
        }

        #[derive(Deserialize)]
        struct OpenRouterResponseMessage {
            content: String,
        }

        let request = OpenRouterRequest {
            model: self.model.clone(),
            messages: vec![
                OpenRouterMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OpenRouterMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            max_tokens: Some(4096),
        };

        let response = self
            .http_client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/nullvektordom/nexus_cli")
            .header("X-Title", "Nexus CLI")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenRouter API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            anyhow::bail!("OpenRouter API error ({status}): {error_text}");
        }

        let openrouter_response: OpenRouterResponse = response
            .json()
            .await
            .context("Failed to parse OpenRouter API response")?;

        openrouter_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No content in OpenRouter response"))
    }

    /// Send a prompt to `OpenRouter` API (OpenAI-compatible format)
    async fn complete_openrouter(&self, prompt: &str) -> Result<String> {
        #[derive(Serialize)]
        struct OpenRouterRequest {
            model: String,
            messages: Vec<OpenRouterMessage>,
            max_tokens: Option<u32>,
        }

        #[derive(Serialize)]
        struct OpenRouterMessage {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct OpenRouterResponse {
            choices: Vec<OpenRouterChoice>,
        }

        #[derive(Deserialize)]
        struct OpenRouterChoice {
            message: OpenRouterResponseMessage,
        }

        #[derive(Deserialize)]
        struct OpenRouterResponseMessage {
            content: String,
        }

        let request = OpenRouterRequest {
            model: self.model.clone(),
            messages: vec![OpenRouterMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: Some(4096),
        };

        let response = self
            .http_client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/nullvektordom/nexus_cli")
            .header("X-Title", "Nexus CLI")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenRouter API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            anyhow::bail!("OpenRouter API error ({status}): {error_text}");
        }

        let openrouter_response: OpenRouterResponse = response
            .json()
            .await
            .context("Failed to parse OpenRouter API response")?;

        openrouter_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No content in OpenRouter response"))
    }

    /// Send a prompt with system message to Claude API
    async fn complete_claude_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String> {
        #[derive(Serialize)]
        struct ClaudeRequest {
            model: String,
            max_tokens: u32,
            system: String,
            messages: Vec<ClaudeMessage>,
        }

        #[derive(Serialize)]
        struct ClaudeMessage {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct ClaudeResponse {
            content: Vec<ClaudeContent>,
        }

        #[derive(Deserialize)]
        struct ClaudeContent {
            text: String,
        }

        let request = ClaudeRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            system: system_prompt.to_string(),
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            }],
        };

        let response = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            anyhow::bail!("Claude API error ({status}): {error_text}");
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .context("Failed to parse Claude API response")?;

        claude_response
            .content
            .first()
            .map(|c| c.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No content in Claude response"))
    }

    /// Send a prompt to Claude API
    async fn complete_claude(&self, prompt: &str) -> Result<String> {
        #[derive(Serialize)]
        struct ClaudeRequest {
            model: String,
            max_tokens: u32,
            messages: Vec<ClaudeMessage>,
        }

        #[derive(Serialize)]
        struct ClaudeMessage {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct ClaudeResponse {
            content: Vec<ClaudeContent>,
        }

        #[derive(Deserialize)]
        struct ClaudeContent {
            text: String,
        }

        let request = ClaudeRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            anyhow::bail!("Claude API error ({status}): {error_text}");
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .context("Failed to parse Claude API response")?;

        claude_response
            .content
            .first()
            .map(|c| c.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No content in Claude response"))
    }

    /// Send a prompt to Gemini API
    async fn complete_gemini(&self, prompt: &str) -> Result<String> {
        let (text, _) = self.complete_gemini_with_system("", prompt).await?;
        Ok(text)
    }

    /// Send a prompt with system message to Gemini API
    pub async fn complete_gemini_full(
        &self,
        system_prompt: &str,
        history: &[(String, String)],
        previous_thought_signature: Option<String>,
        user_prompt: &str,
    ) -> Result<(String, Option<String>)> {
        #[derive(Serialize)]
        struct GeminiRequest {
            contents: Vec<GeminiContent>,
            #[serde(skip_serializing_if = "Option::is_none")]
            system_instruction: Option<GeminiContent>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generation_config: Option<GeminiGenerationConfig>,
        }

        #[derive(Serialize)]
        struct GeminiGenerationConfig {
            #[serde(skip_serializing_if = "Option::is_none")]
            thinking_config: Option<GeminiThinkingConfig>,
        }

        #[derive(Serialize)]
        struct GeminiThinkingConfig {
            include_thoughts: bool,
            thinking_level: String,
        }

        #[derive(Serialize)]
        struct GeminiContent {
            parts: Vec<GeminiPart>,
            #[serde(skip_serializing_if = "Option::is_none")]
            role: Option<String>,
        }

        #[derive(Serialize, Deserialize)]
        struct GeminiPart {
            text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            thought_signature: Option<String>,
        }

        #[derive(Deserialize)]
        struct GeminiResponse {
            candidates: Vec<GeminiCandidate>,
        }

        #[derive(Deserialize)]
        struct GeminiCandidate {
            content: GeminiResponseContent,
        }

        #[derive(Deserialize)]
        struct GeminiResponseContent {
            parts: Vec<GeminiPart>,
        }

        let mut contents = Vec::new();
        for (role, text) in history {
            contents.push(GeminiContent {
                parts: vec![GeminiPart {
                    text: text.clone(),
                    thought_signature: None,
                }],
                role: Some(role.clone()),
            });
        }

        // If we have a previous thought signature, it should be in the last model message
        if let Some(sig) = previous_thought_signature {
            if let Some(last_model) = contents.iter_mut().rev().find(|c| c.role.as_deref() == Some("model")) {
                if let Some(part) = last_model.parts.first_mut() {
                    part.thought_signature = Some(sig);
                }
            }
        }

        contents.push(GeminiContent {
            parts: vec![GeminiPart {
                text: user_prompt.to_string(),
                thought_signature: None,
            }],
            role: Some("user".to_string()),
        });

        let request = GeminiRequest {
            contents,
            system_instruction: if system_prompt.is_empty() {
                None
            } else {
                Some(GeminiContent {
                    parts: vec![GeminiPart {
                        text: system_prompt.to_string(),
                        thought_signature: None,
                    }],
                    role: None,
                })
            },
            generation_config: Some(GeminiGenerationConfig {
                thinking_config: Some(GeminiThinkingConfig {
                    include_thoughts: true,
                    thinking_level: "high".to_string(),
                }),
            }),
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let response = self
            .http_client
            .post(&url)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            anyhow::bail!("Gemini API error ({status}): {error_text}");
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        let candidate = gemini_response
            .candidates
            .first()
            .ok_or_else(|| anyhow::anyhow!("No candidates in Gemini response"))?;

        let text = candidate.content.parts.first()
            .map(|p| p.text.clone())
            .ok_or_else(|| anyhow::anyhow!("No text in Gemini response"))?;

        let thought_signature = candidate.content.parts.iter()
            .find_map(|p| p.thought_signature.clone());

        Ok((text, thought_signature))
    }

    /// Send a prompt with system message to Gemini API
    async fn complete_gemini_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<(String, Option<String>)> {
        self.complete_gemini_full(system_prompt, &[], None, user_prompt).await
    }
}
