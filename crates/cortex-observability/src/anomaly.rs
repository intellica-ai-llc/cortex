use serde::{Deserialize, Serialize};

/// Pattern‑based anomaly detection on agent tool call sequences.
pub struct AnomalyDetector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyAlert {
    pub agent_id: String,
    pub pattern: String,
    pub severity: AnomalySeverity,
    pub detected_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity { Info, Warning, Critical }

impl AnomalyDetector {
    pub fn new() -> Self { Self }
    pub fn detect(&self, _tool_call_sequence: &[String]) -> Vec<AnomalyAlert> {
        vec![]
    }
}
