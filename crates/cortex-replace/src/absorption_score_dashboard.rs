use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tracks absorption progress per source system.
pub struct AbsorptionScoreDashboard {
    scores: HashMap<String, AbsorptionScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsorptionScore {
    pub source: String,
    pub fields_total: u64,
    pub fields_absorbed: u64,
    pub workflows_total: u64,
    pub workflows_migrated: u64,
    pub license_cost_annual: f64,
    pub projected_retirement: Option<chrono::NaiveDate>,
    pub weaning_stage: WeaningStage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeaningStage {
    Observing,
    Mirroring,
    Absorbing,
    SurfacingSuggestions,
    Migrating,
    Deprecating,
}

impl AbsorptionScoreDashboard {
    pub fn new() -> Self { Self { scores: HashMap::new() } }

    pub fn update(&mut self, source: &str, fields_absorbed: u64, workflows_migrated: u64) {
        let entry = self.scores.entry(source.to_string()).or_insert_with(|| AbsorptionScore {
            source: source.to_string(),
            fields_total: 0,
            fields_absorbed: 0,
            workflows_total: 0,
            workflows_migrated: 0,
            license_cost_annual: 0.0,
            projected_retirement: None,
            weaning_stage: WeaningStage::Observing,
        });
        entry.fields_absorbed = fields_absorbed;
        entry.workflows_migrated = workflows_migrated;
        // Determine stage based on percentages
        let field_pct = if entry.fields_total > 0 { fields_absorbed as f64 / entry.fields_total as f64 } else { 0.0 };
        let wf_pct = if entry.workflows_total > 0 { workflows_migrated as f64 / entry.workflows_total as f64 } else { 0.0 };
        entry.weaning_stage = if field_pct >= 0.95 && wf_pct >= 0.95 {
            WeaningStage::Deprecating
        } else if field_pct >= 0.80 && wf_pct >= 0.80 {
            WeaningStage::Migrating
        } else if field_pct >= 0.50 {
            WeaningStage::SurfacingSuggestions
        } else if field_pct >= 0.20 {
            WeaningStage::Absorbing
        } else if field_pct > 0.0 {
            WeaningStage::Mirroring
        } else {
            WeaningStage::Observing
        };
    }

    pub fn get_score(&self, source: &str) -> Option<&AbsorptionScore> {
        self.scores.get(source)
    }
}
