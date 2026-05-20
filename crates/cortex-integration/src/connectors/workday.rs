use crate::connector_registry::*;
pub struct WorkdayConnector;
impl WorkdayConnector {
    pub fn default() -> ConnectorDefinition {
        ConnectorDefinition {
            id: "workday".into(), name: "Workday".into(), system_type: SystemType::Workday, version: "v38.2".into(),
            mcp_endpoint: Some("https://wd3-impl-services1.workday.com/ccx/service/customreport2/mcp".into()), openapi_spec_url: None,
            tools: vec![
                ConnectorTool { name: "wd_get_worker".into(), description: "Get worker by ID".into(), input_schema: serde_json::json!({"worker_id": "string"}), output_schema: None },
                ConnectorTool { name: "wd_submit_time_off".into(), description: "Submit time off request".into(), input_schema: serde_json::json!({"worker_id": "string", "dates": "array"}), output_schema: None },
            ],
            authentication: AuthConfig { method: AuthMethod::OAuth2, client_id: None, tenant_id: Some("mytenant".into()), token_url: Some("https://wd3-impl-services1.workday.com/ccx/oauth2/token".into()), scopes: vec!["system".into()] },
            rate_limits: RateLimits { rpm: 60, burst_size: 5 }, status: ConnectorStatus::Active,
        }
    }
}
