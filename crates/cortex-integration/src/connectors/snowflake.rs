use crate::connector_registry::*;
pub struct SnowflakeConnector;
impl SnowflakeConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "snowflake".into(), name: "Snowflake".into(), system_type: SystemType::Snowflake, version: "1.0".into(),
            mcp_endpoint: Some("https://org.snowflakecomputing.com/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "snowflake_execute_query".into(), description: "Execute SQL query".into(), input_schema: serde_json::json!({"query": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://org.snowflakecomputing.com/oauth/token".into()), scopes: vec!["session:role:*".into()] },
            rate_limits: RateLimits { rpm: 100, burst_size: 5 }, status: ConnectorStatus::Active,
        }
    }
}
