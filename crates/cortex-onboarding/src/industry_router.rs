use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Industry detection and template selection.
pub struct IndustryRouter {
    industries: HashMap<String, IndustryOnboardingProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryOnboardingProfile {
    pub industry: String,
    pub primary_system: String,        // "Maximo", "Temenos", "Epic"
    pub knowledge_snap_template: String,
    pub regulatory_calendar: String,
    pub recommended_first_query: String,
    pub typical_roles: Vec<String>,
}

impl IndustryRouter {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("energy_utilities".into(), IndustryOnboardingProfile {
            industry: "Energy & Utilities".into(), primary_system: "Maximo".into(),
            knowledge_snap_template: "energy_utilities".into(),
            regulatory_calendar: "energy_utilities".into(),
            recommended_first_query: "Show me open work orders across all facilities".into(),
            typical_roles: vec!["COO".into(), "Maintenance Manager".into(), "Compliance Officer".into()],
        });
        m.insert("banking".into(), IndustryOnboardingProfile {
            industry: "Banking".into(), primary_system: "Temenos".into(),
            knowledge_snap_template: "banking".into(),
            regulatory_calendar: "banking".into(),
            recommended_first_query: "Show capital adequacy ratio with peer benchmarks".into(),
            typical_roles: vec!["CFO".into(), "Risk Officer".into(), "Compliance Officer".into()],
        });
        Self { industries: m }
    }

    pub fn detect(&self, industry: &str) -> Option<IndustryOnboardingProfile> {
        self.industries.get(industry).cloned()
    }
}
