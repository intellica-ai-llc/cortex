//! Cortex Deep Research™ — Sovereign Research Fabric (v6).
//!
//! Domain‑specific search agent trained via OpenSeeker‑v2 SFT‑only
//! recipe on customer data, running on‑premise. Context‑efficient
//! via IterResearch Markovian workspace. Self‑improving via KARL
//! RL bootstrapping with Cycle‑Consistent proxy rewards.
//!
//! Subsystems:
//!   openseeker_trainer          — SFT pipeline, 10.6k data pts
//!   knowledge_graph_expander    — richer exploration paths
//!   tool_set_expander           — broader tool functionality
//!   low_step_filter             — strict quality filtering
//!   cycle_consistent_reward     — gold‑supervision‑free RL signal

pub mod openseeker_trainer;
pub mod knowledge_graph_expander;
pub mod tool_set_expander;
pub mod low_step_filter;
pub mod cycle_consistent_reward;

use std::sync::Arc;

pub struct CortexDeepResearch {
    pub trainer: Arc<openseeker_trainer::OpenSeekerTrainer>,
    pub kg_expander: Arc<knowledge_graph_expander::KnowledgeGraphExpander>,
    pub tool_expander: Arc<tool_set_expander::ToolSetExpander>,
    pub step_filter: Arc<low_step_filter::LowStepFilter>,
    pub ccs_reward: Arc<cycle_consistent_reward::CycleConsistentRewarder>,
}

impl CortexDeepResearch {
    pub fn new() -> Self {
        Self {
            trainer: Arc::new(openseeker_trainer::OpenSeekerTrainer::new()),
            kg_expander: Arc::new(knowledge_graph_expander::KnowledgeGraphExpander::new()),
            tool_expander: Arc::new(tool_set_expander::ToolSetExpander::new()),
            step_filter: Arc::new(low_step_filter::LowStepFilter::new()),
            ccs_reward: Arc::new(cycle_consistent_reward::CycleConsistentRewarder::new()),
        }
    }
}
