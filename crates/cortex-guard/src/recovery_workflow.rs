use std::collections::HashSet;
use tokio::sync::RwLock;

/// Selective agent re-enablement after kill switch activation.
///
/// After forensic review, the security officer can:
/// - Review the full incident timeline
/// - Selectively re-enable specific agents
/// - Roll back any state changes made by the suspended agent
/// - Generate a compliance-ready incident report
pub struct RecoveryWorkflow {
    cleared_agents: RwLock<HashSet<String>>,
    restored: RwLock<bool>,
}

impl RecoveryWorkflow {
    pub fn new() -> Self {
        Self {
            cleared_agents: RwLock::new(HashSet::new()),
            restored: RwLock::new(false),
        }
    }

    /// Clear an agent for re-enablement after forensic review.
    pub async fn clear_agent(&self, agent_id: &str, _approver: &str) {
        self.cleared_agents.write().await.insert(agent_id.to_string());
    }

    /// Check if an agent has been cleared.
    pub async fn is_cleared(&self, agent_id: &str) -> bool {
        self.cleared_agents.read().await.contains(agent_id)
    }

    /// Mark the system as fully restored.
    pub async fn mark_restored(&self) {
        *self.restored.write().await = true;
        self.cleared_agents.write().await.clear();
    }

    pub async fn is_restored(&self) -> bool {
        *self.restored.read().await
    }
}
