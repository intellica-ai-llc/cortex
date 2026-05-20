use serde::{Deserialize, Serialize};

/// Workflow‑to‑UI Converter — transforms observed behavioural
/// patterns into interactive Cortex panels.
///
/// Reads behavioural_workflows DAGs from TraceDB and converts
/// the observed sequences of user actions into native Cortex
/// panels. When a user previously navigated Maximo work‑order
/// screens in a specific sequence (e.g., Open Work Order →
/// Check Asset → Update Status), that sequence becomes a
/// native Cortex panel with the same fields and flow.
pub struct WorkflowToUIConverter;

/// A workflow pattern translated into UI panels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowUIPanel {
    pub workflow_id: String,
    pub source_application: String,
    pub panels: Vec<WorkflowStepPanel>,
    pub estimated_time_saved_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStepPanel {
    pub step_order: usize,
    pub step_token: String,          // original behavioural token
    pub panel_type: String,          // "Form", "Table", "Detail", "Confirmation"
    pub fields_involved: Vec<String>,
    pub a2ui_spec: serde_json::Value,
}

impl WorkflowToUIConverter {
    pub fn new() -> Self { Self {} }

    /// Convert a behavioural workflow into a UI panel sequence.
    ///
    /// Token → Panel mapping:
    ///   MODIFY_Field → Form with input fields
    ///   SUBMIT_Form → Confirmation panel
    ///   QUERY_Database → DataTable with results
    ///   APPROVE_Workflow → Approval button
    pub fn convert(
        &self,
        workflow: &cortex_tracedb::behavioral_workflows::BehavioralWorkflow,
    ) -> WorkflowUIPanel {
        let panels: Vec<WorkflowStepPanel> = workflow
            .behavioral_tokens
            .iter()
            .enumerate()
            .map(|(i, token)| {
                let panel_type = match token.as_str() {
                    "MODIFY_Field" => "Form",
                    "SUBMIT_Form" => "Confirmation",
                    "QUERY_Database" => "DataTable",
                    "APPROVE_Workflow" => "ApprovalButton",
                    _ => "Info",
                };
                WorkflowStepPanel {
                    step_order: i,
                    step_token: token.clone(),
                    panel_type: panel_type.to_string(),
                    fields_involved: vec![],
                    a2ui_spec: serde_json::json!({
                        "component": panel_type,
                        "step": i,
                        "token": token,
                    }),
                }
            })
            .collect();

        // Estimate time saved: each step in legacy takes ~60s;
        // in Cortex with auto‑populated fields, ~5s per step.
        let legacy_time = panels.len() as u64 * 60_000;
        let cortex_time = panels.len() as u64 * 5_000;
        let time_saved = legacy_time.saturating_sub(cortex_time);

        WorkflowUIPanel {
            workflow_id: workflow.workflow_id.to_string(),
            source_application: workflow.source_application.clone(),
            panels,
            estimated_time_saved_ms: time_saved,
        }
    }
}
