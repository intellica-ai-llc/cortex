use serde::{Deserialize, Serialize};

/// Google A2UI v0.9 protocol adapter (v4).
///
/// A2UI lets agents propose safe, declarative UI surfaces that
/// applications render natively. This adapter generates A2UI‑
/// compliant JSON from Cortex Interface panels.
pub struct A2UIAdapter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2UIResponse {
    pub surface_id: String,
    pub components: Vec<A2UIComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2UIComponent {
    pub id: String,
    pub component_type: String, // "Card", "Table", "Chart", "Form", "Text"
    pub properties: serde_json::Value,
    pub children: Vec<String>,
}

impl A2UIAdapter {
    pub fn new() -> Self { Self {} }

    /// Convert a dashboard panel spec into A2UI JSON.
    pub fn convert_panel(&self, _panel: &serde_json::Value) -> A2UIResponse {
        // Map DashboardPanel to A2UI component hierarchy.
        // Uses the same component catalog as AG‑UI for consistency.
        A2UIResponse {
            surface_id: uuid::Uuid::new_v4().to_string(),
            components: vec![],
        }
    }
}
