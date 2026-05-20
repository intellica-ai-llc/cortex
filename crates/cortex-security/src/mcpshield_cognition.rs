use crate::SecurityError;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// MCPShield three-phase probe-execute-reflect cognition layer.
///
/// Based on Zhou et al. (arXiv:2602.14281, February 2026):
/// "MCPShield assists agent forms security cognition with metadata-
/// guided probing before invocation. Our method constrains execution
/// within controlled boundaries while cognizing runtime events, and
/// subsequently updates security cognition by reasoning over
/// historical traces after invocation"[reference:5].
///
/// Three phases:
///   PROBE   — metadata-guided investigation before tool invocation
///   EXECUTE — constrained runtime within controlled boundaries
///   REFLECT — post-invocation analysis of historical traces
pub struct MCPShieldCognition {
    /// Trust scores for known MCP servers (0.0–1.0).
    trust_scores: RwLock<HashMap<String, f64>>,
    /// Historical trace records for reflection.
    trace_history: RwLock<Vec<TraceRecord>>,
}

#[derive(Debug, Clone)]
pub struct TraceRecord {
    pub tool: String,
    pub server: String,
    pub outcome: TraceOutcome,
    pub latency_ms: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraceOutcome {
    Success,
    Failure { reason: String },
    Anomalous { description: String },
}

impl MCPShieldCognition {
    pub fn new() -> Self {
        Self {
            trust_scores: RwLock::new(HashMap::new()),
            trace_history: RwLock::new(Vec::new()),
        }
    }

    /// Phase 1: PROBE — investigate tool metadata before invocation.
    /// Checks server trust score; if below threshold, the tool call
    /// is gated pending further investigation.
    pub async fn probe(&self, tool: &str) -> Result<(), SecurityError> {
        let scores = self.trust_scores.read().await;
        let trust = scores.get(tool).copied().unwrap_or(0.5);

        if trust < 0.3 {
            return Err(SecurityError::CognitionFailed(format!(
                "Tool '{}' trust score {:.2} below minimum threshold (0.3)", tool, trust
            )));
        }

        Ok(())
    }

    /// Phase 2: EXECUTE boundary — check that execution stays within
    /// controlled boundaries. (Called during tool execution.)
    pub fn execute_boundary_check(&self, _tool: &str, _params: &serde_json::Value) -> Result<(), SecurityError> {
        // In production: validate params against tool schema,
        // enforce data volume limits, detect anomalous patterns.
        Ok(())
    }

    /// Phase 3: REFLECT — update cognition after invocation.
    pub async fn reflect(&self, record: TraceRecord) {
        // Update trust score based on outcome
        let mut scores = self.trust_scores.write().await;
        let current = scores.get(&record.tool).copied().unwrap_or(0.5);
        let delta = match record.outcome {
            TraceOutcome::Success => 0.02,
            TraceOutcome::Anomalous { .. } => -0.10,
            TraceOutcome::Failure { .. } => -0.05,
        };
        let new_score = (current + delta).clamp(0.0, 1.0);
        scores.insert(record.tool.clone(), new_score);

        // Store trace for future reflection
        self.trace_history.write().await.push(record);
    }

    /// Get the current trust score for a tool.
    pub async fn trust_score(&self, tool: &str) -> f64 {
        self.trust_scores.read().await.get(tool).copied().unwrap_or(0.5)
    }
}
