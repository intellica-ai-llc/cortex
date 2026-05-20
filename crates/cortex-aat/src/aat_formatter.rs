use serde::{Deserialize, Serialize};

/// Generates IETF AAT JSON records.
pub struct AATFormatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AATRecord {
    pub agent_id: String,
    pub action_type: String,
    pub action_target: String,
    pub action_outcome: String,
    pub trust_level: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub parent_action_ids: Vec<String>,
    pub signature: Vec<u8>,
    pub evidence_hash: String,
}

impl AATFormatter {
    pub fn new() -> Self { Self }
    pub fn format(
        agent_id: &str,
        action_type: &str,
        action_target: &str,
        action_outcome: &str,
        trust_level: &str,
        evidence_hash: &str,
    ) -> AATRecord {
        AATRecord {
            agent_id: agent_id.to_string(),
            action_type: action_type.to_string(),
            action_target: action_target.to_string(),
            action_outcome: action_outcome.to_string(),
            trust_level: trust_level.to_string(),
            timestamp: chrono::Utc::now(),
            parent_action_ids: vec![],
            signature: vec![],
            evidence_hash: evidence_hash.to_string(),
        }
    }
}
