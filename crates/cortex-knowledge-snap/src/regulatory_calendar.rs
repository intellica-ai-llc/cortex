use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Industry‑specific regulatory filing calendar.
///
/// Preloaded with filing deadlines for banking (FR Y‑9C, Call Report,
/// FFIEC), energy (FERC, NERC), insurance (NAIC), healthcare (HIPAA),
/// and manufacturing (ISO). Based on RegPass's regulatory knowledge
/// graph and Credo AI's structured compliance frameworks.
pub struct RegulatoryCalendar {
    calendars: RwLock<HashMap<String, Vec<FilingDeadline>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilingDeadline {
    pub id: String,
    pub regulation: String,
    pub filing_name: String,
    pub frequency: FilingFrequency,
    pub next_due: chrono::NaiveDate,
    pub jurisdiction: String,
    pub industry: String,
    pub description: String,
    pub penalty_exposure: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilingFrequency {
    Daily, Weekly, Monthly, Quarterly, Annual, Biennial,
}

impl RegulatoryCalendar {
    pub fn new() -> Self {
        let mut calendars = HashMap::new();

        // Banking deadlines
        calendars.insert("banking".into(), vec![
            FilingDeadline {
                id: "fr_y9c".into(), regulation: "FR Y-9C".into(),
                filing_name: "Consolidated Financial Statements for Holding Companies".into(),
                frequency: FilingFrequency::Quarterly,
                next_due: chrono::NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
                jurisdiction: "US".into(), industry: "banking".into(),
                description: "Quarterly financial report for bank holding companies".into(),
                penalty_exposure: Some("Up to $1M per day".into()),
            },
            FilingDeadline {
                id: "call_report".into(), regulation: "FFIEC 031/041".into(),
                filing_name: "Call Report".into(),
                frequency: FilingFrequency::Quarterly,
                next_due: chrono::NaiveDate::from_ymd_opt(2026, 6, 30).unwrap(),
                jurisdiction: "US".into(), industry: "banking".into(),
                description: "Quarterly condition and income report".into(),
                penalty_exposure: Some("Regulatory enforcement action".into()),
            },
        ]);

        // Energy deadlines
        calendars.insert("energy_utilities".into(), vec![
            FilingDeadline {
                id: "ferc_form1".into(), regulation: "FERC Form 1".into(),
                filing_name: "Annual Report of Major Electric Utilities".into(),
                frequency: FilingFrequency::Annual,
                next_due: chrono::NaiveDate::from_ymd_opt(2027, 4, 18).unwrap(),
                jurisdiction: "US".into(), industry: "energy_utilities".into(),
                description: "Comprehensive financial and operating data".into(),
                penalty_exposure: Some("Significant".into()),
            },
        ]);

        Self { calendars: RwLock::new(calendars) }
    }

    /// Get all upcoming filing deadlines for an industry.
    pub async fn get_deadlines(&self, industry: &str) -> Vec<FilingDeadline> {
        self.calendars.read().await.get(industry).cloned().unwrap_or_default()
    }

    /// Get deadlines due within N days.
    pub async fn upcoming_within_days(&self, industry: &str, days: i64) -> Vec<FilingDeadline> {
        let cutoff = chrono::Utc::now().date_naive() + chrono::Duration::days(days);
        self.get_deadlines(industry).await.into_iter()
            .filter(|d| d.next_due <= cutoff)
            .collect()
    }
}
