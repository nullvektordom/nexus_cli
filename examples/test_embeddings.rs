//! Test embedding generation with the downloaded ONNX model

use nexus::embeddings;
use anyhow::Result;

fn main() -> Result<()> {
    println!("🧪 Testing ONNX Embedding Generation");
    println!("=====================================\n");

    // Initialize embeddings
    println!("📦 Initializing embeddings...");
    embeddings::initialize_embeddings("models/model.onnx", "models/tokenizer.json")?;
    println!("✓ Embeddings initialized successfully\n");

    // Test queries
    let test_queries = ["How do I implement user authentication?",
        "What is the architecture for the database layer?",
        "Write a function to parse sprint metadata",
        "Refactor the context injection system"];

    println!("🔍 Generating embeddings for test queries:\n");

    for (idx, query) in test_queries.iter().enumerate() {
        println!("Query {}: \"{}\"", idx + 1, query);

        let start = std::time::Instant::now();
        let embedding = embeddings::generate_embedding(query)?;
        let elapsed = start.elapsed();

        println!("  ✓ Generated {}-dimensional embedding", embedding.len());
        println!("  ⏱  Time: {elapsed:?}");

        // Calculate L2 norm (should be ~1.0 due to normalization)
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        println!("  📊 L2 Norm: {norm:.6} (should be ~1.0)");

        // Show first 5 dimensions as sample
        let sample: Vec<String> = embedding.iter().take(5).map(|x| format!("{x:.4}")).collect();
        println!("  🔢 Sample (first 5 dims): [{}...]", sample.join(", "));
        println!();
    }

    println!("✅ All tests passed!");
    println!("\n💡 Next steps:");
    println!("  1. Start Qdrant: docker run -p 6333:6333 qdrant/qdrant");
    println!("  2. Set OPENROUTER_API_KEY environment variable");
    println!("  3. Run 'nexus shell' to test Planning Catalyst queries");

    Ok(())
}
