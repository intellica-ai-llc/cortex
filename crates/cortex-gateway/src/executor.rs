use crate::execution_planner::ExecutionPlan;
use cortex_integration::connector::{Connector, ConnectorRegistry, ConnectorError as CError};
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionResult {
    pub plan: ExecutionPlan,
    pub outputs: Vec<serde_json::Value>,
    pub errors: Vec<String>,
    pub total_duration_ms: u64,
}

pub struct ToolExecutor;

impl ToolExecutor {
    pub fn new() -> Self { Self }
    pub async fn execute(plan: ExecutionPlan, registry: &ConnectorRegistry) -> ExecutionResult {
        let start = std::time::Instant::now();
        let mut outputs = Vec::new();
        let mut errors = Vec::new();
        for step in &plan.steps {
            let result = tokio::time::timeout(
                Duration::from_millis(step.timeout_ms),
                Self::execute_step(step.tool_name.as_str(), &step.params, registry),
            ).await;
            match result {
                Ok(Ok(value)) => outputs.push(value),
                Ok(Err(e)) => errors.push(e.to_string()),
                Err(_) => errors.push(format!("Tool '{}' timed out after {}ms", step.tool_name, step.timeout_ms)),
            }
        }
        ExecutionResult { plan, outputs, errors, total_duration_ms: start.elapsed().as_millis() as u64 }
    }
    async fn execute_step(tool_name: &str, params: &serde_json::Value, registry: &ConnectorRegistry) -> Result<serde_json::Value, CError> {
        for name in registry.names() {
            if let Some(connector) = registry.get(name) {
                match connector.execute(tool_name, params).await {
                    Ok(v) => return Ok(v),
                    Err(CError::ToolNotFound(_)) => continue,
                    Err(e) => return Err(e),
                }
            }
        }
        Err(CError::ToolNotFound(tool_name.to_string()))
    }
}
