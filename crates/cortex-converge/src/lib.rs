//! Cortex Converge™ — Convergent Reasoning Layer (v7).
//!
//! Runs three reasoning paths in parallel (Strategic/Opus, Analytical/Sonnet,
//! Creative/Haiku) and converges them into a consensus answer with
//! per‑claim confidence scores.

pub mod converge_controller;
pub mod strategic_reasoner;
pub mod analytical_reasoner;
pub mod creative_reasoner;
pub mod synthesiser;

use std::sync::Arc;

pub struct ConvergeEngine {
    pub controller: Arc<converge_controller::ConvergeController>,
    pub strategic: Arc<strategic_reasoner::StrategicReasoner>,
    pub analytical: Arc<analytical_reasoner::AnalyticalReasoner>,
    pub creative: Arc<creative_reasoner::CreativeReasoner>,
    pub synthesiser: Arc<synthesiser::Synthesiser>,
}

impl ConvergeEngine {
    pub fn new() -> Self {
        Self {
            controller: Arc::new(converge_controller::ConvergeController::new()),
            strategic: Arc::new(strategic_reasoner::StrategicReasoner),
            analytical: Arc::new(analytical_reasoner::AnalyticalReasoner),
            creative: Arc::new(creative_reasoner::CreativeReasoner),
            synthesiser: Arc::new(synthesiser::Synthesiser),
        }
    }
}
