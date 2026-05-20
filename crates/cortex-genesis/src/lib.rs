//! Cortex Genesis — Self‑Building Dashboard Engine (v13).
//!
//! Generates native Cortex UI panels from absorbed fields using
//! the A2UI/AG‑UI dual protocol. Every user receives a personalised,
//! evolving dashboard that replaces the legacy applications they
//! use daily.
//!
//! Key subsystems:
//!   field_to_component_mapper — auto‑creates widgets from absorbed fields
//!   workflow_to_ui_converter   — behavioural patterns become native panels
//!   screen_reconstructor       — legacy‑screen fidelity preservation
//!   intent_driven_composer     — runtime UI composition from NL intent
//!   ux_middleware              — Cognitive Split solution (LLM → tag → render)
//!   schema_version_gate        — UI invalidation on DDL change

pub mod field_to_component_mapper;
pub mod workflow_to_ui_converter;
pub mod screen_reconstructor;
pub mod intent_driven_composer;
pub mod ux_middleware;
pub mod schema_version_gate;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top‑level Genesis orchestrator.
pub struct GenesisEngine {
    pub field_mapper: Arc<field_to_component_mapper::FieldToComponentMapper>,
    pub workflow_converter: Arc<workflow_to_ui_converter::WorkflowToUIConverter>,
    pub screen_reconstructor: Arc<screen_reconstructor::ScreenReconstructor>,
    pub intent_composer: Arc<intent_driven_composer::IntentDrivenComposer>,
    pub ux_middleware: Arc<ux_middleware::UXMiddleware>,
    pub version_gate: Arc<schema_version_gate::SchemaVersionGate>,
    /// Generated dashboards indexed by user_id.
    dashboards: RwLock<std::collections::HashMap<String, GeneratedDashboard>>,
}

/// A complete dashboard generated from absorbed fields.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedDashboard {
    pub user_id: String,
    pub source_application: String,
    pub panels: Vec<GeneratedPanel>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub schema_versions: std::collections::HashMap<String, i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneratedPanel {
    pub panel_id: String,
    pub title: String,
    pub panel_type: PanelType,
    pub a2ui_spec: serde_json::Value,  // A2UI‑compliant JSON
    pub source_fields: Vec<String>,     // absorbed field IDs
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PanelType {
    WorkOrderList,
    WorkOrderDetail,
    AssetDashboard,
    MaintenanceCalendar,
    KpiSummary,
    SearchResults,
    Form,
    Table,
}

impl GenesisEngine {
    pub fn new() -> Self {
        Self {
            field_mapper: Arc::new(field_to_component_mapper::FieldToComponentMapper::new()),
            workflow_converter: Arc::new(workflow_to_ui_converter::WorkflowToUIConverter::new()),
            screen_reconstructor: Arc::new(screen_reconstructor::ScreenReconstructor::new()),
            intent_composer: Arc::new(intent_driven_composer::IntentDrivenComposer::new()),
            ux_middleware: Arc::new(ux_middleware::UXMiddleware::new()),
            version_gate: Arc::new(schema_version_gate::SchemaVersionGate::new()),
            dashboards: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Generate a dashboard for a user from absorbed fields.
    pub async fn generate(
        &self,
        user_id: &str,
        source_application: &str,
        absorbed_fields: &[cortex_tracedb::absorbed_fields::AbsorbedField],
    ) -> GeneratedDashboard {
        let mut panels = Vec::new();

        for field in absorbed_fields {
            if field.absorption_status == "absorbed" || field.absorption_status == "genesis" {
                if let Some(panel) = self.field_mapper.map_field_to_panel(field) {
                    panels.push(panel);
                }
            }
        }

        let dashboard = GeneratedDashboard {
            user_id: user_id.to_string(),
            source_application: source_application.to_string(),
            panels,
            generated_at: chrono::Utc::now(),
            schema_versions: std::collections::HashMap::new(),
        };

        self.dashboards.write().await.insert(user_id.to_string(), dashboard.clone());
        dashboard
    }

    /// Get a previously generated dashboard.
    pub async fn get_dashboard(&self, user_id: &str) -> Option<GeneratedDashboard> {
        self.dashboards.read().await.get(user_id).cloned()
    }
}
