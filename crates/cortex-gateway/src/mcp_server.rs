use crate::GatewayContext;
use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Native MCP server (Streamable HTTP + SSE).
pub struct McpServer {
    gateway: Arc<crate::SemanticGateway>,
}

impl McpServer {
    pub fn new(gateway: Arc<crate::SemanticGateway>) -> Self {
        Self { gateway }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/mcp", post(Self::handle_mcp))
            .with_state(Arc::new(self.gateway))
    }

    async fn handle_mcp(
        State(gateway): State<Arc<crate::SemanticGateway>>,
        Json(req): Json<McpRequest>,
    ) -> Json<McpResponse> {
        // In production, parse the request, route to tools, and return.
        Json(McpResponse {
            result: serde_json::json!({}),
            error: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub intent: Option<String>,
    pub tool: Option<String>,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct McpResponse {
    pub result: serde_json::Value,
    pub error: Option<String>,
}
