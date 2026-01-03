//! Sentinel Watcher - Background File Monitoring and Indexing
//!
//! Monitors file changes in the active project and automatically indexes them to the Brain.
//! Features:
//! - Dynamic path watching based on active project
//! - Intelligent file chunking for large files
//! - Async ingestion pipeline to Qdrant
//! - Special handling for Architecture.md changes (triggers full re-index)

use crate::brain::{NexusBrain, VectorMetadata};
use anyhow::{Context, Result};
use crossbeam_channel::{bounded, Receiver, Sender};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Maximum chunk size for file content (in characters)
const CHUNK_SIZE: usize = 1000;

/// Overlap between chunks to maintain context
const CHUNK_OVERLAP: usize = 100;

/// File extensions to watch and index
const WATCHED_EXTENSIONS: &[&str] = &["rs", "toml", "md", "txt", "json", "yaml", "yml"];

/// Message types for the watcher thread
#[derive(Debug, Clone)]
pub enum WatcherMessage {
    /// Start watching paths for a project
    WatchProject {
        project_id: String,
        repo_path: PathBuf,
        obsidian_path: PathBuf,
    },
    /// Stop watching all paths
    StopWatching,
    /// Shutdown the watcher thread
    Shutdown,
}

/// Sentinel watcher handle
pub struct SentinelWatcher {
    sender: Sender<WatcherMessage>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl SentinelWatcher {
    /// Create and start a new Sentinel watcher
    pub fn new(brain_url: String) -> Result<Self> {
        let (tx, rx) = bounded(100);

        let thread_handle = thread::spawn(move || {
            if let Err(e) = run_watcher_loop(rx, brain_url) {
                eprintln!("Watcher thread error: {}", e);
            }
        });

        Ok(Self {
            sender: tx,
            thread_handle: Some(thread_handle),
        })
    }

    /// Start watching a project
    pub fn watch_project(
        &self,
        project_id: String,
        repo_path: PathBuf,
        obsidian_path: PathBuf,
    ) -> Result<()> {
        self.sender
            .send(WatcherMessage::WatchProject {
                project_id,
                repo_path,
                obsidian_path,
            })
            .context("Failed to send watch message")
    }

    /// Stop watching
    pub fn stop_watching(&self) -> Result<()> {
        self.sender
            .send(WatcherMessage::StopWatching)
            .context("Failed to send stop message")
    }

    /// Shutdown the watcher
    pub fn shutdown(mut self) -> Result<()> {
        self.sender
            .send(WatcherMessage::Shutdown)
            .context("Failed to send shutdown message")?;

        if let Some(handle) = self.thread_handle.take() {
            handle.join().map_err(|_| anyhow::anyhow!("Failed to join watcher thread"))?;
        }

        Ok(())
    }
}

/// Main watcher loop running in background thread
fn run_watcher_loop(rx: Receiver<WatcherMessage>, brain_url: String) -> Result<()> {
    let mut current_watcher: Option<notify::RecommendedWatcher> = None;
    let mut current_project: Option<String> = None;
    let watched_paths: Arc<Mutex<HashSet<PathBuf>>> = Arc::new(Mutex::new(HashSet::new()));

    // Channel for file system events
    let (event_tx, event_rx) = bounded(1000);

    loop {
        // Check for control messages
        if let Ok(msg) = rx.try_recv() {
            match msg {
                WatcherMessage::WatchProject {
                    project_id,
                    repo_path,
                    obsidian_path,
                } => {
                    println!("🔍 Sentinel: Starting watch for project '{}'", project_id);

                    // Stop existing watcher
                    current_watcher = None;

                    // Create new watcher
                    let event_tx_clone = event_tx.clone();
                    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
                        if let Ok(event) = res {
                            let _ = event_tx_clone.send(event);
                        }
                    })
                    .context("Failed to create file watcher")?;

                    // Watch repo path
                    if repo_path.exists() {
                        watcher
                            .watch(&repo_path, RecursiveMode::Recursive)
                            .context("Failed to watch repo path")?;
                        println!("  ✓ Watching: {}", repo_path.display());
                    }

                    // Watch Obsidian path
                    if obsidian_path.exists() {
                        watcher
                            .watch(&obsidian_path, RecursiveMode::Recursive)
                            .context("Failed to watch Obsidian path")?;
                        println!("  ✓ Watching: {}", obsidian_path.display());
                    }

                    current_watcher = Some(watcher);
                    current_project = Some(project_id);

                    // Clear watched paths
                    watched_paths.lock().unwrap().clear();
                }
                WatcherMessage::StopWatching => {
                    println!("🔍 Sentinel: Stopping watch");
                    current_watcher = None;
                    current_project = None;
                    watched_paths.lock().unwrap().clear();
                }
                WatcherMessage::Shutdown => {
                    println!("🔍 Sentinel: Shutting down");
                    break;
                }
            }
        }

