use crate::connector_registry::*;
pub struct TeamsConnector;
impl TeamsConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "teams".into(), name: "Microsoft Teams".into(), system_type: SystemType::Teams, version: "graph/v1.0".into(),
            mcp_endpoint: Some("https://graph.microsoft.com/v1.0/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "teams_send_message".into(), description: "Send message to channel".into(), input_schema: serde_json::json!({"team_id": "string", "channel_id": "string", "content": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: Some("common".into()), token_url: Some("https://login.microsoftonline.com/common/oauth2/token".into()), scopes: vec!["https://graph.microsoft.com/.default".into()] },
            rate_limits: RateLimits { rpm: 180, burst_size: 15 }, status: ConnectorStatus::Active,
        }
    }
}
