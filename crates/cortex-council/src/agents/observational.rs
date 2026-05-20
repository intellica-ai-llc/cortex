use crate::talent::Talent;

/// Observational Agent — field-level user interaction capture.
///
/// Implements the PMAx pattern (arXiv:2603.15351): privacy-preserving
/// multi-agent architecture. Engineer agent analyses event-log metadata
/// and generates local scripts; Analyst agent interprets results.
/// Extends to browser extension, accessibility API, OCR, and terminal
/// emulation for non-web legacy apps.
pub struct ObservationalAgent;

impl ObservationalAgent {
    pub fn talent() -> Talent {
        let mut t = Talent::new("observational", "Observational Agent",
            "Watches users in legacy apps, records field-level interactions, absorbs workflows");
        t.add_capability("field_level_tracking");
        t.add_capability("browser_automation");
        t.add_capability("a11y_inspection");
        t.add_capability("ocr");
        t.add_capability("terminal_emulation");
        t.add_capability("rpa_integration");
        t.add_boundary("Never capture passwords or auth tokens; never record non-work applications");
        t
    }

    /// Capture a field-level interaction event.
    pub fn capture_interaction(
        user_id: &str,
        application: &str,
        field_path: &str,
        old_value: Option<&str>,
        new_value: Option<&str>,
    ) -> FieldInteraction {
        FieldInteraction {
            user_id: user_id.to_string(),
            application: application.to_string(),
            field_path: field_path.to_string(),
            old_value: old_value.map(|s| s.to_string()),
            new_value: new_value.map(|s| s.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldInteraction {
    pub user_id: String,
    pub application: String,
    pub field_path: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
