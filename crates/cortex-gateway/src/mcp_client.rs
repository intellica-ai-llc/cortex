use crate::tool_registry::Tool;
use serde_json::Value;
use std::time::Duration;

/// MCP client for connecting to external MCP servers.
pub struct McpClient {
    pub endpoint: String,
    pub timeout: Duration,
}

impl McpClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            timeout: Duration::from_secs(30),
        }
    }

    pub async fn call_tool(&self, _tool: &Tool, _params: Value) -> Result<Value, String> {
        // Placeholder: HTTP POST to the MCP server's tool endpoint.
        Err("Not implemented".into())
    }
}
