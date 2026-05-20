use crate::connector_registry::*;
pub struct SalesforceConnector;
impl SalesforceConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "salesforce".into(), name: "Salesforce".into(), system_type: SystemType::Salesforce, version: "59.0".into(),
            mcp_endpoint: Some("https://mycompany.my.salesforce.com/services/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "sf_query_accounts".into(), description: "Query accounts".into(), input_schema: serde_json::json!({"soql": "string"}), output_schema: None },
                ConnectorTool { name: "sf_create_opportunity".into(), description: "Create new opportunity".into(), input_schema: serde_json::json!({"name": "string", "close_date": "date"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://login.salesforce.com/services/oauth2/token".into()), scopes: vec!["api".into(), "refresh_token".into()] },
            rate_limits: RateLimits { rpm: 200, burst_size: 20 }, status: ConnectorStatus::Active,
        }
    }
}
