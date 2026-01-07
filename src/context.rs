//! Context Injection Module
//!
//! Retrieves and assembles context for LLM prompts by:
//! - Querying Qdrant for relevant architecture documentation
//! - Reading active sprint information from Obsidian
//! - Wrapping user input with context in a structured template

use crate::brain::{Layer, NexusBrain, SearchResult};
use crate::config::NexusConfig;
use anyhow::{Context as AnyhowContext, Result};
use std::path::Path;

/// Architecture context retrieved from Qdrant
#[derive(Debug, Clone)]
pub struct ArchitectureContext {
    pub snippets: Vec<SearchResult>,
}

/// Sprint context retrieved from Obsidian files
#[derive(Debug, Clone)]
#[allow(clippy::struct_field_names)] // sprint_context is descriptive in this domain
pub struct SprintContext {
    pub sprint_id: String,
    pub tasks: String,
    pub sprint_context: String,
}

/// Complete context for LLM prompt injection
#[derive(Debug, Clone)]
pub struct ActiveContext {
    pub architecture: ArchitectureContext,
    pub sprint: Option<SprintContext>,
}

impl ActiveContext {
    /// Check if context is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.architecture.snippets.is_empty() && self.sprint.is_none()
    }
}

/// Context template that wraps user input with retrieved context
pub struct ContextTemplate {
    architecture_snippets: Vec<String>,
    sprint_id: Option<String>,
    unfinished_tasks: Option<String>,
    sprint_context: Option<String>,
    user_request: String,
}

impl ContextTemplate {
    /// Create a new context template from active context and user input
    pub fn new(context: ActiveContext, user_request: String) -> Self {
        let architecture_snippets = context
            .architecture
            .snippets
            .iter()
            .map(|s| format!("From {}:\n{}", s.file_name(), s.content))
            .collect();

        let (sprint_id, unfinished_tasks, sprint_context) = if let Some(sprint) = context.sprint {
            (
                Some(sprint.sprint_id),
                Some(sprint.tasks),
                Some(sprint.sprint_context),
            )
        } else {
            (None, None, None)
        };

        Self {
            architecture_snippets,
            sprint_id,
            unfinished_tasks,
            sprint_context,
            user_request,
        }
    }

    /// Render the template as a formatted string for LLM input
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Architecture context
        if !self.architecture_snippets.is_empty() {
            output.push_str("[SYSTEM ARCHITECTURE RULES]\n");
            use std::fmt::Write;
            for (idx, snippet) in self.architecture_snippets.iter().enumerate() {
                let _ = writeln!(output, "\n--- Architecture Reference {} ---", idx + 1);
                output.push_str(snippet);
                output.push('\n');
            }
            output.push('\n');
        }

        // Sprint context
        if let Some(ref sprint_id) = self.sprint_id {
            output.push_str("[CURRENT SPRINT STATE]\n");
            use std::fmt::Write;
            let _ = writeln!(output, "Sprint: {sprint_id}\n");

            if let Some(ref tasks) = self.unfinished_tasks {
                output.push_str("Unfinished Tasks:\n");
                output.push_str(tasks);
                output.push_str("\n\n");
            }

            if let Some(ref context) = self.sprint_context {
                output.push_str("Sprint Context:\n");
                output.push_str(context);
                output.push_str("\n\n");
            }
        }

        // User request
        output.push_str("[USER REQUEST]\n");
        output.push_str(&self.user_request);
        output.push('\n');

        output
    }
}

/// Minimum relevance score threshold for architecture snippets
pub const RELEVANCE_THRESHOLD: f32 = 0.75;

/// Retrieve active context by querying Qdrant and reading Obsidian files concurrently
///
/// # Arguments
/// * `user_query` - The user's query/request (used for semantic search)
/// * `project_id` - The active project identifier
/// * `qdrant_url` - URL for the Qdrant server
/// * `obsidian_root` - Root path to the Obsidian vault
/// * `config` - Project configuration
///
/// # Returns
/// `ActiveContext` containing architecture and sprint information
pub async fn get_active_context(
    user_query: &str,
    project_id: &str,
    qdrant_url: &str,
    obsidian_root: &Path,
    config: &NexusConfig,
) -> Result<ActiveContext> {
    // Generate query embedding using local ONNX model
    let query_vector = generate_query_embedding(user_query)?;

    // Spawn both tasks concurrently
    let architecture_task = tokio::spawn({
        let qdrant_url = qdrant_url.to_string();
        let project_id = project_id.to_string();
        let query_vector = query_vector.clone();

        async move { retrieve_architecture_context(&qdrant_url, &project_id, query_vector).await }
    });

    let sprint_task = tokio::spawn({
        let obsidian_root = obsidian_root.to_path_buf();
        let config = config.clone();

        async move { retrieve_sprint_context(&obsidian_root, &config).await }
    });

    // Await both tasks
    let (architecture_result, sprint_result) = tokio::join!(architecture_task, sprint_task);

    // Unwrap spawn results and handle errors
    let mut architecture = architecture_result
        .context("Architecture retrieval task panicked")?
        .unwrap_or_else(|e| {
            eprintln!("Warning: Failed to retrieve architecture context: {e}");
            ArchitectureContext {
                snippets: Vec::new(),
            }
        });

    // Filter architecture snippets by relevance threshold (0.75)
    architecture.snippets.retain(|snippet| snippet.score >= RELEVANCE_THRESHOLD);

    let sprint = sprint_result
        .context("Sprint retrieval task panicked")?
        .ok(); // Sprint context is optional

    Ok(ActiveContext {
        architecture,
        sprint,
    })
}

