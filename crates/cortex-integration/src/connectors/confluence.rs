use crate::connector_registry::*;
pub struct ConfluenceConnector;
impl ConfluenceConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "confluence".into(), name: "Atlassian Confluence".into(), system_type: SystemType::Confluence, version: "cloud".into(),
            mcp_endpoint: Some("https://your-domain.atlassian.net/wiki/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "confluence_get_page".into(), description: "Get Confluence page".into(), input_schema: serde_json::json!({"page_id": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://auth.atlassian.com/oauth/token".into()), scopes: vec!["read:confluence-content.summary".into()] },
            rate_limits: RateLimits { rpm: 60, burst_size: 3 }, status: ConnectorStatus::Active,
        }
    }
}
