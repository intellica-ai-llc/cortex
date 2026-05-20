use crate::benchmark_registry::BenchmarkRegistry;
use crate::experiment_trait::{ValidatableExperiment, ExperimentError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Experiment Designer — NL→Experiment matching (One-Eval NL2Bench pattern).
///
/// Based on One-Eval (Shen et al., arXiv:2603.09821): converts natural-
/// language evaluation requests into executable, traceable, customizable
/// evaluation workflows. Uses cosine similarity between the user's NL
/// description and each experiment's nl_description() to find the best match.
pub struct ExperimentDesigner {
    registry: Arc<BenchmarkRegistry>,
}

/// A resolved experiment with parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentSpec {
    pub experiment_id: String,
    pub resolved_params: serde_json::Value,
    pub confidence: f64,
    pub alternative_experiments: Vec<String>,
}

impl ExperimentDesigner {
    pub fn new(registry: Arc<BenchmarkRegistry>) -> Self {
        Self { registry }
    }

    /// Parse a natural-language request and find the best-matching experiment.
    ///
    /// Uses a simple token-overlap similarity. In production, this uses
    /// the Cortex EmbeddingRouter for semantic matching.
    pub fn resolve(&self, nl: &str) -> Result<ExperimentSpec, ExperimentError> {
        let lower = nl.to_lowercase();
        let mut scored: Vec<(f64, &str)> = self.registry.all_ids().iter()
            .filter_map(|id| {
                let exp = self.registry.get(id).ok()?;
                let desc = exp.nl_description().to_lowercase();
                // Jaccard-like token overlap
                let nl_tokens: std::collections::HashSet<_> = lower.split_whitespace().collect();
                let desc_tokens: std::collections::HashSet<_> = desc.split_whitespace().collect();
                let intersection = nl_tokens.intersection(&desc_tokens).count();
                let union = nl_tokens.union(&desc_tokens).count();
                let similarity = if union > 0 { intersection as f64 / union as f64 } else { 0.0 };
                Some((similarity, *id))
            })
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        if scored.is_empty() || scored[0].0 < 0.05 {
            return Err(ExperimentError::NotFound(
                format!("No experiment matched: '{}'", nl)
            ));
        }

        let best = scored[0];
        let alternatives: Vec<String> = scored.iter().skip(1).take(3)
            .map(|(_, id)| id.to_string())
            .collect();

        Ok(ExperimentSpec {
            experiment_id: best.1.to_string(),
            resolved_params: serde_json::json!({}),
            confidence: best.0,
            alternative_experiments: alternatives,
        })
    }
}
