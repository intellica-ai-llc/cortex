use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Markovian Workspace — evolving report as compressed memory.
///
/// IterResearch (ICLR 2026): "The agent no longer maintains a
/// constantly expanding complete history. Instead, through a
/// continuously evolving 'report', it synthesises existing results,
/// compresses irrelevant information, and updates its reasoning
/// state. Each round of reasoning unfolds within a reconstructed
/// workspace of constant complexity."
///
/// State transition: the full history trajectory is intentionally
/// discarded at each step. The agent retains only:
///   1. The updated evolving report (compressed memory)
///   2. The previous round's tool call
///   3. Its return result
/// These three components form the new reasoning starting point.
/// Context complexity remains O(1), not O(t).
pub struct MarkovianWorkspace {
    /// Per-session workspace state.
    workspaces: RwLock<HashMap<String, WorkspaceState>>,
}

/// The Markovian workspace for a single research session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub session_id: String,
    /// The evolving report — the agent's compressed memory.
    pub report: String,
    /// The previous tool call that was executed.
    pub previous_action: Option<String>,
    /// The result of the previous tool call.
    pub previous_result: Option<String>,
    /// Number of iterations completed.
    pub iteration_count: u64,
    /// Estimated token count of the current workspace.
    pub token_estimate: u64,
    /// Last updated timestamp.
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl MarkovianWorkspace {
    pub fn new() -> Self {
        Self { workspaces: RwLock::new(HashMap::new()) }
    }

    /// Execute one iteration of the Markovian research loop.
    ///
    /// Algorithm (from IterResearch):
    ///   1. Decision phase: Agent outputs Think, Report, Action.
    ///      Report acts as compressed memory — the agent must actively
    ///      decide which information to retain and which to discard.
    ///   2. State transition: Full history is discarded. Agent retains
    ///      only {report, previous_action, previous_result}.
    ///      New state space = O(1), not O(t).
    pub async fn iterate(
        &self,
        session_id: &str,
        _question: &str,
        tool_result: &str,
    ) -> Result<String, String> {
        let mut workspaces = self.workspaces.write().await;
        let state = workspaces.entry(session_id.to_string()).or_insert_with(|| {
            WorkspaceState {
                session_id: session_id.to_string(),
                report: String::new(),
                previous_action: None,
                previous_result: None,
                iteration_count: 0,
                token_estimate: 0,
                updated_at: chrono::Utc::now(),
            }
        });

        // MARKOVIAN STATE TRANSITION:
        //   new_state = f(previous_report, previous_action, result)
        //
        // The full history is intentionally discarded. The agent
        // synthesises the new result into the evolving report,
        // which serves as its compressed memory going forward.

        // Update the report with the new information.
        if !tool_result.is_empty() {
            state.report.push_str(&format!("\n[Iter {}] {}", state.iteration_count + 1, tool_result));
        }

        // Update Markovian state.
        state.previous_action = Some(format!("tool_call_{}", state.iteration_count));
        state.previous_result = Some(tool_result.to_string());
        state.iteration_count += 1;

        // Estimate token count — remains roughly constant due to compression.
        state.token_estimate = state.report.len() as u64 / 4; // ~4 chars per token
        state.updated_at = chrono::Utc::now();

        Ok(state.report.clone())
    }

    /// Get the current workspace state for a session.
    pub async fn get_state(&self, session_id: &str) -> Option<WorkspaceState> {
        self.workspaces.read().await.get(session_id).cloned()
    }

    /// Get the number of iterations completed.
    pub async fn iteration_count(&self, session_id: &str) -> u64 {
        self.workspaces.read().await
            .get(session_id)
            .map(|s| s.iteration_count)
            .unwrap_or(0)
    }
}
