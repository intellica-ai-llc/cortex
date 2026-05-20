use crate::connector_registry::*;
pub struct Dynamics365Connector;
impl Dynamics365Connector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "dynamics365".into(), name: "Microsoft Dynamics 365".into(), system_type: SystemType::Dynamics365, version: "9.2".into(),
            mcp_endpoint: Some("https://org.crm.dynamics.com/api/data/v9.2/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "d365_get_contacts".into(), description: "Query contacts".into(), input_schema: serde_json::json!({"filter": "string"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: Some("common".into()), token_url: Some("https://login.microsoftonline.com/common/oauth2/token".into()), scopes: vec!["https://org.crm.dynamics.com/.default".into()] },
            rate_limits: RateLimits { rpm: 200, burst_size: 20 }, status: ConnectorStatus::Active,
        }
    }
}
