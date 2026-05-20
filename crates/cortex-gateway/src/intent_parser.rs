use serde::{Deserialize, Serialize};

/// Structured representation of a natural-language intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
    /// The action verb extracted from the query: "show", "create", "update", "delete", "compare", "alert".
    pub action: String,

    /// Entities or systems targeted: ["employee", "work order", "revenue"].
    pub targets: Vec<String>,

    /// Conditions applied: [{field: "performance_score", op: "gt", value: "4"}].
    pub filters: Vec<IntentFilter>,

    /// Aggregation: "count", "sum", "avg", "min", "max".
    pub aggregation: Option<String>,

    /// Grouping field: "region", "department".
    pub group_by: Option<String>,

    /// Maximum number of results.
    pub limit: Option<usize>,

    /// Time range: "last 7 days", "Q3 2026".
    pub time_range: Option<String>,

    /// Raw original text for provenance.
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentFilter {
    pub field: String,
    pub operator: FilterOp,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOp {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    Contains,
    StartsWith,
}

pub struct IntentParser {
    // In production, this wraps an LLM call.
    // For now, we provide a deterministic rule-based parser.
}

impl IntentParser {
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a natural language string into a structured intent.
    /// Placeholder that performs basic keyword extraction.
    pub fn parse(&self, text: &str) -> Result<ParsedIntent, super::GatewayError> {
        let lower = text.to_lowercase();
        let action = if lower.contains("compare") {
            "compare"
        } else if lower.contains("create") || lower.contains("add") {
            "create"
        } else if lower.contains("update") || lower.contains("change") {
            "update"
        } else if lower.contains("delete") || lower.contains("remove") {
            "delete"
        } else if lower.contains("alert") || lower.contains("notify") {
            "alert"
        } else {
            "show"   // default
        };

        Ok(ParsedIntent {
            action: action.to_string(),
            targets: extract_targets(text),
            filters: vec![],
            aggregation: None,
            group_by: None,
            limit: Some(50),
            time_range: None,
            raw: text.to_string(),
        })
    }
}

/// Very basic target extraction: nouns that follow common enterprise patterns.
fn extract_targets(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();
    let known = [
        "employee", "work order", "asset", "revenue", "customer", "vendor",
        "invoice", "purchase order", "contract", "facility", "equipment",
        "maintenance", "inspection", "incident", "claim", "policy",
    ];

    known
        .iter()
        .filter(|kw| lower.contains(*kw))
        .map(|s| s.to_string())
        .collect()
}
