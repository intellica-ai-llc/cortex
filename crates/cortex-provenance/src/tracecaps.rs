#![allow(unused_imports, dead_code, unused_variables)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A provenance capsule recording an agent step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceCaps {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub agent_id: uuid::Uuid,
    pub action: ActionKind,
    pub inputs: Vec<uuid::Uuid>,         // parent capsule IDs
    pub output_hash: Option<String>,
    pub risk_score: f64,
    pub signature: Option<Vec<u8>>,
    pub parent_hashes: Vec<String>,
    pub vap_level: VAPLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionKind {
    Inference,
    ToolCall,
    Decision,
    Effect,
    MemoryAccess,
    DreamPhase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VAPLevel {
    Bronze,
    Silver,
    Gold,
}

/// Accumulator that creates capsules and tracks risk.
pub struct TraceCapsAccumulator {
    history: Vec<TraceCaps>,
    risk_threshold_warn: f64,
    risk_threshold_block: f64,
}

impl TraceCapsAccumulator {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            risk_threshold_warn: 0.7,
            risk_threshold_block: 0.95,
        }
    }

    pub fn attach(
        &mut self,
        agent_id: uuid::Uuid,
        action: ActionKind,
        parent_capsules: &[&TraceCaps],
    ) -> TraceCaps {
        let max_parent_risk = parent_capsules.iter().map(|p| p.risk_score).fold(0.0, f64::max);
        let risk_score = max_parent_risk + 0.05; // simplistic increment

        let capsule = TraceCaps {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            agent_id,
            action,
            inputs: parent_capsules.iter().map(|p| p.id).collect(),
            output_hash: None,
            risk_score,
            signature: None,
            parent_hashes: parent_capsules.iter().map(|p| format!("{:x}", p.id.as_u128())).collect(),
            vap_level: VAPLevel::Silver,
        };

        self.history.push(capsule.clone());
        capsule
    }

    pub fn compute_risk(&self, capsule: &TraceCaps) -> f64 {
        capsule.risk_score
    }
}