/// Retrieve architecture context from Qdrant
async fn retrieve_architecture_context(
    qdrant_url: &str,
    project_id: &str,
    query_vector: Vec<f32>,
) -> Result<ArchitectureContext> {
    let brain = NexusBrain::connect(qdrant_url)
        .await
        .context("Failed to connect to Qdrant")?;

    // Search for top 3 architecture snippets
    let snippets = brain
        .search(
            query_vector,
            3, // Top 3 results
            project_id,
            Some(vec![Layer::ProjectArchitecture, Layer::GlobalStandard]),
        )
        .await
        .context("Failed to search architecture in Qdrant")?;

    Ok(ArchitectureContext { snippets })
}

/// Retrieve sprint context from Obsidian files
async fn retrieve_sprint_context(
    obsidian_root: &Path,
    config: &NexusConfig,
) -> Result<SprintContext> {
    // Get active sprint from config
    let sprint_id = config
        .state
        .as_ref()
        .and_then(|s| s.active_sprint.as_ref())
        .map(|s| s.current.clone())
        .ok_or_else(|| anyhow::anyhow!("No active sprint configured"))?;

    // Build paths to sprint files
    let sprint_dir = obsidian_root
        .join(&config.structure.sprint_dir)
        .join(&sprint_id);

    let tasks_path = sprint_dir.join("Tasks.md");
    let context_path = sprint_dir.join("Sprint-Context.md");

    // Read files concurrently
    let tasks_task = tokio::fs::read_to_string(&tasks_path);
    let context_task = tokio::fs::read_to_string(&context_path);

    let (tasks_result, context_result) = tokio::join!(tasks_task, context_task);

    // Parse tasks file to extract unfinished tasks
    let tasks_content = tasks_result.with_context(|| {
        format!(
            "Failed to read Tasks.md from: {}",
            tasks_path.display()
        )
    })?;

    let unfinished_tasks = extract_unfinished_tasks(&tasks_content);

    let sprint_context = context_result.with_context(|| {
        format!(
            "Failed to read Sprint-Context.md from: {}",
            context_path.display()
        )
    })?;

    Ok(SprintContext {
        sprint_id,
        tasks: unfinished_tasks,
        sprint_context,
    })
}

/// Extract unfinished tasks from Tasks.md content
fn extract_unfinished_tasks(content: &str) -> String {
    let mut unfinished = Vec::new();

    for line in content.lines() {
        // Look for unchecked task items: - [ ] or * [ ]
        if line.contains("- [ ]") || line.contains("* [ ]") {
            unfinished.push(line.trim().to_string());
        }
    }

    if unfinished.is_empty() {
        "No unfinished tasks".to_string()
    } else {
        unfinished.join("\n")
    }
}

/// Generate a query embedding using the local ONNX model
///
/// Uses the all-MiniLM-L6-v2 model (384 dimensions) running locally via ONNX Runtime.
/// This eliminates external API dependencies and runs efficiently on Fedora machines.
///
/// # Arguments
/// * `query` - The query text to embed
///
/// # Returns
/// A 384-dimensional embedding vector (for all-MiniLM-L6-v2)
///
/// # Note
/// The embedding generator must be initialized via `crate::embeddings::initialize_embeddings()`
/// before calling this function.
fn generate_query_embedding(query: &str) -> Result<Vec<f32>> {
    use crate::embeddings;

    // Check if embeddings are initialized
    if !embeddings::is_initialized() {
        // Return a warning but continue with zero vector for graceful degradation
        eprintln!("Warning: Embedding generator not initialized. Using zero vector.");
        eprintln!("Call embeddings::initialize_embeddings() to enable semantic search.");
        return Ok(vec![0.0; embeddings::EMBEDDING_DIM]);
    }

    embeddings::generate_embedding(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_unfinished_tasks() {
        let content = r"
# Sprint Tasks

- [x] Completed task
- [ ] Unfinished task 1
- [ ] Unfinished task 2
* [x] Another completed
* [ ] Unfinished task 3
        ";

        let result = extract_unfinished_tasks(content);

        assert!(result.contains("Unfinished task 1"));
        assert!(result.contains("Unfinished task 2"));
        assert!(result.contains("Unfinished task 3"));
        assert!(!result.contains("Completed task"));
    }

    #[test]
    fn test_context_template_render() {
        let context = ActiveContext {
            architecture: ArchitectureContext {
                snippets: vec![],
            },
            sprint: Some(SprintContext {
                sprint_id: "sprint-3".to_string(),
                tasks: "- [ ] Task 1\n- [ ] Task 2".to_string(),
                sprint_context: "Focus on feature X".to_string(),
            }),
        };

        let template = ContextTemplate::new(context, "Write a new function".to_string());
        let rendered = template.render();

        assert!(rendered.contains("[CURRENT SPRINT STATE]"));
        assert!(rendered.contains("Sprint: sprint-3"));
        assert!(rendered.contains("[USER REQUEST]"));
        assert!(rendered.contains("Write a new function"));
    }
}
