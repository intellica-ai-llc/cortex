use crate::connector_registry::*;
pub struct SlackConnector;
impl SlackConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "slack".into(), name: "Slack".into(), system_type: SystemType::Slack, version: "1.0".into(),
            mcp_endpoint: Some("https://slack.com/api/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "slack_post_message".into(), description: "Post a message to a channel".into(), input_schema: serde_json::json!({"channel": "string", "text": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://slack.com/api/oauth.v2.access".into()), scopes: vec!["chat:write".into(), "channels:read".into()] },
            rate_limits: RateLimits { rpm: 300, burst_size: 30 }, status: ConnectorStatus::Active,
        }
    }
}
