use serde_json::json;

/// IETF Agent Audit Trail formatter.
pub struct AATFormatter;

impl AATFormatter {
    pub fn new() -> Self { Self }

    pub fn format(
        agent_id: &str,
        action: &str,
        outcome: &str,
        trust_level: &str,
        evidence_hash: &str,
    ) -> serde_json::Value {
        json!({
            "agent_id": agent_id,
            "action_type": action,
            "action_outcome": outcome,
            "trust_level": trust_level,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "evidence_hash": evidence_hash,
            "signature": "..."
        })
    }
}
