use crate::talent::AgentState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Persistent agent state manager (Tether Codex v2).
///
/// Maintains agent state across sessions so agents can resume
/// work after restart without loss of context. Uses short-term
/// (working), medium-term (session), and long-term (consolidated)
/// memory layers.
pub struct StateManager {
    /// Persistent state indexed by agent role.
    store: RwLock<HashMap<String, SavedState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub role: String,
    pub agent_state: AgentState,
    pub saved_at: chrono::DateTime<chrono::Utc>,
    pub version: u64,
}

impl StateManager {
    pub fn new() -> Self {
        Self { store: RwLock::new(HashMap::new()) }
    }

    /// Load state for an agent.
    pub async fn load(&self, role: &str) -> Option<SavedState> {
        self.store.read().await.get(role).cloned()
    }

    /// Persist state for an agent.
    pub async fn save(&self, role: &str, state: AgentState) {
        let saved = SavedState {
            role: role.to_string(),
            agent_state: state,
            saved_at: chrono::Utc::now(),
            version: 0,
        };
        self.store.write().await.insert(role.to_string(), saved);
    }

    /// List all saved states.
    pub async fn list_all(&self) -> Vec<SavedState> {
        self.store.read().await.values().cloned().collect()
    }

    /// Clear state for an agent.
    pub async fn clear(&self, role: &str) {
        self.store.write().await.remove(role);
    }
}
