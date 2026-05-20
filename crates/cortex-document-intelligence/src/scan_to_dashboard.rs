//! Converts extracted document data into A2UI dashboard widgets.
//! Maps scanned reports, compliance certificates, and equipment tags into
//! native Cortex panels using the 18‑component A2UI v0.9 Basic Catalog.

use serde::{Deserialize, Serialize};

pub struct ScanToDashboard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub widget_id: String,
    pub title: String,
    pub a2ui_spec: serde_json::Value,
    pub source_doc_id: String,
}

impl ScanToDashboard {
    pub fn new() -> Self { Self }

    /// Convert a compliance feedback result into an A2UI dashboard panel.
    /// The panel shows the compliance score, gaps, and recommendations in a
    /// Card component with nested Text and List children per the A2UI v0.9 spec.
    pub fn compliance_to_panel(
        &self,
        feedback: &super::feedback_generator::ComplianceFeedback,
    ) -> DashboardWidget {
        let a2ui_spec = serde_json::json!({
            "surfaceId": uuid::Uuid::new_v4().to_string(),
            "components": [
                {
                    "id": "compliance-card",
                    "component": {
                        "Card": {
                            "children": {
                                "explicitList": ["score-text", "findings-list", "recommendations-list"]
                            }
                        }
                    }
                },
                {
                    "id": "score-text",
                    "component": {
                        "Text": {
                            "text": { "literalString": feedback.summary }
                        }
                    }
                },
                {
                    "id": "findings-list",
                    "component": {
                        "List": {
                            "items": feedback.detailed_findings.iter().map(|f| {
                                serde_json::json!({ "Text": { "text": { "literalString": f } } })
                            }).collect::<Vec<_>>()
                        }
                    }
                }
            ]
        });

        DashboardWidget {
            widget_id: uuid::Uuid::new_v4().to_string(),
            title: "Compliance Scan Result".into(),
            a2ui_spec,
            source_doc_id: feedback.doc_id.clone(),
        }
    }
}
