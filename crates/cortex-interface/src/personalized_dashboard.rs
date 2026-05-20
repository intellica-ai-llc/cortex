use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// A dashboard uniquely generated for a single user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub user_id: String,
    pub panels: Vec<DashboardPanel>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub adaptive_score: f64,   // how well the dashboard matches recent behaviour
}

/// A single panel on the dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardPanel {
    pub id: String,
    pub title: String,
    pub panel_type: PanelType,
    pub content: serde_json::Value, // layout, widgets, data
    pub position: (u32, u32),
    pub last_interacted: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanelType {
    KpiCard,
    Chart,
    Table,
    Narrative,
    CommandBar,
    NotificationFeed,
    WorkflowTrigger,
}

/// The engine that builds and evolves dashboards per user.
pub struct PersonalizedDashboard {
    /// Stored dashboards keyed by user ID.
    dashboards: RwLock<HashMap<String, Dashboard>>,
    /// Learned user preferences (panel types, colour schemes, density).
    preferences: RwLock<HashMap<String, UserPreferences>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_chart_type: ChartType,
    pub density: Density,
    pub notification_frequency: NotificationFrequency,
    pub auto_dismiss_panels: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType { Bar, Line, Pie, Table, Narrative }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Density { Compact, Comfortable, Spacious }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationFrequency { RealTime, Hourly, Daily, Never }

impl PersonalizedDashboard {
    pub fn new() -> Self {
        Self {
            dashboards: RwLock::new(HashMap::new()),
            preferences: RwLock::new(HashMap::new()),
        }
    }

    /// Render (or re‑render) a user’s dashboard.
    ///
    /// Algorithm (v3 enhanced):
    ///   1. Load industry intelligence template for the user’s industry.
    ///   2. Load role template within that industry.
    ///   3. Query HR system for direct reports, department, initiatives.
    ///   4. Generate initial panels: “What needs your attention”,
    ///      “Key metrics”, “Cross‑system insight”, “Command Bar”.
    ///   5. Over 30 days, adapt: remove unused panels, suggest new
    ///      panels from similar users, visualisation preferences.
    pub async fn render(&self, user_id: &str) -> Dashboard {
        let mut dashboards = self.dashboards.write().await;
        // Return existing if fresh enough, else regenerate.
        if let Some(dash) = dashboards.get(user_id) {
            if dash.generated_at > chrono::Utc::now() - chrono::Duration::minutes(15) {
                return dash.clone();
            }
        }

        // Build default panels for new user (production: loads industry templates).
        let panels = vec![
            DashboardPanel {
                id: "needs-attention".into(),
                title: "What Needs Your Attention".into(),
                panel_type: PanelType::NotificationFeed,
                content: serde_json::json!({"alerts": []}),
                position: (0, 0),
                last_interacted: chrono::Utc::now(),
            },
            DashboardPanel {
                id: "key-metrics".into(),
                title: "Key Metrics".into(),
                panel_type: PanelType::KpiCard,
                content: serde_json::json!({"metrics": []}),
                position: (1, 0),
                last_interacted: chrono::Utc::now(),
            },
            DashboardPanel {
                id: "cross-system-insight".into(),
                title: "Cross‑System Insight".into(),
                panel_type: PanelType::Narrative,
                content: serde_json::json!({"query": "", "result": ""}),
                position: (0, 1),
                last_interacted: chrono::Utc::now(),
            },
            DashboardPanel {
                id: "command-bar".into(),
                title: "Command Bar".into(),
                panel_type: PanelType::CommandBar,
                content: serde_json::json!({"placeholder": "Ask anything across all systems..."}),
                position: (0, 2),
                last_interacted: chrono::Utc::now(),
            },
        ];

        let dashboard = Dashboard {
            user_id: user_id.to_string(),
            panels,
            generated_at: chrono::Utc::now(),
            adaptive_score: 1.0,
        };
        dashboards.insert(user_id.to_string(), dashboard.clone());
        dashboard
    }

    /// Record an interaction to improve future dashboards.
    pub async fn record_interaction(&self, user_id: &str, panel_id: &str) {
        let mut dashboards = self.dashboards.write().await;
        if let Some(dash) = dashboards.get_mut(user_id) {
            if let Some(panel) = dash.panels.iter_mut().find(|p| p.id == panel_id) {
                panel.last_interacted = chrono::Utc::now();
            }
        }
    }

    /// Remove panels that haven’t been interacted with for 14 days.
    pub async fn prune_stale_panels(&self, user_id: &str) {
        let mut dashboards = self.dashboards.write().await;
        if let Some(dash) = dashboards.get_mut(user_id) {
            let cutoff = chrono::Utc::now() - chrono::Duration::days(14);
            dash.panels.retain(|p| p.last_interacted > cutoff);
        }
    }
}
