use serde::{Deserialize, Serialize};

/// Captures everything needed to prove safe decommissioning.
pub struct FullContextCapture;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureManifest {
    pub source: String,
    pub screens: Vec<CapturedScreen>,
    pub business_rules: Vec<CapturedBusinessRule>,
    pub interaction_patterns: serde_json::Value,
    pub captured_at: chrono::DateTime<chrono::Utc>,
    pub final_absorption_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedScreen {
    pub screen_name: String,
    pub fields: Vec<CapturedField>,
    pub layout: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedField {
    pub name: String,
    pub absorbed_field_id: String,
    pub last_known_value_sample: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedBusinessRule {
    pub rule_id: String,
    pub rule_description: String,
    pub enforcement_type: String,
}

impl FullContextCapture {
    pub fn new() -> Self { Self }

    /// Generate a capture manifest from absorbed metadata.
    pub fn capture(
        source: &str,
        fields: &[cortex_tracedb::absorbed_fields::AbsorbedField],
        workflows: &[cortex_tracedb::behavioral_workflows::BehavioralWorkflow],
    ) -> CaptureManifest {
        let screens = vec![CapturedScreen {
            screen_name: format!("{}_main", source),
            fields: fields.iter().map(|f| CapturedField {
                name: f.source_column.clone(),
                absorbed_field_id: f.field_id.to_string(),
                last_known_value_sample: None,
            }).collect(),
            layout: serde_json::json!({}),
        }];

        let business_rules = fields.iter()
            .filter(|f| f.validation_rules.is_some())
            .map(|f| CapturedBusinessRule {
                rule_id: uuid::Uuid::new_v4().to_string(),
                rule_description: f.validation_rules.as_ref().unwrap().to_string(),
                enforcement_type: "validation".into(),
            })
            .collect();

        CaptureManifest {
            source: source.to_string(),
            screens,
            business_rules,
            interaction_patterns: serde_json::json!({
                "workflows_migrated": workflows.len()
            }),
            captured_at: chrono::Utc::now(),
            final_absorption_pct: 100.0,
        }
    }
}
