use async_trait::async_trait;
use super::super::connector::{Connector, ConnectorError, ConnectorTool};

pub struct SnowflakeConnector { client: Option<reqwest::Client> }

impl SnowflakeConnector {
    pub fn new() -> Self {
        let token = std::env::var("SNOWFLAKE_TOKEN").ok();
        let account = std::env::var("SNOWFLAKE_ACCOUNT").ok();
        let client = match (&token, &account) {
            (Some(_), Some(_)) => Some(reqwest::Client::new()),
            _ => None,
        };
        Self { client }
    }
    pub fn is_configured(&self) -> bool { self.client.is_some() }
}

#[async_trait]
impl Connector for SnowflakeConnector {
    fn name(&self) -> &str { "snowflake" }
    fn tools(&self) -> Vec<ConnectorTool> {
        vec![ConnectorTool { name: "snowflake_execute".into(), description: "Execute a SQL query against Snowflake".into(), input_schema: serde_json::json!({"type":"object","properties":{"query":{"type":"string"}},"required":["query"]}), output_schema: Some(serde_json::json!({"type":"array","items":{"type":"object"}})) }]
    }
    async fn execute(&self, tool_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ConnectorError> {
        let _client = self.client.as_ref().ok_or_else(|| ConnectorError::AuthFailed("SNOWFLAKE_TOKEN or SNOWFLAKE_ACCOUNT not set".into()))?;
        match tool_name {
            "snowflake_execute" => {
                let _query = params.get("query").and_then(|v| v.as_str()).ok_or_else(|| ConnectorError::ExecutionFailed("Missing 'query'".into()))?;
                Ok(serde_json::json!({"message":"Snowflake connector (MVP)","status":"ok"}))
            }
            _ => Err(ConnectorError::ToolNotFound(tool_name.to_string())),
        }
    }
}
