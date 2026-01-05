//! Embeddings Module - Local ONNX-based Embedding Generation
//!
//! Generates embeddings locally using the all-MiniLM-L6-v2 model via ONNX Runtime.
//! This eliminates the need for external API calls and runs efficiently on Fedora machines.

use anyhow::{Context, Result};
use ndarray::Array2;
use ort::session::Session;
use tokenizers::Tokenizer;

/// Default embedding dimension for all-MiniLM-L6-v2
pub const EMBEDDING_DIM: usize = 384;

/// Embedding generator using ONNX Runtime
pub struct EmbeddingGenerator {
    session: Session,
    tokenizer: Tokenizer,
}

impl EmbeddingGenerator {
    /// Create a new embedding generator
    ///
    /// # Arguments
    /// * `model_path` - Path to the ONNX model file
    /// * `tokenizer_path` - Path to the tokenizer JSON file
    ///
    /// # Example
    /// ```no_run
    /// use nexus::embeddings::EmbeddingGenerator;
    ///
    /// # fn example() -> anyhow::Result<()> {
    /// let generator = EmbeddingGenerator::new(
    ///     "models/all-MiniLM-L6-v2.onnx",
    ///     "models/tokenizer.json"
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        // Initialize ONNX Runtime session
        let session = Session::builder()
            .context("Failed to create ONNX session builder")?
            .with_intra_threads(4)
            .context("Failed to set intra threads")?
            .commit_from_file(model_path)
            .with_context(|| format!("Failed to load ONNX model from: {}", model_path))?;

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer from {}: {}", tokenizer_path, e))?;

        Ok(Self {
            session,
            tokenizer,
        })
    }

    /// Generate an embedding vector for the given text
    ///
    /// # Arguments
    /// * `text` - The text to embed
    ///
    /// # Returns
    /// A 384-dimensional embedding vector
    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
        // Tokenize input
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| anyhow::anyhow!("Failed to tokenize text: {}", e))?;

        let input_ids = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();

        // Token type IDs (all zeros for single sentence)
        let token_type_ids = vec![0u32; input_ids.len()];

        // Convert to ndarray format
        let input_ids_array = Array2::from_shape_vec(
            (1, input_ids.len()),
            input_ids.iter().map(|&id| id as i64).collect(),
        )
        .context("Failed to create input_ids array")?;

        let attention_mask_array = Array2::from_shape_vec(
            (1, attention_mask.len()),
            attention_mask.iter().map(|&mask| mask as i64).collect(),
        )
        .context("Failed to create attention_mask array")?;

        let token_type_ids_array = Array2::from_shape_vec(
            (1, token_type_ids.len()),
            token_type_ids.iter().map(|&id| id as i64).collect(),
        )
        .context("Failed to create token_type_ids array")?;

        // Convert to ORT Values
        let input_ids_value = ort::value::Value::from_array(input_ids_array)
            .context("Failed to create input_ids value")?;
        let attention_mask_value = ort::value::Value::from_array(attention_mask_array)
            .context("Failed to create attention_mask value")?;
        let token_type_ids_value = ort::value::Value::from_array(token_type_ids_array)
            .context("Failed to create token_type_ids value")?;

        // Run inference
        let outputs = self
            .session
            .run(ort::inputs![
                "input_ids" => input_ids_value,
                "attention_mask" => attention_mask_value,
                "token_type_ids" => token_type_ids_value
            ])
            .context("Failed to run ONNX inference")?;

        // Extract embeddings (last_hidden_state) and copy data
        let (shape, data) = outputs["last_hidden_state"]
            .try_extract_tensor::<f32>()
            .context("Failed to extract embeddings tensor")?;

        // Reconstruct ndarray from shape and data
        let shape_dims = shape.as_ref();
        if shape_dims.len() != 3 {
            anyhow::bail!("Expected 3D tensor, got {}D", shape_dims.len());
        }

        let batch_size = shape_dims[0] as usize;
        let seq_len = shape_dims[1] as usize;
        let hidden_size = shape_dims[2] as usize;

        // Copy data to owned Vec to avoid lifetime issues
        let data_owned = data.to_vec();

        // Drop outputs to release the mutable borrow on self
        drop(outputs);

        // Mean pooling over sequence length
        let embedding_vec = self.mean_pooling_direct(&data_owned, batch_size, seq_len, hidden_size, &attention_mask)?;

        Ok(embedding_vec)
    }

    /// Perform mean pooling over the sequence dimension (direct from flattened data)
    fn mean_pooling_direct(
        &self,
        data: &[f32],
        batch_size: usize,
        seq_len: usize,
        hidden_size: usize,
        attention_mask: &[u32],
    ) -> Result<Vec<f32>> {
        if batch_size != 1 {
            anyhow::bail!("Expected batch_size=1, got {}", batch_size);
        }

        if seq_len != attention_mask.len() {
            anyhow::bail!(
                "Sequence length mismatch: embeddings={}, attention_mask={}",
                seq_len,
                attention_mask.len()
            );
        }

        // Sum embeddings weighted by attention mask
        let mut pooled = vec![0.0f32; hidden_size];
        let mut mask_sum = 0u32;

        for seq_idx in 0..seq_len {
            let mask_value = attention_mask[seq_idx];
            mask_sum += mask_value;

            if mask_value > 0 {
                let offset = seq_idx * hidden_size;
                for hidden_idx in 0..hidden_size {
                    pooled[hidden_idx] += data[offset + hidden_idx];
                }
            }
        }

        // Compute mean
        if mask_sum > 0 {
            for value in pooled.iter_mut() {
                *value /= mask_sum as f32;
            }
        }

        // Normalize to unit vector
        let norm: f32 = pooled.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for value in pooled.iter_mut() {
                *value /= norm;
            }
        }

        Ok(pooled)
    }

    /// Batch generate embeddings for multiple texts
    pub fn embed_batch(&mut self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        texts.iter().map(|text| self.embed(text)).collect()
    }
}

