use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GenUI Component Catalog (v11, gap 7).
///
/// Implements the Dashy action‑object matrix: a fixed lookup table
/// that maps observed user behaviours (view · zone, compare · period,
/// create · record) to prioritised UI component chains. Embedded in
/// the system prompt so the LLM reads it at inference time — no
/// retrieval step needed.
///
/// This solves the “Cognitive Split” problem: the LLM outputs
/// structured data + an action‑object tag; the middleware maps the
/// tag to pre‑configured components from this catalog.
pub struct ComponentCatalog {
    /// The action‑object matrix.
    matrix: HashMap<String, Vec<ComponentChain>>,
}

/// A prioritised chain of UI components for a given action‑object pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentChain {
    pub components: Vec<CatalogComponent>,
    pub priority: u8, // 1 = primary, 2 = secondary, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogComponent {
    pub component_type: CatalogComponentType,
    pub default_props: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatalogComponentType {
    BarChart,
    LineChart,
    DataTable,
    KpiNumber,
    NarrativeText,
    Form,
    RecommendedActions,
    DrillDown,
    FilterBar,
    Timeline,
}

impl ComponentCatalog {
    pub fn new() -> Self {
        let mut matrix = HashMap::new();

        // populate the Dashy action‑object matrix
        matrix.insert("view · zone".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::BarChart, default_props: json!({"stacked": false}) },
                    CatalogComponent { component_type: CatalogComponentType::RecommendedActions, default_props: json!({}) },
                ],
                priority: 1,
            },
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::DataTable, default_props: json!({"sortable": true}) },
                ],
                priority: 2,
            },
        ]);

        matrix.insert("compare · period".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::LineChart, default_props: json!({"multi_series": true}) },
                    CatalogComponent { component_type: CatalogComponentType::DrillDown, default_props: json!({"drill_by": "month"}) },
                ],
                priority: 1,
            },
        ]);

        matrix.insert("compare · employee".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::BarChart, default_props: json!({"horizontal": true}) },
                    CatalogComponent { component_type: CatalogComponentType::DataTable, default_props: json!({"sortable": true}) },
                ],
                priority: 1,
            },
        ]);

        matrix.insert("create · record".into(), vec![
            ComponentChain {
                components: vec![
                    CatalogComponent { component_type: CatalogComponentType::Form, default_props: json!({"validation": "inline"}) },
                ],
                priority: 1,
            },
        ]);

        Self { matrix }
    }

    /// Look up component chains for a given action‑object tag.
    pub fn get_chains(&self, action_object: &str) -> Vec<&ComponentChain> {
        self.matrix.get(action_object).map(|v| v.iter().collect()).unwrap_or_default()
    }

    /// Serialise the entire matrix for inclusion in the system prompt.
    pub fn to_prompt_context(&self) -> String {
        let mut out = String::from("Available UI components for action-object pairs:\n");
        for (key, chains) in &self.matrix {
            out.push_str(&format!("  {}:\n", key));
            for chain in chains {
                let names: Vec<String> = chain.components.iter().map(|c| format!("{:?}", c.component_type)).collect();
                out.push_str(&format!("    - [{}] (priority {})\n", names.join(", "), chain.priority));
            }
        }
        out
    }
}
