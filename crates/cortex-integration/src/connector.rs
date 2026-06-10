use async_trait::async_trait;
use std::collections::HashMap;

/// A simplified tool descriptor returned by connectors.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectorTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: Option<serde_json::Value>,
}

/// The universal connector trait.
#[async_trait]
pub trait Connector: Send + Sync {
    fn name(&self) -> &str;
    fn tools(&self) -> Vec<ConnectorTool>;
    async fn execute(
        &self,
        tool_name: &str,
        params: &serde_json::Value,
    ) -> Result<serde_json::Value, ConnectorError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectorError {
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
}

/// Registry holding all connectors. Initialised once at startup.
pub struct ConnectorRegistry {
    connectors: HashMap<String, Box<dyn Connector + Send + Sync>>,
}

impl ConnectorRegistry {
    pub fn new() -> Self {
        Self { connectors: HashMap::new() }
    }
    pub fn register(&mut self, connector: Box<dyn Connector + Send + Sync>) {
        self.connectors.insert(connector.name().to_string(), connector);
    }
    pub fn get(&self, name: &str) -> Option<&(dyn Connector + Send + Sync)> {
        self.connectors.get(name).map(|c| c.as_ref())
    }
    pub fn names(&self) -> Vec<&String> {
        self.connectors.keys().collect()
    }
    pub fn len(&self) -> usize {
        self.connectors.len()
    }
}
