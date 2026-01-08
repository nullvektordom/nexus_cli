use anyhow::{Context, Result};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct,
    SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use crate::embeddings::generate_embedding;
use std::collections::HashMap;
use uuid::Uuid;

pub const MEMORY_URL: &str = "http://100.105.8.97:6334";  // gRPC port
pub const COLLECTION_NAME: &str = "nexus_ledger";
pub const VECTOR_SIZE: u64 = 384;

pub struct NexusMemory {
    client: Qdrant,
}

impl NexusMemory {
    pub async fn connect() -> Result<Self> {
        let client = Qdrant::from_url(MEMORY_URL)
            .skip_compatibility_check() // Disable version check for Tailscale/gRPC compatibility
            .build()
            .context("Failed to create Qdrant client for memory")?;
        
        let memory = Self { client };
        // Ensure collection exists on connect
        if let Err(e) = memory.ensure_collection().await {
            eprintln!("Warning: Failed to ensure memory collection: {e}");
        }
        Ok(memory)
    }

    async fn ensure_collection(&self) -> Result<()> {
        let exists = self.client.collection_exists(COLLECTION_NAME).await?;
        if !exists {
            self.client.create_collection(
                CreateCollectionBuilder::new(COLLECTION_NAME)
                    .vectors_config(VectorParamsBuilder::new(VECTOR_SIZE, Distance::Cosine))
            ).await?;
        }
        Ok(())
    }

    pub async fn store_decision(&self, text: &str) -> Result<()> {
        let vector = generate_embedding(text)?;
        let mut payload = HashMap::<String, qdrant_client::qdrant::Value>::new();
        payload.insert("role".to_string(), "architectural_decision".into());
        payload.insert("content".to_string(), text.to_string().into());

        let id = Uuid::new_v4().to_string();
        
        let point = PointStruct::new(id, vector, payload);
        self.client.upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, vec![point])).await?;
        Ok(())
    }

    pub async fn retrieve_context(&self, query: &str) -> Result<Vec<String>> {
        let vector = generate_embedding(query)?;
        let search_result = self.client.search_points(
            SearchPointsBuilder::new(COLLECTION_NAME, vector, 3)
                .with_payload(true)
        ).await?;

        let decisions = search_result.result.into_iter()
            .filter_map(|p| {
                p.payload.get("content")
                    .and_then(|v| match &v.kind {
                        Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => Some(s.clone()),
                        _ => None,
                    })
            })
            .collect();
        
        Ok(decisions)
    }
}
