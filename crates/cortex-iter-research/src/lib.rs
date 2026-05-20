//! Cortex IterResearch™ — Context-Efficient Research Engine (v6).
//!
//! Based on IterResearch (Renmin/Qwen, ICLR 2026): Markovian workspace
//! reconstruction. The agent maintains a dynamic, evolving report as its
//! memory, reconstructing only what's needed at each step rather than
//! carrying the full history. This enables 2,048+ tool calls with only
//! 40K context and performance improving from 3.5% to 42.5% on BrowseComp.
//!
//! Key insight (from the IterResearch paper): "The report draft itself
//! serves as the agent's memory. Each iteration reads the current draft,
//! executes a tool call to gather more information, updates the draft,
//! and discards the tool response from context — keeping the context
//! window at ~40K tokens regardless of how many tool calls are executed."

pub mod markovian_workspace;
pub mod context_budget;
pub mod tool_call_scaler;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct IterResearchEngine {
    pub workspace: Arc<markovian_workspace::MarkovianWorkspace>,
    pub budget: Arc<context_budget::ContextBudgetManager>,
    pub scaler: Arc<tool_call_scaler::ToolCallScaler>,
    /// Active research sessions.
    sessions: RwLock<std::collections::HashMap<String, ResearchSession>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResearchSession {
    pub session_id: String,
    pub question: String,
    pub tool_calls_executed: u64,
    pub context_used_tokens: u64,
    pub current_report: String,
    pub status: SessionStatus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SessionStatus { Active, Complete, ContextExhausted }

impl IterResearchEngine {
    pub fn new() -> Self {
        Self {
            workspace: Arc::new(markovian_workspace::MarkovianWorkspace::new()),
            budget: Arc::new(context_budget::ContextBudgetManager::new(40_000)),
            scaler: Arc::new(tool_call_scaler::ToolCallScaler::new()),
            sessions: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Execute a single research iteration.
    pub async fn iterate(
        &self,
        session_id: &str,
        question: &str,
        tool_result: &str,
    ) -> Result<String, String> {
        self.workspace.iterate(session_id, question, tool_result).await
    }

    /// Get current context usage for a session.
    pub async fn context_usage(&self, session_id: &str) -> u64 {
        self.budget.current_usage(session_id).await
    }
}
