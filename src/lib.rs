//! Nexus CLI Library
//!
//! This library exposes core functionality for examples and testing.

// Module declarations (these files are shared between lib and binary)
#[path = "embeddings.rs"]
pub mod embeddings;

#[path = "history.rs"]
pub mod history;

#[path = "tasks.rs"]
pub mod tasks;

// Re-export commonly used types
pub use embeddings::{generate_embedding, initialize_embeddings, is_initialized, EMBEDDING_DIM};
