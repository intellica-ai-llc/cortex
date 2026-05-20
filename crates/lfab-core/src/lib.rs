//! LFAB — Lightweight Future‑Aware Brain (v6).
//!
//! On‑device cognitive runtime for Cortex Mobile Brain.
//! Includes the S‑HAI Core (probabilistic planning), Predictive
//! World Engine, token pruner for efficiency, and latent bridge
//! for server offload.

pub mod engine;
pub mod model_freshness;
pub mod token_pruner;

use std::sync::Arc;

pub struct LFABRuntime {
    pub engine: Arc<engine::SHAICore>,
    pub freshness: Arc<model_freshness::ModelFreshnessChecker>,
    pub token_pruner: Arc<token_pruner::TokenPruner>,
}

impl LFABRuntime {
    pub fn new(model_version: &str) -> Self {
        Self {
            engine: Arc::new(engine::SHAICore::new()),
            freshness: Arc::new(model_freshness::ModelFreshnessChecker::new(model_version)),
            token_pruner: Arc::new(token_pruner::TokenPruner::new()),
        }
    }
}
