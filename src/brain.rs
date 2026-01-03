//! Brain Module - Qdrant Vector Database Integration
//!
//! Manages connection to the Hetzner Qdrant node for semantic search and knowledge storage.
//! Provides methods for:
//! - Connecting to Qdrant via gRPC over Tailscale
//! - Managing the nexus_brain collection
//! - Storing vectors with rich metadata (project_id, file_path, sprint_context)
//! - Querying the semantic brain

use anyhow::{Context, Result};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, GetCollectionInfoResponse, PointStruct,
    ScoredPoint, SearchPointsBuilder, UpsertPointsBuilder, Value, VectorParamsBuilder,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collection name for the Nexus brain
pub const COLLECTION_NAME: &str = "nexus_brain";

/// Vector dimension size (using OpenAI ada-002 compatible size)
pub const VECTOR_SIZE: u64 = 1536;

/// Metadata attached to each vector in the collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMetadata {
    /// Unique identifier for the project
    pub project_id: String,

    /// Path to the source file (relative to project root)
    pub file_path: String,

    /// Sprint context (e.g., "sprint-5-nexus-sentinel")
    pub sprint_context: String,

    /// Optional: File type (e.g., "rust", "markdown", "toml")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,

    /// Optional: Content chunk index (for large files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_index: Option<u32>,

    /// Optional: Timestamp when indexed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_at: Option<String>,
}

impl VectorMetadata {
    /// Create new metadata
    pub fn new(project_id: String, file_path: String, sprint_context: String) -> Self {
        Self {
            project_id,
            file_path,
            sprint_context,
            file_type: None,
            chunk_index: None,
            indexed_at: Some(chrono::Utc::now().to_rfc3339()),
        }
    }

    /// Convert to Qdrant payload format
    pub fn to_payload(&self) -> HashMap<String, Value> {
        let mut payload = HashMap::new();
        payload.insert("project_id".to_string(), self.project_id.clone().into());
        payload.insert("file_path".to_string(), self.file_path.clone().into());
        payload.insert(
            "sprint_context".to_string(),
            self.sprint_context.clone().into(),
        );

        if let Some(ref file_type) = self.file_type {
            payload.insert("file_type".to_string(), file_type.clone().into());
        }
        if let Some(chunk_index) = self.chunk_index {
            payload.insert("chunk_index".to_string(), (chunk_index as i64).into());
        }
        if let Some(ref indexed_at) = self.indexed_at {
            payload.insert("indexed_at".to_string(), indexed_at.clone().into());
        }

        payload
    }
}

/// Qdrant client wrapper for the Nexus brain
pub struct NexusBrain {
    client: Qdrant,
}

impl NexusBrain {
    /// Create a new connection to the Qdrant node
    ///
    /// # Arguments
    /// * `url` - The gRPC URL of the Qdrant server (e.g., "http://100.x.x.x:6334")
    ///
    /// # Example
    /// ```no_run
    /// use nexus::brain::NexusBrain;
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let brain = NexusBrain::connect("http://100.64.0.1:6334").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(url: &str) -> Result<Self> {
        let client = Qdrant::from_url(url)
            .build()
            .context("Failed to create Qdrant client")?;

        Ok(Self { client })
    }

    /// Ensure the nexus_brain collection exists with the correct schema
    pub async fn ensure_collection(&self) -> Result<()> {
        // Check if collection exists
        let exists = self
            .client
            .collection_exists(COLLECTION_NAME)
            .await
            .context("Failed to check if collection exists")?;

        if !exists {
            // Create collection with on-disk payload storage
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(COLLECTION_NAME)
                        .vectors_config(
                            VectorParamsBuilder::new(VECTOR_SIZE, Distance::Cosine).on_disk(true),
                        )
                        .on_disk_payload(true),
                )
                .await
                .context("Failed to create collection")?;
        }

