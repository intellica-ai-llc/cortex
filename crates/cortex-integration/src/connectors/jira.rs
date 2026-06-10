use async_trait::async_trait;
use super::super::connector::{Connector, ConnectorError, ConnectorTool};

pub struct JiraConnector { client: Option<reqwest::Client> }

impl JiraConnector {
    pub fn new() -> Self {
        let token = std::env::var("JIRA_TOKEN").ok();
        let domain = std::env::var("JIRA_DOMAIN").ok();
        let client = match (&token, &domain) {
            (Some(_), Some(_)) => {
                let mut headers = reqwest::header::HeaderMap::new();
                if let Ok(val) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token.as_ref().unwrap())) {
                    headers.insert(reqwest::header::AUTHORIZATION, val);
                }
                Some(reqwest::Client::builder().default_headers(headers).build().unwrap_or_else(|_| reqwest::Client::new()))
            }
            _ => None,
        };
        Self { client }
    }
    pub fn is_configured(&self) -> bool { self.client.is_some() }
}

#[async_trait]
impl Connector for JiraConnector {
    fn name(&self) -> &str { "jira" }
    fn tools(&self) -> Vec<ConnectorTool> {
        vec![ConnectorTool { name: "jira_get_issues".into(), description: "Get Jira issues by project".into(), input_schema: serde_json::json!({"type":"object","properties":{"project":{"type":"string"}}}), output_schema: Some(serde_json::json!({"type":"array","items":{"type":"object"}})) }]
    }
    async fn execute(&self, tool_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ConnectorError> {
        let _client = self.client.as_ref().ok_or_else(|| ConnectorError::AuthFailed("JIRA_TOKEN or JIRA_DOMAIN not set".into()))?;
        match tool_name {
            "jira_get_issues" => {
                let _project = params.get("project").and_then(|v| v.as_str()).unwrap_or("MAINT");
                Ok(serde_json::json!({"message":"Jira connector (MVP)","status":"ok"}))
            }
            _ => Err(ConnectorError::ToolNotFound(tool_name.to_string())),
        }
    }
}
