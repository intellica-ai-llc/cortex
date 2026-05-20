use serde::{Deserialize, Serialize};

/// Hybrid Rollback Handler – ensures seamless fallback to legacy
/// if an agent skill fails during the Replace phase.
///
/// Inspired by Strangler Fig pattern – the legacy system remains
/// available as a fallback, but Cortex is the first choice.
/// Captures user context (inputs, state) at each step and can
/// redirect to the exact legacy screen with pre‑populated data.
pub struct HybridRollbackHandler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackContext {
    pub user_id: String,
    pub legacy_application: String,
    pub workflow_step: usize,
    pub captured_inputs: serde_json::Value,
    pub deep_link: Option<String>,   // URL or screen ID to resume in legacy
}

impl HybridRollbackHandler {
    pub fn new() -> Self { Self }

    /// Store a rollback point before executing a Cortex skill step.
    pub fn create_rollback_point(
        user_id: &str,
        app: &str,
        step: usize,
        inputs: &serde_json::Value,
        deep_link: Option<&str>,
    ) -> RollbackContext {
        RollbackContext {
            user_id: user_id.to_string(),
            legacy_application: app.to_string(),
            workflow_step: step,
            captured_inputs: inputs.clone(),
            deep_link: deep_link.map(|s| s.to_string()),
        }
    }

    /// On skill failure, guide the user back to the legacy app.
    pub fn initiate_rollback(ctx: &RollbackContext) -> String {
        format!(
            "Cortex skill failed at step {}. Opening {} in legacy mode with your data pre‑filled.",
            ctx.workflow_step, ctx.legacy_application
        )
    }
}
