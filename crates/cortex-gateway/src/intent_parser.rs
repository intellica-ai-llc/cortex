use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
    pub action: String,       // "show" | "compare" | "create" | …
    pub targets: Vec<String>, // entities: "work order", "asset", …
}

pub struct IntentParser;

impl IntentParser {
    pub fn new() -> Self { Self }

    pub fn parse(&self, text: &str) -> Result<ParsedIntent, crate::GatewayError> {
        let lower = text.to_lowercase();
        let action = if lower.contains("compare") { "compare" }
        else if lower.contains("create") || lower.contains("add") { "create" }
        else if lower.contains("update") || lower.contains("change") { "update" }
        else if lower.contains("delete") || lower.contains("remove") { "delete" }
        else if lower.contains("alert") || lower.contains("notify") { "alert" }
        else { "show" };

        let known = [
            "work order", "asset", "employee", "revenue", "customer", "vendor",
            "invoice", "purchase order", "contract", "facility", "equipment",
            "maintenance", "inspection", "incident", "claim", "policy",
        ];
        let targets: Vec<String> = known.iter()
            .filter(|kw| lower.contains(*kw))
            .map(|s| s.to_string())
            .collect();

        Ok(ParsedIntent { action: action.to_string(), targets })
    }
}
