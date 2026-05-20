use crate::connector_registry::*;
pub struct NetSuiteConnector;
impl NetSuiteConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "netsuite".into(), name: "NetSuite".into(), system_type: SystemType::NetSuite, version: "2024.2".into(),
            mcp_endpoint: Some("https://netsuite.internal/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "ns_get_customer".into(), description: "Get customer record".into(), input_schema: serde_json::json!({"internalid": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://netsuite.internal/oauth/token".into()), scopes: vec!["rest_webservices".into()] },
            rate_limits: RateLimits { rpm: 100, burst_size: 5 }, status: ConnectorStatus::Active,
        }
    }
}
