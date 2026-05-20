//! Cortex RL Bootstrapper — Self-Improving Research Agent Training (v6).
//!
//! Based on KARL (Databricks, arXiv:2603.05218, Mar 2026): iterative
//! large-batch off-policy RL (OAPL) for training enterprise search agents.
//! Two-phase pipeline: Question-Answer Synthesis (generating hard, diverse
//! questions) and Solution Synthesis (generating multi-step tool-call
//! trajectories). Uses Cycle-Consistent Search (CCS) proxy rewards for
//! gold-supervision-free RL signal.
//!
//! The bootstrapping loop:
//!   SFT Training → RL Fine-Tuning → Iterative Bootstrapping →
//!   Improved agent → Higher-quality trajectories → Retrain.

pub mod karl_pipeline;
pub mod cycle_consistent_eval;
pub mod iterative_bootstrapper;

use std::sync::Arc;

pub struct RLBootstrapper {
    pub karl_pipeline: Arc<karl_pipeline::KARLPipeline>,
    pub ccs_eval: Arc<cycle_consistent_eval::CycleConsistentEvaluator>,
    pub bootstrapper: Arc<iterative_bootstrapper::IterativeBootstrapper>,
}

impl RLBootstrapper {
    pub fn new() -> Self {
        Self {
            karl_pipeline: Arc::new(karl_pipeline::KARLPipeline::new()),
            ccs_eval: Arc::new(cycle_consistent_eval::CycleConsistentEvaluator::new()),
            bootstrapper: Arc::new(iterative_bootstrapper::IterativeBootstrapper::new()),
        }
    }

    /// Run a complete RL bootstrapping cycle.
    pub async fn run_cycle(&self) -> Result<BootstrappingReport, String> {
        self.bootstrapper.run_cycle().await
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BootstrappingReport {
    pub cycle_number: u64,
    pub trajectories_synthesised: u64,
    pub trajectories_accepted: u64,
    pub avg_reward: f64,
    pub performance_delta: f64,
}
