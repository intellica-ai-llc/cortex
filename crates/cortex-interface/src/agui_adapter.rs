use serde::{Deserialize, Serialize};

/// CopilotKit AG-UI protocol adapter (v4).
///
/// AG‑UI standardises the live, tool‑aware interaction stream
/// between an agent run and an application. When a client
/// speaks AG‑UI, this adapter converts Cortex Interface specs
/// into the AG‑UI event stream format.
pub struct AGUIAdapter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGUIEvent {
    pub event_type: String, // "text_message", "tool_call", "state_snapshot", "ui_update"
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AGUIAdapter {
    pub fn new() -> Self { Self {} }

    /// Convert an Interface Engine panel into an AG‑UI stream.
    pub fn convert_panel(&self, _panel: &serde_json::Value) -> Vec<AGUIEvent> {
        // Map DashboardPanel fields to AG‑UI components:
        // KpiCard → Chart, Table → DataGrid, CommandBar → ChatInput.
        vec![]
    }

    /// Parse an incoming AG‑UI user action.
    pub fn parse_action(&self, _event: &AGUIEvent) -> Option<String> {
        None
    }
}
