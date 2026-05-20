use super::*;

/// Orchestrates the full Peyrano semantic gateway pipeline:
/// Discover → Authorise → Execute → Prove.
pub struct SemanticGatewayPipeline {
    pub gateway: SemanticGateway,
}

impl SemanticGatewayPipeline {
    pub fn new() -> Self {
        Self { gateway: SemanticGateway::new() }
    }

    /// Full end-to-end intent routing with provenance capsule attachment.
    pub async fn handle_intent(
        &self,
        intent: &str,
        context: &GatewayContext,
    ) -> Result<execution_planner::ExecutionResult, GatewayError> {
        let plan = self.gateway.route_intent(intent, context).await?;
        // Future: execute plan, attach TraceCaps capsule, return result
        Err(GatewayError::PlanError("Execution not yet implemented".into()))
    }
}