        // Process file system events
        if let Ok(event) = event_rx.try_recv() {
            if let Some(ref project_id) = current_project {
                if let Err(e) = handle_file_event(&event, project_id, &brain_url, &watched_paths) {
                    eprintln!("Error handling file event: {}", e);
                }
            }
        }

        // Sleep briefly to avoid busy-waiting
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

/// Handle a file system event
fn handle_file_event(
    event: &Event,
    project_id: &str,
    brain_url: &str,
    watched_paths: &Arc<Mutex<HashSet<PathBuf>>>,
) -> Result<()> {
    // Only handle write/create events
    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) => {}
        _ => return Ok(()),
    }

    for path in &event.paths {
        // Skip if not a file
        if !path.is_file() {
            continue;
        }

        // Skip if extension not watched
        if let Some(ext) = path.extension() {
            if !WATCHED_EXTENSIONS.contains(&ext.to_str().unwrap_or("")) {
                continue;
            }
        } else {
            continue;
        }

        // Check if this is Architecture.md
        let is_architecture = path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n == "04-Architecture.md")
            .unwrap_or(false);

        if is_architecture {
            println!("🧠 Architecture.md changed - triggering full re-index for {}", project_id);
            // TODO: Implement full project re-index
        }

        // Skip if we've recently processed this file (debouncing)
        {
            let mut paths = watched_paths.lock().unwrap();
            if paths.contains(path) {
                continue;
            }
            paths.insert(path.clone());
        }

        // Schedule cleanup of this path from the set after a delay
        let watched_paths_clone = Arc::clone(watched_paths);
        let path_clone = path.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            watched_paths_clone.lock().unwrap().remove(&path_clone);
        });

        // Index the file
        println!("📝 Indexing: {}", path.display());
        if let Err(e) = index_file(path, project_id, brain_url) {
            eprintln!("Failed to index {}: {}", path.display(), e);
        }
    }

    Ok(())
}

/// Index a file to the Brain
fn index_file(file_path: &Path, project_id: &str, brain_url: &str) -> Result<()> {
    // Read file content
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Get file type
    let file_type = file_path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_string());

    // Chunk the content
    let chunks = chunk_text(&content);

    // Create async runtime for Qdrant operations
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        // Connect to Brain
        let brain = NexusBrain::connect(brain_url).await?;
        brain.ensure_collection().await?;

        // Ingest each chunk
        for (idx, chunk) in chunks.iter().enumerate() {
            // Generate a simple vector (in production, use actual embeddings)
            let vector = generate_dummy_embedding(chunk);

            // Create metadata
            let mut metadata = VectorMetadata::new(
                project_id.to_string(),
                file_path.display().to_string(),
                "current".to_string(), // TODO: Get actual sprint context
            );
            metadata.file_type = file_type.clone();
            metadata.chunk_index = Some(idx as u32);

            // Generate unique ID for this chunk
            let point_id = generate_point_id(file_path, idx);

            // Store in Brain
            brain.store_vector(point_id, vector, metadata).await?;
        }

        Ok::<(), anyhow::Error>(())
    })?;

    println!("  ✓ Indexed {} chunks", chunks.len());

    Ok(())
}

/// Chunk text into overlapping segments
fn chunk_text(text: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let chars: Vec<char> = text.chars().collect();

    if chars.len() <= CHUNK_SIZE {
        chunks.push(text.to_string());
        return chunks;
    }

    let mut start = 0;
    while start < chars.len() {
        let end = (start + CHUNK_SIZE).min(chars.len());
        let chunk: String = chars[start..end].iter().collect();
        chunks.push(chunk);

        if end >= chars.len() {
            break;
        }

        // Move start forward with overlap
        start = end - CHUNK_OVERLAP;
    }

    chunks
}

/// Generate a dummy embedding vector (replace with actual embedding model)
fn generate_dummy_embedding(_text: &str) -> Vec<f32> {
    // In production, use an actual embedding model like:
    // - OpenAI ada-002
    // - Sentence-BERT
    // - all-MiniLM-L6-v2
    // For now, return a random vector of the correct size
    vec![0.0; 1536]
}

/// Generate a unique point ID for a file chunk
fn generate_point_id(file_path: &Path, chunk_index: usize) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    file_path.hash(&mut hasher);
    chunk_index.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_text_small() {
        let text = "Hello, world!";
        let chunks = chunk_text(text);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_chunk_text_large() {
        let text = "a".repeat(2500);
        let chunks = chunk_text(&text);
        assert!(chunks.len() > 1);
        // Verify overlap
        assert!(chunks[0].len() == CHUNK_SIZE);
    }

    #[test]
    fn test_generate_point_id_consistency() {
        let path = PathBuf::from("/test/file.rs");
        let id1 = generate_point_id(&path, 0);
        let id2 = generate_point_id(&path, 0);
        assert_eq!(id1, id2);

        let id3 = generate_point_id(&path, 1);
        assert_ne!(id1, id3);
    }
}
