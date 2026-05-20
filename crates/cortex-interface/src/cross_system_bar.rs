use serde::{Deserialize, Serialize};

/// Interface‑specific cross‑system command bar.
///
/// This wraps the gateway’s CrossSystemCommandBar with UI‑level
/// result rendering, visualisation selection, and user history.
pub struct CrossSystemCommandBar {
    /// Recent queries for auto‑complete.
    history: tokio::sync::RwLock<Vec<QueryHistoryEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHistoryEntry {
    pub user_id: String,
    pub nl: String,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBarResult {
    pub nl: String,
    pub answer: String,
    pub visualization: Option<VisualizationSpec>,
    /// Which systems were queried and which fields accessed (audit trail).
    pub audit: serde_json::Value,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationSpec {
    pub chart_type: super::personalized_dashboard::ChartType,
    pub data: serde_json::Value,
    pub config: serde_json::Value,
}

impl CrossSystemCommandBar {
    pub fn new() -> Self {
        Self { history: tokio::sync::RwLock::new(Vec::new()) }
    }

    /// Execute a natural‑language query and return UI‑ready result.
    pub async fn execute(&self, nl: &str, user_id: &str) -> CommandBarResult {
        // In production: decompose via gateway, join sub‑results,
        // auto‑select visualisation based on data shape.
        let mut history = self.history.write().await;
        history.push(QueryHistoryEntry {
            user_id: user_id.to_string(),
            nl: nl.to_string(),
            executed_at: chrono::Utc::now(),
        });

        // Determine best visualisation (heuristic):
        //   - fewer than 10 rows → table
        //   - time series → line chart
        //   - categorical → bar chart
        //   - summary → narrative text
        CommandBarResult {
            nl: nl.to_string(),
            answer: "Query result placeholder".into(),
            visualization: None,
            audit: serde_json::json!({}),
            duration_ms: 0,
        }
    }
}
