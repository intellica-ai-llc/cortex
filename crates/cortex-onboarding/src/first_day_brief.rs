use serde::{Deserialize, Serialize};

/// Day‑1 intelligence brief generation.
pub struct FirstDayBrief;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayOneBrief {
    pub user_id: String,
    pub greeting: String,
    pub industry_context: String,
    pub key_systems_connected: u32,
    pub regulatory_alerts: u32,
    pub first_query_suggestion: String,
}

impl FirstDayBrief {
    pub fn new() -> Self { Self }
    pub fn generate(&self, user_id: &str, role: &str, industry: &str) -> DayOneBrief {
        DayOneBrief {
            user_id: user_id.into(),
            greeting: format!("Good morning, {}. Welcome to Cortex.", role),
            industry_context: format!("Preloaded with {} intelligence and regulatory calendar.", industry),
            key_systems_connected: 8,
            regulatory_alerts: 3,
            first_query_suggestion: "Show me what needs my attention today".into(),
        }
    }
}
