use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Typed tool catalogue with semantic descriptions and embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub description_embedding: Vec<f32>,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
    pub connector_id: Option<String>,
    pub plan_required: PlanTier,
    pub rate_limit_rpm: u32,
    pub is_active: bool,
    pub tool_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanTier {
    Free,
    Pro,
    Enterprise,
}

pub struct ToolRegistry {
    tools: HashMap<String, Tool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    /// Register a tool in the catalogue.
    pub fn register(&mut self, tool: Tool) {
        info!(tool_id = %tool.id, tool_name = %tool.name, "Registering tool");
        self.tools.insert(tool.id.clone(), tool);
    }

    /// Search tools by cosine similarity against a query embedding.
    pub fn search(&self, query_embedding: &[f32], top_k: usize, min_score: f32) -> Vec<Tool> {
        let mut scored: Vec<(f32, &Tool)> = self
            .tools
            .values()
            .filter(|t| t.is_active)
            .map(|t| {
                let sim = cosine_similarity(query_embedding, &t.description_embedding);
                (sim, t)
            })
            .filter(|(sim, _)| *sim >= min_score)
            .collect();

        // Sort descending by similarity
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        scored
            .into_iter()
            .take(top_k)
            .map(|(_, tool)| tool.clone())
            .collect()
    }

    /// Look up a tool by ID.
    pub fn get(&self, id: &str) -> Option<&Tool> {
        self.tools.get(id)
    }

    /// Total number of registered tools.
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// List all tool IDs.
    pub fn ids(&self) -> Vec<&String> {
        self.tools.keys().collect()
    }
}

/// Cosine similarity between two equal-length vectors.
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}
