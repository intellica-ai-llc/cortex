use serde::{Deserialize, Serialize};

/// Low‑Step Filter — strict quality filtering.
///
/// OpenSeeker‑v2 modification #3: "Strict low‑step filtering."
/// Trajectories with fewer than a minimum number of tool calls
/// are excluded from training. This ensures the model learns to
/// perform deep, multi‑step research rather than surface‑level
/// retrieval. Combined with IterResearch's Markovian workspace,
/// the agent is trained for depth, not breadth.
pub struct LowStepFilter {
    /// Minimum tool‑call steps for a trajectory to be included.
    min_steps: u32,
    /// Maximum steps before a trajectory is considered noisy/divergent.
    max_steps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterStats {
    pub trajectories_evaluated: u64,
    pub trajectories_accepted: u64,
    pub trajectories_rejected: u64,
    pub rejection_reasons: Vec<String>,
    pub acceptance_rate: f64,
}

impl LowStepFilter {
    pub fn new() -> Self {
        Self { min_steps: 3, max_steps: 200 }
    }

    /// Evaluate whether a research trajectory passes the step filter.
    ///
    /// Trajectories with fewer than min_steps are insufficiently
    /// deep (the model didn't explore). Trajectories exceeding
    /// max_steps may be divergent or stuck in loops.
    pub fn evaluate(&self, trajectory_steps: u32) -> FilterDecision {
        if trajectory_steps < self.min_steps {
            FilterDecision::Rejected {
                reason: format!(
                    "Trajectory has {} steps, below minimum of {}",
                    trajectory_steps, self.min_steps
                ),
            }
        } else if trajectory_steps > self.max_steps {
            FilterDecision::Rejected {
                reason: format!(
                    "Trajectory has {} steps, above maximum of {}",
                    trajectory_steps, self.max_steps
                ),
            }
        } else {
            FilterDecision::Accepted
        }
    }

    /// Filter a batch of trajectories and return statistics.
    pub fn filter_batch(&self, step_counts: &[u32]) -> FilterStats {
        let total = step_counts.len() as u64;
        let mut accepted = 0u64;
        let mut rejected = 0u64;
        let mut reasons = Vec::new();

        for &steps in step_counts {
            match self.evaluate(steps) {
                FilterDecision::Accepted => { accepted += 1; }
                FilterDecision::Rejected { reason } => {
                    rejected += 1;
                    reasons.push(reason);
                }
            }
        }

        FilterStats {
            trajectories_evaluated: total,
            trajectories_accepted: accepted,
            trajectories_rejected: rejected,
            rejection_reasons: reasons,
            acceptance_rate: if total > 0 { accepted as f64 / total as f64 } else { 0.0 },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterDecision {
    Accepted,
    Rejected { reason: String },
}
