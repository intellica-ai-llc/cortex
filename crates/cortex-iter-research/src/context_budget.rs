use std::collections::HashMap;
use tokio::sync::RwLock;

/// Context Budget Manager — enforces 40K context ceiling.
///
/// IterResearch: "The workspace remains consistently at ~40K tokens
/// regardless of how many tool calls are executed. This is the key
/// architectural invariant that enables 2048+ tool interactions
/// without performance degradation."
///
/// Budget enforcement strategies:
///   1. Workspace pruning: remove stale/irrelevant sections
///   2. Hierarchical summarisation: compress older report sections
///   3. Budget overflow: if workspace exceeds limit, force compression
pub struct ContextBudgetManager {
    max_tokens: u64,
    budgets: RwLock<HashMap<String, u64>>,
}

impl ContextBudgetManager {
    pub fn new(max_tokens: u64) -> Self {
        Self { max_tokens, budgets: RwLock::new(HashMap::new()) }
    }

    /// Check whether a proposed addition fits within the context budget.
    /// Returns remaining budget after the addition.
    pub async fn check_budget(
        &self,
        session_id: &str,
        current_tokens: u64,
        proposed_addition_tokens: u64,
    ) -> BudgetDecision {
        let projected = current_tokens + proposed_addition_tokens;

        if projected <= self.max_tokens {
            BudgetDecision::WithinBudget { remaining: self.max_tokens - projected }
        } else if projected <= self.max_tokens * 12 / 10 {
            BudgetDecision::NeedsCompression {
                excess: projected - self.max_tokens,
                suggestion: "Compress older report sections via hierarchical summarisation".into(),
            }
        } else {
            BudgetDecision::OverBudget {
                excess: projected - self.max_tokens,
                action: "Prune lowest-relevance sections to free space".into(),
            }
        }
    }

    /// Record the current token usage for a session.
    pub async fn update_usage(&self, session_id: &str, tokens: u64) {
        self.budgets.write().await.insert(session_id.to_string(), tokens);
    }

    /// Get current token usage.
    pub async fn current_usage(&self, session_id: &str) -> u64 {
        self.budgets.read().await.get(session_id).copied().unwrap_or(0)
    }

    /// Get the maximum budget.
    pub fn max_tokens(&self) -> u64 { self.max_tokens }
}

#[derive(Debug, Clone)]
pub enum BudgetDecision {
    WithinBudget { remaining: u64 },
    NeedsCompression { excess: u64, suggestion: String },
    OverBudget { excess: u64, action: String },
}
