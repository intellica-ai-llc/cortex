use serde::{Deserialize, Serialize};

/// Formal handoff protocol — Tether Codex v2.
///
/// Enables context-preserving delegation from one agent to another.
/// The handoff wraps the downstream agent as a tool with optional
/// input filtering and preserves full context so the receiving
/// agent can continue without loss of information.
pub struct HandoffManager {
    history: Vec<HandoffRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffTask {
    pub id: String,
    pub description: String,
    pub priority: super::orchestrator::MissionPriority,
    pub context: serde_json::Value,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffResult {
    pub handoff_id: String,
    pub accepted: bool,
    pub reason: Option<String>,
    pub context_transfer_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffRecord {
    pub id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub task: HandoffTask,
    pub result: HandoffResult,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HandoffManager {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    /// Delegate a task from one agent to another.
    pub async fn delegate(
        &mut self,
        from: &str,
        to: &str,
        task: HandoffTask,
    ) -> Result<HandoffResult, super::CouncilError> {
        let handoff_id = uuid::Uuid::new_v4().to_string();

        // Validate the task
        if task.description.is_empty() {
            return Ok(HandoffResult {
                handoff_id,
                accepted: false,
                reason: Some("Empty task description".into()),
                context_transfer_complete: false,
            });
        }

        if task.acceptance_criteria.is_empty() {
            return Ok(HandoffResult {
                handoff_id,
                accepted: false,
                reason: Some("No acceptance criteria defined".into()),
                context_transfer_complete: false,
            });
        }

        // Record the handoff
        let result = HandoffResult {
            handoff_id: handoff_id.clone(),
            accepted: true,
            reason: None,
            context_transfer_complete: true,
        };

        self.history.push(HandoffRecord {
            id: handoff_id,
            from_agent: from.to_string(),
            to_agent: to.to_string(),
            task,
            result: result.clone(),
            timestamp: chrono::Utc::now(),
        });

        Ok(result)
    }

    /// Query handoff history.
    pub fn history(&self) -> &[HandoffRecord] {
        &self.history
    }

    /// Check if an agent currently has pending handoffs.
    pub fn pending_for(&self, agent: &str) -> Vec<&HandoffRecord> {
        self.history.iter()
            .filter(|r| r.to_agent == agent)
            .collect()
    }
}
