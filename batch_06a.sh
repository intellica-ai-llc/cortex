#!/bin/bash
# ============================================================
# BATCH 6a: CORTEX INTERFACE ENGINE — THE INTERFACE OF ONE (Part 1)
# ============================================================
# Grounded in: NOVAID/AGENTUI.AI widget generation; Capgemini
# Zero UI; Octalysis Voluntary Adoption Cascade (Moore’s Chasm
# bridge at 16%); Generative UX (Thinking Frontend paradigm);
# Dashy action‑object matrix; CopilotKit AG-UI & Google A2UI
# protocol mappings; personalized, role‑adaptive, industry‑refined
# dashboards that evolve per user.
# ============================================================
set -e

mkdir -p crates/cortex-interface/src

# Crate manifest
cat > crates/cortex-interface/Cargo.toml << 'CRATETOML'
[package]
name = "cortex-interface"
version.workspace = true
edition.workspace = true

[dependencies]
cortex-core = { path = "../cortex-core" }
cortex-gateway = { path = "../cortex-gateway" }
cortex-council = { path = "../cortex-council" }
cortex-provenance = { path = "../cortex-provenance" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
async-trait = "0.1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
reqwest = { version = "0.12", features = ["json"] }
CRATETOML

# ---- lib.rs: InterfaceEngine orchestrator ----
cat > crates/cortex-interface/src/lib.rs << 'LIBEOF'
//! Cortex InterfaceEngine — the Interface of One.
//!
//! Every user sees a unique dashboard generated from their role,
//! behaviour, and industry context. The engine learns which fields
//! they access, which queries they ask, and proactively surfaces
//! exactly what they need across all devices.
//!
//! Subsystems:
//!   PersonalizedDashboard   — per‑user, evolving, role‑adaptive
//!   CrossSystemCommandBar   — single NL input for all systems
//!   WidgetGenerator         — auto‑charts from NL queries
//!   NotificationManager     — proactive alerts
//!   WeaningEngine           — progressive legacy‑app replacement
//!   ObservationalCapture    — browser ext / accessibility / OCR
//!   (Further modules in subsequent batches)

pub mod personalized_dashboard;
pub mod cross_system_bar;
pub mod widget_generator;
pub mod notification_manager;
pub mod weaning_engine;
pub mod observational_capture;
// Remaining modules will be added in batches 6b and 6c

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top‑level interface orchestrator.
pub struct InterfaceEngine {
    pub dashboard: personalized_dashboard::PersonalizedDashboard,
    pub command_bar: cross_system_bar::CrossSystemCommandBar,
    pub widget_gen: widget_generator::WidgetGenerator,
    pub notifier: notification_manager::NotificationManager,
    pub weaning: weaning_engine::WeaningEngine,
    pub obs_capture: observational_capture::ObservationalCapture,
}

impl InterfaceEngine {
    pub fn new() -> Self {
        Self {
            dashboard: personalized_dashboard::PersonalizedDashboard::new(),
            command_bar: cross_system_bar::CrossSystemCommandBar::new(),
            widget_gen: widget_generator::WidgetGenerator::new(),
            notifier: notification_manager::NotificationManager::new(),
            weaning: weaning_engine::WeaningEngine::new(),
            obs_capture: observational_capture::ObservationalCapture::new(),
        }
    }

    /// Render the full dashboard for a user.
    pub async fn render_dashboard(&self, user_id: &str) -> personalized_dashboard::Dashboard {
        self.dashboard.render(user_id).await
    }

    /// Handle a natural‑language query via the command bar.
    pub async fn handle_query(
        &self,
        nl: &str,
        user_id: &str,
    ) -> cross_system_bar::CommandBarResult {
        self.command_bar.execute(nl, user_id).await
    }
}
LIBEOF

# ---- personalized_dashboard.rs ----
cat > crates/cortex-interface/src/personalized_dashboard.rs << 'DASHBOARDEnd'
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
DASHBOARDEnd

# ---- cross_system_bar.rs (interface‑specific version) ----
cat > crates/cortex-interface/src/cross_system_bar.rs << 'CSBEOF'
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
CSBEOF

# ---- widget_generator.rs ----
cat > crates/cortex-interface/src/widget_generator.rs << 'WIDGETEOF'
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
WIDGETEOF

# ---- notification_manager.rs ----
cat > crates/cortex-interface/src/notification_manager.rs << 'NOTIFYEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Proactive alerts that pull users into Cortex instead of requiring
/// them to remember to check it.
///
/// Part of the “addictive” UX architecture: alerts are role‑specific,
/// learned from behaviour, and delivered across devices.
pub struct NotificationManager {
    /// Subscriptions per user per channel.
    subscriptions: RwLock<HashMap<String, Vec<NotificationChannel>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub body: String,
    pub severity: NotificationSeverity,
    pub action: Option<NotificationAction>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationAction {
    pub label: String,
    pub action_type: ActionType,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    OpenPanel,
    ExecuteSkill,
    ViewReport,
    ApproveRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    InApp,
    Email,
    Slack,
    Teams,
    Push,
    SMS,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self { subscriptions: RwLock::new(HashMap::new()) }
    }

    /// Send a notification to a user through all active channels.
    pub async fn notify(&self, notification: Notification) {
        tracing::info!(
            user = %notification.user_id,
            title = %notification.title,
            "Sending notification"
        );
        // In production: dispatch via configured channels.
    }

    /// Register a channel preference for a user.
    pub async fn set_channels(&self, user_id: &str, channels: Vec<NotificationChannel>) {
        self.subscriptions.write().await.insert(user_id.to_string(), channels);
    }
}
NOTIFYEOF

# ---- weaning_engine.rs ----
cat > crates/cortex-interface/src/weaning_engine.rs << 'WEANEOF'
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Progressive Weaning Engine (v2–v8).
///
/// Tracks which workflows users still perform in legacy applications
/// and proactively migrates them to Cortex. Over 4‑6 weeks, 80% of
/// workflows migrate by convenience, not mandate.
///
/// Implements the Octalysis Voluntary Adoption Cascade and the
/// Strangler Fig façade pattern: the legacy app remains available as
/// a fallback; users stop using it because Cortex is faster.
pub struct WeaningEngine {
    /// Per‑user migration progress (percentage of workflows absorbed).
    progress: RwLock<HashMap<String, WeaningProgress>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaningProgress {
    pub user_id: String,
    pub legacy_application: String,
    pub absorbed_workflow_count: u64,
    pub remaining_workflow_count: u64,
    pub absorption_pct: f64,
    pub last_suggestion_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl WeaningEngine {
    pub fn new() -> Self {
        Self { progress: RwLock::new(HashMap::new()) }
    }

    /// Check if a user should be nudged toward Cortex for a given workflow.
    /// The nudge is shown only once per workflow every 7 days.
    pub async fn should_nudge(
        &self,
        user_id: &str,
        legacy_app: &str,
        skill_name: &str,
    ) -> Option<String> {
        let progress = self.progress.read().await;
        let key = format!("{user_id}:{legacy_app}:{skill_name}");
        if let Some(entry) = progress.get(&key) {
            // Already nudged recently
            if let Some(last) = entry.last_suggestion_at {
                if chrono::Utc::now() - last < chrono::Duration::days(7) {
                    return None;
                }
            }
        }

        Some(format!(
            "I can now run '{skill_name}' in Cortex — it takes 30 seconds instead of 20 minutes. Want to try it?"
        ))
    }

    /// Record that a nudge was shown.
    pub async fn record_nudge(&self, user_id: &str, legacy_app: &str, skill_name: &str) {
        let mut progress = self.progress.write().await;
        let key = format!("{user_id}:{legacy_app}:{skill_name}");
        progress.entry(key)
            .and_modify(|e| e.last_suggestion_at = Some(chrono::Utc::now()))
            .or_insert_with(|| WeaningProgress {
                user_id: user_id.to_string(),
                legacy_application: legacy_app.to_string(),
                absorbed_workflow_count: 1,
                remaining_workflow_count: 0,
                absorption_pct: 100.0,
                last_suggestion_at: Some(chrono::Utc::now()),
            });
    }

    /// Get absorption score for a user/application.
    pub async fn absorption_score(&self, user_id: &str, app: &str) -> f64 {
        let progress = self.progress.read().await;
        progress.iter()
            .filter(|(k, v)| k.starts_with(&format!("{user_id}:{app}:")) && v.absorbed_workflow_count > 0)
            .map(|(_, v)| v.absorption_pct)
            .fold(0.0, f64::max)
    }
}
WEANEOF

# ---- observational_capture.rs ----
cat > crates/cortex-interface/src/observational_capture.rs << 'OBSEOF'
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Observational Capture Engine (v2/v3/v8).
///
/// Records user interactions with legacy applications via browser
/// extension, accessibility API, OCR, and terminal emulation.
/// Converts observed workflows into reusable agent skills.
pub struct ObservationalCapture {
    sessions: RwLock<Vec<CaptureSession>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureSession {
    pub session_id: String,
    pub user_id: String,
    pub application: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub events: Vec<CaptureEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureEvent {
    pub event_type: CaptureEventType,
    pub field_id: Option<String>,
    pub value: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptureEventType {
    PageNavigation,
    FieldFocus,
    FieldChange,
    FormSubmit,
    ButtonClick,
    ErrorDisplayed,
}

impl ObservationalCapture {
    pub fn new() -> Self {
        Self { sessions: RwLock::new(Vec::new()) }
    }

    /// Start a new capture session for a user.
    pub async fn start_session(&self, user_id: &str, application: &str) -> CaptureSession {
        let session = CaptureSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            application: application.to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            events: Vec::new(),
        };
        self.sessions.write().await.push(session.clone());
        session
    }

    /// Record an event within an active session.
    pub async fn record_event(&self, session_id: &str, event: CaptureEvent) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.iter_mut().find(|s| s.session_id == session_id) {
            session.events.push(event);
        }
    }

    /// End a capture session.
    pub async fn end_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.iter_mut().find(|s| s.session_id == session_id) {
            session.end_time = Some(chrono::Utc::now());
        }
    }

    /// Convert an observed workflow into a skill draft (for Forge).
    pub async fn convert_to_skill_draft(&self, session_id: &str) -> Option<SkillDraft> {
        let sessions = self.sessions.read().await;
        let session = sessions.iter().find(|s| s.session_id == session_id)?;
        if session.events.len() < 3 { return None; }

        let tokens: Vec<String> = session.events.iter().map(|e| format!("{:?}", e.event_type)).collect();
        Some(SkillDraft {
            source_session: session_id.to_string(),
            tokens,
            confidence: 0.8,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDraft {
    pub source_session: String,
    pub tokens: Vec<String>,
    pub confidence: f64,
}
OBSEOF

echo "✅ Batch 6a complete — Interface Engine part 1 (7 files)"
echo ""
echo "Created:"
echo "  - Cargo.toml"
echo "  - lib.rs                  (InterfaceEngine orchestrator)"
echo "  - personalized_dashboard.rs  (Evolving per‑user dashboard)"
echo "  - cross_system_bar.rs     (Command bar with history)"
echo "  - widget_generator.rs     (Auto‑charts from NL queries)"
echo "  - notification_manager.rs (Proactive alerts across channels)"
echo "  - weaning_engine.rs       (Progressive legacy‑app replacement)"
echo "  - observational_capture.rs (Browser / a11y / OCR / terminal)"
echo ""
echo "Literature grounding:"
echo "  - NOVAID/AGENTUI.AI widget generation"
echo "  - Capgemini Zero UI (informational to transactional journey)"
echo "  - Octalysis Voluntary Adoption Cascade (16% Moore’s Chasm)"
echo "  - Generative UX / Thinking Frontend (2026 paradigm)"
echo "  - Dashy action‑object matrix (UX Middleware for Cognitive Split)"
echo "  - CopilotKit AG-UI & Google A2UI dual protocol"