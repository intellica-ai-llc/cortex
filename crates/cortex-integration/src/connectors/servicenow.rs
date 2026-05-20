use crate::connector_registry::*;
pub struct ServiceNowConnector;
impl ServiceNowConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "servicenow".into(), name: "ServiceNow".into(), system_type: SystemType::ServiceNow, version: "Washington".into(),
            mcp_endpoint: Some("https://dev.service-now.com/api/now/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "sn_get_incident".into(), description: "Retrieve incident".into(), input_schema: serde_json::json!({"sys_id": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://dev.service-now.com/oauth_token.do".into()), scopes: vec!["snc_platform_rest_api_access".into()] },
            rate_limits: RateLimits { rpm: 150, burst_size: 10 }, status: ConnectorStatus::Active,
        }
    }
}
