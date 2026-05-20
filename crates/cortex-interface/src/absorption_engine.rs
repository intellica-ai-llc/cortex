use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Progressive Application Absorption Engine (v3/v8).
///
/// Tracks the 5‑phase lifecycle (Observe→Convert→Surface→Migrate→
/// Deprecate) and computes the Absorption Score per legacy application.
pub struct AbsorptionEngine {
    status: RwLock<HashMap<String, ApplicationAbsorption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationAbsorption {
    pub application: String,
    pub phase: AbsorptionPhase,
    pub workflows_observed: u64,
    pub workflows_converted: u64,
    pub workflows_surfaced: u64,
    pub workflows_migrated: u64,
    pub absorption_score: f64,  // 0.0 – 100.0
    pub projected_retirement: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AbsorptionPhase {
    Observe,    // Days 1–14
    Convert,    // Days 7–21
    Surface,    // Days 14–35
    Migrate,    // Days 30–60
    Deprecate,  // Months 3–6
}

impl AbsorptionEngine {
    pub fn new() -> Self {
        Self { status: RwLock::new(HashMap::new()) }
    }

    /// Register a legacy application for absorption tracking.
    pub async fn register(&self, app: &str) {
        self.status.write().await.insert(app.to_string(), ApplicationAbsorption {
            application: app.to_string(),
            phase: AbsorptionPhase::Observe,
            workflows_observed: 0,
            workflows_converted: 0,
            workflows_surfaced: 0,
            workflows_migrated: 0,
            absorption_score: 0.0,
            projected_retirement: None,
        });
    }

    /// Advance the absorption phase for an application.
    pub async fn advance_phase(&self, app: &str, new_phase: AbsorptionPhase) {
        if let Some(status) = self.status.write().await.get_mut(app) {
            status.phase = new_phase;
            status.absorption_score = match new_phase {
                AbsorptionPhase::Observe => 5.0,
                AbsorptionPhase::Convert => 20.0,
                AbsorptionPhase::Surface => 50.0,
                AbsorptionPhase::Migrate => 80.0,
                AbsorptionPhase::Deprecate => 95.0,
            };
        }
    }

    /// Get current absorption status.
    pub async fn get_status(&self, app: &str) -> Option<ApplicationAbsorption> {
        self.status.read().await.get(app).cloned()
    }
}
