use serde::{Deserialize, Serialize};

/// UX Middleware — Cognitive Split solution.
///
/// Based on the article "It's 2026. Why Does Your AI Product Still
/// Look Like a Chatbot?" (Apr 2026): when an LLM is forced to
/// handle both business logic and UI layout simultaneously, the
/// result is "Context Pollution" — the model degrades because it
/// is simultaneously acting as "The Mathematician" (business logic)
/// and "The Painter" (UI layout). The consequence: "You get a
/// painting, but the calculation behind it becomes shallow or
/// hallucinated."
///
/// Solution: separation into two layers.
///   1. Intent Layer (LLM): agent reasons about what the user
///      needs and outputs structured data + an action‑object tag.
///   2. Render Layer (Middleware): deterministic engine maps the
///      tag to pre‑configured components from the GenUI catalog.
///      No LLM involvement in component selection.
///
/// Adding a new button is a configuration change, not an AI
/// alignment task. (Dashy Enterprise demo, May 5, 2026.)
pub struct UXMiddleware;

/// Structured output from the LLM (Intent Layer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIntent {
    /// The action‑object tag (e.g., "view · zone", "compare · period").
    pub action_object: String,
    /// Structured data the agent wants to display.
    pub data: serde_json::Value,
    /// Suggested component chain preference (optional).
    pub component_hint: Option<String>,
}

/// The resolved UI component from the Render Layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderDecision {
    pub action_object: String,
    pub selected_component: String,
    pub a2ui_spec: serde_json::Value,
    pub confidence: f64,
}

impl UXMiddleware {
    pub fn new() -> Self { Self {} }

    /// Render Layer: resolve an agent intent into a UI component.
    /// This is a deterministic mapping — no LLM, no embedding search.
    pub fn resolve(&self, intent: &AgentIntent) -> RenderDecision {
        // Deterministic lookup in the action‑object matrix.
        let component = match intent.action_object.as_str() {
            "view · zone" => "BarChart",
            "compare · period" => "LineChart",
            "compare · employee" | "compare · region" => "BarChart",
            "view · record" => "DataTable",
            "create · record" => "Form",
            "alert · threshold" => "NotificationCard",
            "summarise · meeting" => "NarrativeText",
            _ => {
                // Fallback: if data is array with > 1 element, use DataTable.
                if intent.data.as_array().map(|a| a.len() > 1).unwrap_or(false) {
                    "DataTable"
                } else {
                    "KpiCard"
                }
            }
        };

        let a2ui_spec = serde_json::json!({
            "surface_id": uuid::Uuid::new_v4().to_string(),
            "components": [{
                "id": "resolved",
                "component_type": component,
                "properties": {
                    "data": intent.data,
                }
            }]
        });

        RenderDecision {
            action_object: intent.action_object.clone(),
            selected_component: component.to_string(),
            a2ui_spec,
            confidence: 1.0, // deterministic
        }
    }
}
