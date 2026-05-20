use crate::connector_registry::*;
pub struct GitHubConnector;
impl GitHubConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "github".into(), name: "GitHub Enterprise".into(), system_type: SystemType::GitHub, version: "3.15".into(),
            mcp_endpoint: Some("https://github.internal/api/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "github_get_pr".into(), description: "Get pull request".into(), input_schema: serde_json::json!({"owner": "string", "repo": "string", "pull_number": "integer"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://github.com/login/oauth/access_token".into()), scopes: vec!["repo".into()] },
            rate_limits: RateLimits { rpm: 5000, burst_size: 500 }, status: ConnectorStatus::Active,
        }
    }
}
