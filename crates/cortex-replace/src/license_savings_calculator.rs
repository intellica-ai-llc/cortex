use serde::{Deserialize, Serialize};

/// Computes financial ROI from progressive weaning.
pub struct LicenseSavingsCalculator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsReport {
    pub source: String,
    pub annual_license_cost: f64,
    pub absorption_pct: f64,
    pub estimated_annual_savings: f64,
    pub projected_retirement_date: Option<chrono::NaiveDate>,
    pub cumulative_savings_since_start: f64,
}

impl LicenseSavingsCalculator {
    pub fn new() -> Self { Self }

    pub fn calculate(
        source: &str,
        annual_cost: f64,
        absorption_pct: f64,
        months_active: u32,
    ) -> SavingsReport {
        let savings_to_date = (annual_cost / 12.0) * months_active as f64 * absorption_pct;
        SavingsReport {
            source: source.to_string(),
            annual_license_cost: annual_cost,
            absorption_pct,
            estimated_annual_savings: annual_cost * absorption_pct,
            projected_retirement_date: if absorption_pct >= 0.9 {
                Some(chrono::Utc::now().date_naive() + chrono::Duration::days(90))
            } else { None },
            cumulative_savings_since_start: savings_to_date,
        }
    }
}
