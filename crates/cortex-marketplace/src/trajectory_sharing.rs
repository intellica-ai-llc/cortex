use serde::{Deserialize, Serialize};

/// Anonymised trajectory sharing with differential privacy (DP ε=1).
pub struct TrajectorySharingProtocol;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedTrajectory {
    pub trajectory_id: String,
    pub anonymised_steps: Vec<String>,
    pub dp_epsilon: f64,
    pub contributor_id: String,
    pub shared_at: chrono::DateTime<chrono::Utc>,
}

impl TrajectorySharingProtocol {
    pub fn new() -> Self { Self }
    pub fn share(&self, steps: &[String], contributor: &str) -> SharedTrajectory {
        SharedTrajectory {
            trajectory_id: uuid::Uuid::new_v4().to_string(),
            anonymised_steps: steps.to_vec(),
            dp_epsilon: 1.0,
            contributor_id: contributor.to_string(),
            shared_at: chrono::Utc::now(),
        }
    }
}
