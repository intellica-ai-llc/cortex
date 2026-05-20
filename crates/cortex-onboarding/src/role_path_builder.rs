use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Generates role‑specific onboarding paths.
pub struct RolePathBuilder {
    paths: RwLock<HashMap<String, OnboardingPath>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingPath {
    pub role: String,
    pub industry: String,
    pub phases: Vec<OnboardingPhase>,
    pub estimated_days_to_proficiency: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingPhase {
    pub day: u32,
    pub title: String,
    pub actions: Vec<String>,
    pub dashboard_panel: String,
}

impl RolePathBuilder {
    pub fn new() -> Self { Self { paths: RwLock::new(HashMap::new()) } }

    pub async fn build(&self, role: &str, industry: &str) -> OnboardingPath {
        let phases = match (industry, role) {
            ("banking", "CFO") => vec![
                OnboardingPhase { day: 1, title: "Financial Overview".into(), actions: vec!["Cross‑system balance query".into()], dashboard_panel: "KPI Summary".into() },
                OnboardingPhase { day: 7, title: "Regulatory Calendar".into(), actions: vec!["Review upcoming filings".into()], dashboard_panel: "Regulatory Alerts".into() },
            ],
            _ => vec![
                OnboardingPhase { day: 1, title: "Welcome".into(), actions: vec!["Explore dashboard".into()], dashboard_panel: "Command Bar".into() },
            ],
        };
        OnboardingPath { role: role.into(), industry: industry.into(), phases, estimated_days_to_proficiency: 14 }
    }
}
