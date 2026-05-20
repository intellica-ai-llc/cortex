use crate::semantic_gateway::SemanticGatewayPipeline;
use crate::GatewayContext;
use serde::{Deserialize, Serialize};

/// Single NL interface for multi‑system queries.
pub struct CrossSystemCommandBar {
    pipeline: SemanticGatewayPipeline,
}

impl CrossSystemCommandBar {
    pub fn new(pipeline: SemanticGatewayPipeline) -> Self {
        Self { pipeline }
    }

    /// Execute a natural‑language query spanning multiple connected systems.
    pub async fn execute(
        &self,
        nl: &str,
        context: &GatewayContext,
    ) -> Result<CrossSystemResult, crate::GatewayError> {
        // Decomposition would happen here; for now, route as single intent.
        let plan = self.pipeline.gateway.route_intent(nl, context).await?;
        // In production, execute the plan and collect results.
        Ok(CrossSystemResult {
            summary: format!("Plan created with {} steps", plan.steps.len()),
            plan,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CrossSystemResult {
    pub summary: String,
    pub plan: crate::execution_planner::ExecutionPlan,
}
