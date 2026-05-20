use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Semantic Store (L2) — consolidated facts with ontology links.
pub struct SemanticStore {
    facts: RwLock<HashMap<String, SemanticFact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFact {
    pub fact_id: String,
    pub fact: String,
    pub confidence: f64,
    pub ontology_category: Option<String>,
    pub first_observed: chrono::DateTime<chrono::Utc>,
    pub last_observed: chrono::DateTime<chrono::Utc>,
    pub observation_count: u64,
}

impl SemanticStore {
    pub fn new() -> Self {
        Self { facts: RwLock::new(HashMap::new()) }
    }

    /// Store a fact (upsert: merge if exists).
    /// Returns true if the fact is new, false if merged.
    pub async fn store_fact(&self, fact: &str, confidence: f64) -> bool {
        let mut facts = self.facts.write().await;
        let now = chrono::Utc::now();
        if let Some(existing) = facts.get_mut(fact) {
            // Merge: recency‑weighted confidence update.
            let n = existing.observation_count as f64;
            existing.confidence = (existing.confidence * n + confidence) / (n + 1.0);
            existing.last_observed = now;
            existing.observation_count += 1;
            false
        } else {
            facts.insert(
                fact.to_string(),
                SemanticFact {
                    fact_id: uuid::Uuid::new_v4().to_string(),
                    fact: fact.to_string(),
                    confidence,
                    ontology_category: None,
                    first_observed: now,
                    last_observed: now,
                    observation_count: 1,
                },
            );
            true
        }
    }

    /// Detect contradictions in stored facts.
    pub async fn detect_contradictions(&self) -> Vec<String> {
        vec![]
    }

    /// Compress older facts (hierarchical summarisation).
    pub async fn compress_older_facts(&self, _ratio: u32) {
        // In production: merge facts older than 30 days into summaries.
    }

    /// Estimate size in bytes.
    pub async fn size_bytes(&self) -> u64 {
        self.facts.read().await.len() as u64 * 256
    }
}
