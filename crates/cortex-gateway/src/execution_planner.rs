use crate::intent_parser::ParsedIntent;
use crate::tool_registry::Tool;
use crate::GatewayError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan { pub steps: Vec<PlanStep> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub tool_name: String,
    pub params: serde_json::Value,
    pub timeout_ms: u64,
}

pub struct ExecutionPlanner { default_timeout_ms: u64 }

impl ExecutionPlanner {
    pub fn new() -> Self { Self { default_timeout_ms: 30_000 } }

    pub fn construct(
        &self,
        intent: &ParsedIntent,
        candidates: &[Tool],
    ) -> Result<ExecutionPlan, GatewayError> {
        let steps = candidates.iter().map(|t| PlanStep {
            tool_name: t.name.clone(),
            params: serde_json::json!({ "action": intent.action, "targets": intent.targets }),
            timeout_ms: self.default_timeout_ms,
        }).collect();
        Ok(ExecutionPlan { steps })
    }
}
