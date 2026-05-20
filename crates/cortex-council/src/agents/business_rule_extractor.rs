use crate::talent::Talent;

/// Business Rule Extractor — captures implicit business logic from legacy apps.
///
/// Gap closure (v11 review): "application modernisation fails because
/// recovering the knowledge buried inside legacy systems is hard".
/// Monitors validation failures, trigger cascades, and workflow exceptions
/// to build a catalogue of business rules stored in absorbed_fields.
pub struct BusinessRuleExtractor;

impl BusinessRuleExtractor {
    pub fn talent() -> Talent {
        let mut t = Talent::new("business_rule_extractor", "Business Rule Extractor",
            "Watches legacy validation errors and trigger cascades to extract implicit business rules");
        t.add_capability("validation_error_capture");
        t.add_capability("trigger_cascade_tracing");
        t.add_capability("constraint_discovery");
        t.add_capability("rule_cataloguing");
        t.add_boundary("Extracted rules are stored as metadata only; never execute against source without approval");
        t
    }

    /// Capture a validation error from a legacy application and extract the rule.
    pub fn capture_validation_error(
        field: &str,
        attempted_value: &str,
        error_message: &str,
    ) -> Option<BusinessRule> {
        Some(BusinessRule {
            id: uuid::Uuid::new_v4().to_string(),
            field: field.to_string(),
            rule_description: format!("Error '{}' when value='{}'", error_message, attempted_value),
            rule_type: RuleType::Validation,
            source_application: "unknown".into(),
            discovered_at: chrono::Utc::now(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BusinessRule {
    pub id: String,
    pub field: String,
    pub rule_description: String,
    pub rule_type: RuleType,
    pub source_application: String,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum RuleType {
    Validation,
    Trigger,
    Constraint,
    WorkflowException,
}
