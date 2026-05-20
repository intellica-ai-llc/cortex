use crate::connector_registry::*;
pub struct SharePointConnector;
impl SharePointConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "sharepoint".into(), name: "SharePoint Online".into(), system_type: SystemType::SharePoint, version: "graph/v1.0".into(),
            mcp_endpoint: Some("https://graph.microsoft.com/v1.0/sites/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "sharepoint_get_file".into(), description: "Download file".into(), input_schema: serde_json::json!({"site_id": "string", "file_path": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: Some("common".into()), token_url: Some("https://login.microsoftonline.com/common/oauth2/token".into()), scopes: vec!["https://graph.microsoft.com/Sites.ReadWrite.All".into()] },
            rate_limits: RateLimits { rpm: 120, burst_size: 10 }, status: ConnectorStatus::Active,
        }
    }
}
