use serde::{Deserialize, Serialize};

/// Structured Error Recovery Framework (SERF).
///
/// Based on Srinivasan (arXiv:2603.13417, March 2026):
/// "provides machine-readable failure semantics that enable
/// deterministic agent self-correction"[reference:12].
///
/// Five failure dimensions:
///   1. Server contracts   — tool schema mismatch, version drift
///   2. User context       — missing params, invalid input
///   3. Timeouts           — ATBA budget exhausted
///   4. Errors             — runtime failures, validation errors
///   5. Observability      — missing spans, incomplete traces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SERFEnvelope {
    pub error_id: String,
    pub error_type: SERFErrorType,
    pub severity: SERFSeverity,
    pub recoverable: bool,
    pub suggested_action: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SERFErrorType {
    ServerContractViolation,
    UserContextError,
    TimeoutExhausted,
    RuntimeFailure,
    ObservabilityGap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SERFSeverity {
    Fatal,
    Error,
    Warning,
    Info,
}

impl SERFEnvelope {
    /// Wrap a generic error into a machine-readable SERF envelope.
    pub fn wrap(error: &dyn std::error::Error, error_type: SERFErrorType) -> Self {
        Self {
            error_id: uuid::Uuid::new_v4().to_string(),
            error_type,
            severity: SERFSeverity::Error,
            recoverable: true,
            suggested_action: format!("Retry with adjusted parameters: {}", error),
            details: serde_json::json!({}),
        }
    }

    /// Create a timeout envelope.
    pub fn timeout(tool: &str, budget_ms: u64) -> Self {
        Self {
            error_id: uuid::Uuid::new_v4().to_string(),
            error_type: SERFErrorType::TimeoutExhausted,
            severity: SERFSeverity::Warning,
            recoverable: true,
            suggested_action: format!("Increase ATBA budget for tool '{}' or split into sub-queries", tool),
            details: serde_json::json!({"tool": tool, "budget_ms": budget_ms}),
        }
    }
}
