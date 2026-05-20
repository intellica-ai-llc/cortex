use std::collections::HashMap;
use tracing::debug;

/// Cosine-similarity tool discovery (MeetingMind ClawRouter pattern).
pub struct EmbeddingRouter {
    /// Cache of pre-computed tool embeddings for fast similarity search.
    cache: HashMap<String, Vec<f32>>,
}

impl EmbeddingRouter {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    /// Convert a natural language string into a fixed-size embedding vector.
    /// Production will use a real embedding model; this is a deterministic proxy.
    pub fn embed(&self, text: &str) -> Vec<f32> {
        // Use a simple bag-of-trigrams hash to produce a 128-dim vector.
        let mut vec = vec![0.0f32; 128];
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len().saturating_sub(2) {
            let trigram = format!("{}{}{}", chars[i], chars[i+1], chars[i+2]);
            let hash = seahash::hash(trigram.as_bytes());
            let idx = (hash % 128) as usize;
            vec[idx] += 1.0;
        }

        // L2-normalise
        let norm: f32 = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 0.0 {
            vec.iter_mut().for_each(|v| *v /= norm);
        }

        vec
    }

    /// Register a tool's embedding for future searches.
    pub fn register(&mut self, tool_id: &str, embedding: Vec<f32>) {
        debug!(tool_id, dims = embedding.len(), "Registering tool embedding");
        self.cache.insert(tool_id.to_string(), embedding);
    }

    /// Retrieve a cached embedding.
    pub fn get(&self, tool_id: &str) -> Option<&Vec<f32>> {
        self.cache.get(tool_id)
    }

    /// Number of cached embeddings.
    pub fn len(&self) -> usize {
        self.cache.len()
    }
}

// Use seahash for consistent fast hashing across platforms
mod seahash {
    pub fn hash(data: &[u8]) -> u64 {
        let mut state = 0x6eed_0e9d_a4d9_4e9b_u64;
        for (i, &byte) in data.iter().enumerate() {
            state = state.wrapping_mul(0xa076_1d64_78bd_642f)
                .wrapping_add((byte as u64).wrapping_mul((i as u64 + 1) * 0x9e37_79b9_7f4a_7c15));
        }
        state
    }
}
