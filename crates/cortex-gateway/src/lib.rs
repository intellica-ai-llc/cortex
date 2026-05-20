//! Cortex Semantic Gateway – the MCP control plane.
//!
//! Based on Peyrano architecture (arXiv:2604.25555).
//! Dynamically discovers, authorises, and executes enterprise tools.

pub mod semantic_gateway;
pub mod embedding_router;
pub mod tool_registry;
pub mod intent_parser;
pub mod execution_planner;
pub mod cross_system_bar;
pub mod connector_auto_discovery;
pub mod code_mode_cache;
pub mod tool_versioning;
pub mod mcp_server;
pub mod mcp_client;
pub mod a2a_bridge;
pub mod transport;
pub mod sessions;

use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// The core Semantic Gateway composite.
pub struct SemanticGateway {
    pub router: embedding_router::EmbeddingRouter,
    pub registry: Arc<tool_registry::ToolRegistry>,
    pub parser: intent_parser::IntentParser,
    pub planner: execution_planner::ExecutionPlanner,
}

impl SemanticGateway {
    pub fn new() -> Self {
        Self {
            router: embedding_router::EmbeddingRouter::new(),
            registry: Arc::new(tool_registry::ToolRegistry::new()),
            parser: intent_parser::IntentParser::new(),
            planner: execution_planner::ExecutionPlanner::new(),
        }
    }

    /// Primary entry point: route a natural-language intent to an execution plan.
    pub async fn route_intent(
        &self,
        intent: &str,
        context: &GatewayContext,
    ) -> Result<execution_planner::ExecutionPlan, GatewayError> {
        // 1. Parse intent into structured representation
        let parsed = self.parser.parse(intent)?;

        // 2. Embed the intent and find top-K matching tools
        let embedding = self.router.embed(intent);
        let candidates = self.registry.search(&embedding, 5, 0.3);

        if candidates.is_empty() {
            return Err(GatewayError::NoToolsFound(intent.to_string()));
        }

        // 3. Construct a multi-step execution plan
        let plan = self.planner.construct(&parsed, &candidates, context)?;

        Ok(plan)
    }
}

/// Shared context for gateway operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayContext {
    pub user_id: Option<String>,
    pub session_id: String,
    pub roles: Vec<String>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("No tools found for intent: {0}")]
    NoToolsFound(String),

    #[error("Intent parsing failed: {0}")]
    ParseError(String),

    #[error("Plan construction failed: {0}")]
    PlanError(String),

    #[error("Unauthorised: {0}")]
    Unauthorized(String),
}
