use crate::connector_registry::*;
pub struct JiraConnector;
impl JiraConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "jira".into(), name: "Atlassian Jira".into(), system_type: SystemType::Jira, version: "cloud".into(),
            mcp_endpoint: Some("https://your-domain.atlassian.net/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "jira_get_issue".into(), description: "Get issue by key".into(), input_schema: serde_json::json!({"issue_key": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://auth.atlassian.com/oauth/token".into()), scopes: vec!["read:jira-work".into(), "write:jira-work".into()] },
            rate_limits: RateLimits { rpm: 100, burst_size: 5 }, status: ConnectorStatus::Active,
        }
    }
}
