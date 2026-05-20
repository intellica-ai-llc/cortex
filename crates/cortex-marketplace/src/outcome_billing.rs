use serde::{Deserialize, Serialize};

/// Consumption-based billing for research reports, regulatory
/// filings, and competitive intelligence briefs.
pub struct OutcomeBillingEngine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillableOutcome {
    pub outcome_id: String,
    pub customer: String,
    pub outcome_type: OutcomeType,
    pub quantity: u64,
    pub unit_price_usd: f64,
    pub total_usd: f64,
    pub billed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutcomeType {
    ResearchReport,
    RegulatoryFiling,
    CompetitiveBrief,
    CustomSkillExecution,
}

impl OutcomeBillingEngine {
    pub fn new() -> Self { Self }
    pub fn bill(&self, customer: &str, outcome_type: OutcomeType, quantity: u64) -> BillableOutcome {
        let unit_price = match outcome_type {
            OutcomeType::ResearchReport => 9.99,
            OutcomeType::RegulatoryFiling => 49.99,
            OutcomeType::CompetitiveBrief => 19.99,
            OutcomeType::CustomSkillExecution => 0.10,
        };
        BillableOutcome {
            outcome_id: uuid::Uuid::new_v4().to_string(),
            customer: customer.to_string(),
            outcome_type,
            quantity,
            unit_price_usd: unit_price,
            total_usd: unit_price * quantity as f64,
            billed_at: chrono::Utc::now(),
        }
    }
}
