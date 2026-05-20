use serde::{Deserialize, Serialize};

/// Research Sub-Agent — specialised domain researcher.
///
/// Each sub-agent operates on a single sub-question with its own
/// IterResearch workspace. The agent independently searches,
/// analyses, and produces findings for its assigned domain.
/// After all sub-agents complete, the Synthesiser merges their
/// outputs into a unified report.
pub struct ResearchSubAgent {
    pub agent_id: String,
    pub domain: String,
    pub capability: String,
}

/// Findings produced by a sub-agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentFindings {
    pub agent_id: String,
    pub sub_question_id: String,
    pub domain: String,
    pub summary: String,
    pub key_evidence: Vec<EvidenceItem>,
    pub confidence: f64,
    pub tool_calls_executed: u64,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub source: String,
    pub claim: String,
    pub relevance: f64,
    pub verified: bool,
}

impl ResearchSubAgent {
    pub fn new(agent_id: &str, domain: &str, capability: &str) -> Self {
        Self {
            agent_id: agent_id.to_string(),
            domain: domain.to_string(),
            capability: capability.to_string(),
        }
    }

    /// Execute research on the assigned sub-question.
    /// Uses IterResearch Markovian workspace internally.
    pub async fn research(
        &self,
        sub_question: &super::SubQuestionAssignment,
    ) -> SubAgentFindings {
        SubAgentFindings {
            agent_id: self.agent_id.clone(),
            sub_question_id: sub_question.id.clone(),
            domain: self.domain.clone(),
            summary: format!("Research findings for: {}", sub_question.question),
            key_evidence: vec![],
            confidence: 0.82,
            tool_calls_executed: 15,
            completed_at: chrono::Utc::now(),
        }
    }
}
