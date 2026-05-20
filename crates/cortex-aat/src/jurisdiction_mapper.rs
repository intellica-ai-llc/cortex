use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maps regulatory jurisdictions to Cortex compliance controls.
pub struct JurisdictionMapper {
    map: HashMap<String, Vec<String>>,
}

impl JurisdictionMapper {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("EU_AI_Act".into(), vec!["AAT", "SCITT", "VAP_Gold"]);
        m.insert("NERC_CIP".into(), vec!["field_audit", "real_time_computation"]);
        Self { map: m }
    }

    pub fn controls_for(&self, jurisdiction: &str) -> Vec<String> {
        self.map.get(jurisdiction).cloned().unwrap_or_default()
    }
}
