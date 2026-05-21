pub mod embedding_router;
pub mod tool_registry;
pub mod intent_parser;
pub mod execution_planner;
pub mod mcp_server;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SemanticGateway {
    pub router: embedding_router::EmbeddingRouter,
    pub registry: Arc<RwLock<tool_registry::ToolRegistry>>,
    pub parser: intent_parser::IntentParser,
    pub planner: execution_planner::ExecutionPlanner,
}

impl SemanticGateway {
    pub fn new() -> Self {
        Self {
            router: embedding_router::EmbeddingRouter::new(),
            registry: Arc::new(RwLock::new(tool_registry::ToolRegistry::new())),
            parser: intent_parser::IntentParser::new(),
            planner: execution_planner::ExecutionPlanner::new(),
        }
    }

    pub async fn route_intent(&self, intent: &str) -> Result<execution_planner::ExecutionPlan, GatewayError> {
        let parsed = self.parser.parse(intent)?;
        let embedding = self.router.embed(intent);
        let candidates = self.registry.read().await.search(&embedding, 5, 0.3);
        if candidates.is_empty() {
            return Err(GatewayError::NoToolsFound(intent.to_string()));
        }
        Ok(self.planner.construct(&parsed, &candidates)?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("no tools found for intent: {0}")]
    NoToolsFound(String),
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("plan error: {0}")]
    PlanError(String),
}