        Ok(())
    }

    /// Get collection information and stats
    pub async fn get_collection_info(&self) -> Result<GetCollectionInfoResponse> {
        let info = self
            .client
            .collection_info(COLLECTION_NAME)
            .await
            .context("Failed to get collection info")?;

        Ok(info)
    }

    /// Check if the brain is online and accessible
    pub async fn health_check(&self) -> Result<BrainHealth> {
        // Get collection info to verify connection
        let response = self.get_collection_info().await?;
        let info = response
            .result
            .ok_or_else(|| anyhow::anyhow!("No collection info in response"))?;

        Ok(BrainHealth {
            online: true,
            collection_name: COLLECTION_NAME.to_string(),
            points_count: info.points_count.unwrap_or(0),
            segments_count: info.segments_count,
            vectors_count: 0, // Not directly available in new API
            indexed_vectors_count: info.indexed_vectors_count.unwrap_or(0),
            disk_data_size: 0, // Not directly available, would need to sum segment sizes
            ram_data_size: 0,  // Not directly available
        })
    }

    /// Store a vector with metadata in the brain
    ///
    /// # Arguments
    /// * `id` - Unique point ID
    /// * `vector` - The embedding vector
    /// * `metadata` - Metadata for this vector
    pub async fn store_vector(
        &self,
        id: u64,
        vector: Vec<f32>,
        metadata: VectorMetadata,
    ) -> Result<()> {
        if vector.len() != VECTOR_SIZE as usize {
            anyhow::bail!(
                "Vector size mismatch: expected {}, got {}",
                VECTOR_SIZE,
                vector.len()
            );
        }

        let point = PointStruct::new(id, vector, metadata.to_payload());

        self.client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, vec![point]))
            .await
            .context("Failed to upsert point")?;

        Ok(())
    }

    /// Search for similar vectors in the collection
    ///
    /// # Arguments
    /// * `query_vector` - The query embedding vector
    /// * `limit` - Maximum number of results to return
    /// * `project_id` - Optional project ID filter
    /// * `file_pattern` - Optional file path pattern (e.g., "Architecture.md")
    pub async fn search(
        &self,
        query_vector: Vec<f32>,
        limit: u64,
        project_id: Option<&str>,
        file_pattern: Option<&str>,
    ) -> Result<Vec<SearchResult>> {
        // Build filter conditions
        let mut conditions = Vec::new();

        if let Some(pid) = project_id {
            conditions.push(Condition::matches("project_id", pid.to_string()));
        }

        if let Some(pattern) = file_pattern {
            conditions.push(Condition::matches("file_path", pattern.to_string()));
        }

        // Build search request
        let mut search_builder =
            SearchPointsBuilder::new(COLLECTION_NAME, query_vector, limit).with_payload(true);

        if !conditions.is_empty() {
            search_builder = search_builder.filter(Filter::must(conditions));
        }

        // Execute search
        let results = self
            .client
            .search_points(search_builder)
            .await
            .context("Failed to search points")?;

        // Convert to SearchResult
        let search_results = results
            .result
            .into_iter()
            .map(SearchResult::from_scored_point)
            .collect();

        Ok(search_results)
    }

    /// Search specifically for architecture-related content
    pub async fn search_architecture(
        &self,
        query_vector: Vec<f32>,
        project_id: &str,
        limit: u64,
    ) -> Result<Vec<SearchResult>> {
        self.search(
            query_vector,
            limit,
            Some(project_id),
            Some("Architecture.md"),
        )
        .await
    }
}

/// Search result from the Brain
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub score: f32,
    pub file_path: String,
    pub content: String,
    #[allow(dead_code)]
    pub project_id: Option<String>,
    pub file_type: Option<String>,
    #[allow(dead_code)]
    pub chunk_index: Option<u32>,
}

impl SearchResult {
    /// Convert from Qdrant ScoredPoint
    fn from_scored_point(point: ScoredPoint) -> Self {
        let payload = point.payload;

        let file_path = payload
            .get("file_path")
            .and_then(|v| match &v.kind {
                Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "unknown".to_string());

        let project_id = payload.get("project_id").and_then(|v| match &v.kind {
            Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => Some(s.clone()),
            _ => None,
        });

        let file_type = payload.get("file_type").and_then(|v| match &v.kind {
            Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => Some(s.clone()),
            _ => None,
        });

        let chunk_index = payload.get("chunk_index").and_then(|v| match &v.kind {
            Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) => Some(*i as u32),
            _ => None,
        });

        // For now, we don't have the actual content stored in payload
        // In production, you'd want to store it or retrieve from original file
        let content = format!("[Content from chunk {}]", chunk_index.unwrap_or(0));

        Self {
            score: point.score,
            file_path,
            content,
            project_id,
            file_type,
            chunk_index,
        }
    }

    /// Format as a citation string
    #[allow(dead_code)]
    pub fn format_citation(&self) -> String {
        format!(
            "From {}{}: {}",
            self.file_path,
            self.chunk_index
                .map(|i| format!(" (chunk {})", i))
                .unwrap_or_default(),
            self.content
        )
    }

    /// Get a short file name for display
    pub fn file_name(&self) -> String {
        std::path::Path::new(&self.file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&self.file_path)
            .to_string()
    }
}

/// Health status of the Nexus brain
#[derive(Debug, Clone)]
pub struct BrainHealth {
    #[allow(dead_code)]
    pub online: bool,
    pub collection_name: String,
    pub points_count: u64,
    pub segments_count: u64,
    pub vectors_count: u64,
    pub indexed_vectors_count: u64,
    pub disk_data_size: u64,
    pub ram_data_size: u64,
}

impl BrainHealth {
    /// Format memory usage in human-readable format
    pub fn format_memory(&self) -> (String, String) {
        (
            format_bytes(self.ram_data_size),
            format_bytes(self.disk_data_size),
        )
    }
}

/// Format bytes into human-readable format
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_metadata_creation() {
        let metadata = VectorMetadata::new(
            "nexus_cli".to_string(),
            "src/main.rs".to_string(),
            "sprint-5".to_string(),
        );

        assert_eq!(metadata.project_id, "nexus_cli");
        assert_eq!(metadata.file_path, "src/main.rs");
        assert_eq!(metadata.sprint_context, "sprint-5");
        assert!(metadata.indexed_at.is_some());
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 bytes");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }
}