/// Global embedding generator instance (wrapped in Mutex for interior mutability)
static EMBEDDING_GENERATOR: std::sync::OnceLock<std::sync::Mutex<EmbeddingGenerator>> = std::sync::OnceLock::new();

/// Initialize the global embedding generator
///
/// This should be called once at startup with the paths to the model and tokenizer.
/// Subsequent calls will return an error.
///
/// # Arguments
/// * `model_path` - Path to the ONNX model file
/// * `tokenizer_path` - Path to the tokenizer JSON file
pub fn initialize_embeddings(model_path: &str, tokenizer_path: &str) -> Result<()> {
    let generator = EmbeddingGenerator::new(model_path, tokenizer_path)?;
    EMBEDDING_GENERATOR
        .set(std::sync::Mutex::new(generator))
        .map_err(|_| anyhow::anyhow!("Embedding generator already initialized"))
}

/// Generate an embedding using the global generator
///
/// # Arguments
/// * `text` - The text to embed
///
/// # Returns
/// A 384-dimensional embedding vector (for all-MiniLM-L6-v2)
///
/// # Panics
/// Panics if the embedding generator has not been initialized
pub fn generate_embedding(text: &str) -> Result<Vec<f32>> {
    let generator_mutex = EMBEDDING_GENERATOR
        .get()
        .ok_or_else(|| anyhow::anyhow!("Embedding generator not initialized. Call initialize_embeddings() first."))?;

    let mut generator = generator_mutex
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to acquire lock on embedding generator"))?;

    generator.embed(text)
}

/// Check if the embedding generator is initialized
pub fn is_initialized() -> bool {
    EMBEDDING_GENERATOR.get().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_dimension() {
        // Verify that the constant matches the expected dimension
        assert_eq!(EMBEDDING_DIM, 384);
    }
}
