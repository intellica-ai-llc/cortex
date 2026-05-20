use serde::{Deserialize, Serialize};

/// Field‑to‑Component Mapper — auto‑creates dashboard widgets
/// from absorbed field definitions.
///
/// Uses the GenUI Component Catalog (Dashy action‑object matrix,
/// v11 Interface Engine) to map semantic labels and field types
/// to native Cortex UI components expressed as A2UI JSON.
///
/// Based on Oracle/Google/CopilotKit alignment (Mar 2026):
/// "A2UI is a declarative specification for generative UI where
/// an agent emits JSON that describes UI surfaces. The frontend
/// renders those surfaces using native components."
pub struct FieldToComponentMapper;

/// The result of mapping a single absorbed field to a panel component.
#[derive(Debug, Clone)]
pub enum FieldComponentMapping {
    /// A single field mapped to a text input in a Form.
    FormField(FormFieldSpec),
    /// A field mapped to a column in a Table.
    TableColumn(TableColumnSpec),
    /// A field mapped to a metric on a KPI card.
    KpiMetric(KpiMetricSpec),
    /// A field mapped to a label on a Detail view.
    DetailField(DetailFieldSpec),
    /// Cannot be auto‑mapped — requires manual configuration.
    Unmapped { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFieldSpec {
    pub field_label: String,
    pub field_name: String,
    pub field_type: String,
    pub required: bool,
    pub placeholder: Option<String>,
    pub validation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumnSpec {
    pub header: String,
    pub accessor: String,   // column key in data
    pub sortable: bool,
    pub filterable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KpiMetricSpec {
    pub label: String,
    pub value_field: String,
    pub format: String,     // "number", "currency", "percentage"
    pub trend_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailFieldSpec {
    pub label: String,
    pub value_field: String,
    pub display_format: String,
}

impl FieldToComponentMapper {
    pub fn new() -> Self { Self {} }

    /// Map a single absorbed field to a panel component.
    pub fn map_field(
        &self,
        field: &cortex_tracedb::absorbed_fields::AbsorbedField,
    ) -> Option<super::GeneratedPanel> {
        let label = field.semantic_label.as_deref().unwrap_or(&field.source_column);
        let field_type = &field.field_type;

        // Heuristic mapping based on semantic label and data type.
        let (panel_type, a2ui_spec) = match (label.to_lowercase().as_str(), field_type.as_str()) {
            // Work order fields
            (l, _) if l.contains("work order") || l.contains("wonum") => {
                (super::PanelType::WorkOrderList, serde_json::json!({
                    "component": "DataTable",
                    "columns": [{"header": label, "accessor": "value"}],
                    "sortable": true
                }))
            }
            // Asset fields
            (l, _) if l.contains("asset") || l.contains("equipment") => {
                (super::PanelType::AssetDashboard, serde_json::json!({
                    "component": "Card",
                    "title": label,
                    "children": [{"component": "Text", "value": "{{value}}"}]
                }))
            }
            // Date / timestamp fields
            (_, "TIMESTAMPTZ") | (_, "DATE") | (_, "DATETIME") => {
                (super::PanelType::Table, serde_json::json!({
                    "component": "DataTable",
                    "columns": [{"header": label, "accessor": "value", "type": "datetime"}]
                }))
            }
            // Numeric fields — KPI card
            (_, "NUMERIC") | (_, "NUMBER") | (_, "INTEGER") | (_, "BIGINT") | (_, "FLOAT") => {
                (super::PanelType::KpiSummary, serde_json::json!({
                    "component": "KpiCard",
                    "label": label,
                    "format": "number"
                }))
            }
            // Default: display as a text field
            _ => {
                (super::PanelType::Table, serde_json::json!({
                    "component": "Text",
                    "value": format!("{{{{ {} }}}}", field.source_column)
                }))
            }
        };

        Some(super::GeneratedPanel {
            panel_id: uuid::Uuid::new_v4().to_string(),
            title: format!("{} - {}", field.source_table, label),
            panel_type,
            a2ui_spec,
            source_fields: vec![field.field_id.to_string()],
        })
    }

    /// Map a single field to a component mapping (internal use).
    pub fn map_field_to_panel(
        &self,
        field: &cortex_tracedb::absorbed_fields::AbsorbedField,
    ) -> Option<super::GeneratedPanel> {
        self.map_field(field)
    }
}
