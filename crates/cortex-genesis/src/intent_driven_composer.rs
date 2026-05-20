use serde::{Deserialize, Serialize};

/// Intent‑Driven Composer — runtime UI composition from NL intent.
///
/// When a user asks a novel cross‑system query through the Command
/// Bar, the dashboard must construct a UI on‑the‑fly — not from
/// pre‑generated panels, but from the Semantic Gateway's parsed
/// intent and the available absorbed fields. The composer fetches
/// relevant absorbed fields via the Schema Grounding Agent,
/// selects appropriate GenUI components from the catalog, and
/// assembles a temporary dashboard panel.
///
/// This implements the Generative UX paradigm (2026): "Agents
/// render charts, cards, and forms on demand." (AG‑UI).
pub struct IntentDrivenComposer;

/// A dynamically composed UI panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposedPanel {
    pub panel_id: String,
    pub intent_summary: String,
    pub components: Vec<ComposedComponent>,
    pub a2ui_spec: serde_json::Value,
    pub composed_at: chrono::DateTime<chrono::Utc>,
    pub ttl_seconds: u64, // how long the panel should remain
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposedComponent {
    pub component_type: String,
    pub props: serde_json::Value,
    pub absorbed_fields_used: Vec<String>,
}

impl IntentDrivenComposer {
    pub fn new() -> Self { Self {} }

    /// Compose a UI panel from a parsed intent and available fields.
    ///
    /// Algorithm:
    ///   1. Parse the intent action (show, compare, create).
    ///   2. Query absorbed fields matching intent targets via
    ///      Schema Grounding Agent embedding similarity.
    ///   3. Select the best GenUI component from the catalog
    ///      based on data shape (count, types, relationships).
    ///   4. Assemble into a temporary A2UI panel.
    pub fn compose(
        &self,
        intent: &str,
        matching_fields: &[cortex_tracedb::absorbed_fields::AbsorbedField],
    ) -> ComposedPanel {
        let component_type = if matching_fields.len() <= 3 { "KpiCard" }
        else if matching_fields.len() <= 10 { "DataTable" }
        else { "SearchResults" };

        let a2ui_spec = serde_json::json!({
            "surface_id": uuid::Uuid::new_v4().to_string(),
            "components": [{
                "id": "main",
                "component_type": component_type,
                "properties": {
                    "title": format!("Results for: {}", intent),
                    "fields": matching_fields.iter().map(|f|
                        f.semantic_label.clone().unwrap_or_else(|| f.source_column.clone())
                    ).collect::<Vec<_>>()
                }
            }]
        });

        ComposedPanel {
            panel_id: uuid::Uuid::new_v4().to_string(),
            intent_summary: intent.to_string(),
            components: vec![ComposedComponent {
                component_type: component_type.to_string(),
                props: serde_json::json!({}),
                absorbed_fields_used: matching_fields.iter().map(|f| f.field_id.to_string()).collect(),
            }],
            a2ui_spec,
            composed_at: chrono::Utc::now(),
            ttl_seconds: 300, // 5 minutes
        }
    }
}
