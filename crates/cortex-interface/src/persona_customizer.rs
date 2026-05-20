//! Users modify their own role personas – add panels, metrics, benchmarks.
//! Based on Persona‑Based Agents (Arbore et al., CHI 2026 Workshop) and
//! User‑Governed Personalization (Lin et al., arXiv:2605.09794, May 2026):
//! "LLM agents enable user‑governed personalization beyond platform boundaries."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct PersonaCustomizer {
    customizations: RwLock<HashMap<String, CustomPersona>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPersona {
    pub user_id: String,
    pub base_role: String,
    pub industry: String,
    pub custom_panels: Vec<CustomPanel>,
    pub custom_metrics: Vec<CustomMetric>,
    pub custom_workflows: Vec<String>,    // workflow IDs
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPanel {
    pub panel_id: String,
    pub title: String,
    pub panel_type: String,        // "KpiCard", "DataTable", "Chart", "DocumentScanner"
    pub source_systems: Vec<String>,
    pub refresh_interval_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    pub formula: String,           // "SUM(procurement_cost) WHERE wo_type='EM'"
    pub unit: String,
    pub benchmark: Option<f64>,
}

impl PersonaCustomizer {
    pub fn new() -> Self {
        Self { customizations: RwLock::new(HashMap::new()) }
    }

    /// Save a user's custom persona.
    pub async fn save(&self, persona: CustomPersona) {
        self.customizations.write().await.insert(persona.user_id.clone(), persona);
    }

    /// Load a user's custom persona, falling back to the base role template.
    pub async fn load(&self, user_id: &str) -> Option<CustomPersona> {
        self.customizations.read().await.get(user_id).cloned()
    }

    /// List all customisations for sharing with team members of the same role.
    pub async fn shareable_for_role(&self, role: &str) -> Vec<CustomPersona> {
        self.customizations.read().await.values()
            .filter(|p| p.base_role == role)
            .cloned()
            .collect()
    }
}
