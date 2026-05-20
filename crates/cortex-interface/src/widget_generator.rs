use serde::{Deserialize, Serialize};

/// Generates charts, tables, and visualisations from natural‑language
/// queries. The user doesn’t build dashboards — they ask questions
/// and the dashboard builds itself.
///
/// Based on NOVAID/AGENTUI.AI widget generation and Dashy’s
/// action‑object matrix: maps observed behaviours to prioritized
/// UI component chains.
pub struct WidgetGenerator {
    // In production: renders components using AG‑UI / A2UI protocols.
}

/// A widget specification for the A2UI/AG‑UI renderer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSpec {
    pub widget_type: WidgetType,
    pub title: String,
    pub data_source: String,           // reference to absorbed fields / connector
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    BarChart,
    LineChart,
    PieChart,
    DataTable,
    KpiNumber,
    NarrativeText,
    Form,
    RecommendedActions,
    DrillDown,
}

impl WidgetGenerator {
    pub fn new() -> Self { Self {} }

    /// Generate a widget from structured data and an intent tag.
    /// The intent tag comes from the LLM (Cognitive Split solution:
    /// the LLM outputs structured data + tag; the renderer maps to
    /// pre‑configured components).
    pub fn generate(&self, intent_tag: &str, data: &serde_json::Value) -> WidgetSpec {
        // Map action‑object pairs to widget chains.
        // Example: "view · zone" → BarChart → RecommendedActions
        let widget_type = match intent_tag {
            "compare · period" => WidgetType::LineChart,
            "compare · employee" | "compare · region" => WidgetType::BarChart,
            "view · record" => WidgetType::DataTable,
            "create · record" => WidgetType::Form,
            _ => WidgetType::NarrativeText,
        };

        WidgetSpec {
            widget_type,
            title: "Auto‑generated".into(),
            data_source: "query".into(),
            config: serde_json::json!({"data": data}),
        }
    }
}
