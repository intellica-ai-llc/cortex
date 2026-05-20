use crate::intent_parser::ParsedIntent;
use crate::tool_registry::Tool;
use crate::{GatewayContext, GatewayError};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Multi-step tool‑chain construction with ATBA timeouts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub steps: Vec<PlanStep>,
    pub total_budget_ms: u64,
    pub metadata: PlanMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub tool_id: String,
    pub tool_name: String,
    pub params: serde_json::Value,
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub depends_on: Vec<usize>, // index of prerequisite steps
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanMetadata {
    pub parsed_intent: ParsedIntent,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub estimated_tokens: u64,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub plan: ExecutionPlan,
    pub outputs: Vec<serde_json::Value>,
    pub total_duration_ms: u64,
}

pub struct ExecutionPlanner {
    default_timeout: Duration,
    max_concurrent: usize,
}

impl ExecutionPlanner {
    pub fn new() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            max_concurrent: 4,
        }
    }

    /// Build a plan from parsed intent and candidate tools.
    pub fn construct(
        &self,
        intent: &ParsedIntent,
        candidates: &[Tool],
        context: &GatewayContext,
    ) -> Result<ExecutionPlan, GatewayError> {
        if candidates.is_empty() {
            return Err(GatewayError::PlanError("No candidate tools".into()));
        }

        // Simple planner: one step per candidate tool,
        // ordered by relevance score (already sorted).
        let steps: Vec<PlanStep> = candidates
            .iter()
            .enumerate()
            .map(|(i, tool)| PlanStep {
                tool_id: tool.id.clone(),
                tool_name: tool.name.clone(),
                params: build_params(intent, tool),
                timeout_ms: self.default_timeout.as_millis() as u64,
                max_retries: 1,
                depends_on: if i > 0 { vec![i - 1] } else { vec![] },
            })
            .collect();

        let total_budget_ms = steps.iter().map(|s| s.timeout_ms).sum();

        Ok(ExecutionPlan {
            steps,
            total_budget_ms,
            metadata: PlanMetadata {
                parsed_intent: intent.clone(),
                created_at: chrono::Utc::now(),
                estimated_tokens: 0,
            },
        })
    }
}

fn build_params(intent: &ParsedIntent, tool: &Tool) -> serde_json::Value {
    // Merge intent fields into tool's input schema shape
    let mut params = serde_json::json!({});
    params["action"] = serde_json::Value::String(intent.action.clone());
    if !intent.targets.is_empty() {
        params["targets"] = serde_json::to_value(&intent.targets).unwrap();
    }
    if !intent.filters.is_empty() {
        params["filters"] = serde_json::to_value(&intent.filters).unwrap();
    }
    if let Some(agg) = &intent.aggregation {
        params["aggregation"] = serde_json::Value::String(agg.clone());
    }
    params
}
