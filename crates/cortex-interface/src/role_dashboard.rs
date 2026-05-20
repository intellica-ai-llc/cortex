use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Role‑Adaptive, Industry‑Refined Evolving Dashboard (v3/v4).
///
/// Generates industry‑specific dashboard templates with preconfigured
/// KPIs and benchmarks, then adapts them per user over 30 days.
pub struct RoleAdaptiveDashboard {
    /// Pre‑loaded industry templates (from Knowledge Snap).
    templates: RwLock<HashMap<String, IndustryDashboardTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryDashboardTemplate {
    pub industry: String,
    pub roles: HashMap<String, RoleTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleTemplate {
    pub role: String,
    pub default_panels: Vec<super::personalized_dashboard::DashboardPanel>,
    pub recommended_metrics: Vec<String>,
    pub regulatory_alerts: bool,
}

impl RoleAdaptiveDashboard {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Banking CFO template
        let mut banking = IndustryDashboardTemplate {
            industry: "Banking".into(),
            roles: HashMap::new(),
        };
        banking.roles.insert("CFO".into(), RoleTemplate {
            role: "CFO".into(),
            default_panels: vec![],
            recommended_metrics: vec![
                "Capital Adequacy Ratio".into(),
                "Liquidity Coverage Ratio".into(),
                "Net Interest Margin".into(),
                "Loan Loss Provisions".into(),
            ],
            regulatory_alerts: true,
        });
        templates.insert("Banking".into(), banking);

        // Energy COO template
        let mut energy = IndustryDashboardTemplate {
            industry: "Energy & Utilities".into(),
            roles: HashMap::new(),
        };
        energy.roles.insert("COO".into(), RoleTemplate {
            role: "COO".into(),
            default_panels: vec![],
            recommended_metrics: vec![
                "Generation Availability".into(),
                "Forced Outage Rate".into(),
                "Heat Rate".into(),
                "Emissions Compliance".into(),
            ],
            regulatory_alerts: true,
        });
        templates.insert("Energy & Utilities".into(), energy);

        Self { templates: RwLock::new(templates) }
    }

    /// Get the recommended metrics for a role in an industry.
    pub async fn get_metrics(&self, industry: &str, role: &str) -> Vec<String> {
        let templates = self.templates.read().await;
        templates.get(industry)
            .and_then(|t| t.roles.get(role))
            .map(|r| r.recommended_metrics.clone())
            .unwrap_or_default()
    }
}
