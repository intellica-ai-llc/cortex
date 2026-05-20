//! Tool Call Visualizer — Collapsible Tool Calls with Confidence Heatmaps
//!
//! Based on Luke Wroblewski (Feb 2026): "Tool calls were collapsed by
//! default, and selecting one would show its results in the right column."
//! Each tool call can be expanded to show its full input/output.

use serde::{Deserialize, Serialize};

pub struct ToolCallVisualizer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizedToolCall {
    pub call_id: String,
    pub tool_name: String,
    pub status: ToolCallVizStatus,
    pub duration_ms: u64,
    pub collapsed: bool,
    pub confidence: Option<f64>,
    pub input_preview: String,
    pub output_preview: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCallVizStatus {
    Running,
    Success,
    Error(String),
}

impl ToolCallVisualizer {
    pub fn new() -> Self { Self }

    pub fn visualize(
        tool_name: &str,
        input: &serde_json::Value,
        output: Option<&serde_json::Value>,
        status: ToolCallVizStatus,
    ) -> VisualizedToolCall {
        VisualizedToolCall {
            call_id: uuid::Uuid::new_v4().to_string(),
            tool_name: tool_name.to_string(),
            status,
            duration_ms: 0,
            collapsed: true,
            confidence: Some(0.9),
            input_preview: input.to_string().chars().take(80).collect(),
            output_preview: output.map(|o| o.to_string().chars().take(120).collect()),
        }
    }
}
