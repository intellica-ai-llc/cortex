use crate::connector_registry::*;
pub struct SAPConnector;
impl SAPConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "sap-s4hana".into(), name: "SAP S/4HANA".into(), system_type: SystemType::SAP, version: "2023".into(),
            mcp_endpoint: Some("https://sap.internal/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "sap_get_business_partner".into(), description: "Retrieve business partner data".into(), input_schema: serde_json::json!({"id": "string"}), output_schema: None },
                ConnectorTool { name: "sap_create_purchase_order".into(), description: "Create a purchase order".into(), input_schema: serde_json::json!({"vendor": "string", "items":"array"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: None, token_url: Some("https://sap.internal/oauth/token".into()), scopes: vec!["api".into()] },
            rate_limits: RateLimits { rpm: 120, burst_size: 10 }, status: ConnectorStatus::Active,
        }
    }
}
