//! Unified Workspace — Merged Morning Brief + Command Center + Command Bar
//!
//! Based on Lofty AI Dashboard (Apr 2026): "A multimodal, voice-enabled
//! AI summary that instantly gives agents the pulse of their pipeline
//! and their daily agenda, combined with an AI Command Center."
//! Glean Assistant (Feb 2026): unified agentic workspace.
//!
//! The Unified Workspace presents the Morning Brief as the starting
//! point, with the Command Bar embedded directly within it so users
//! can act on the brief immediately. From there, users can seamlessly
//! transition into the full Command Center for agent monitoring.

use serde::{Deserialize, Serialize};

pub struct UnifiedWorkspace;

/// The complete unified workspace for a user session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub user_id: String,
    pub workspace_mode: WorkspaceMode,
    pub morning_brief: Option<BriefPanel>,
    pub command_bar: CommandBarPanel,
    pub command_center: Option<CommandCenterPanel>,
    pub topology: Option<super::agent_topology_view::AgentTopology>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkspaceMode {
    Brief,           // Morning Brief with embedded Command Bar
    CommandCenter,   // Full agent monitoring
    Hybrid,          // Split view: Brief on left, Topology on right
}

/// The Morning Brief panel within the unified workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BriefPanel {
    pub greeting: String,
    pub pulse_score: Option<f64>,
    pub key_metrics: Vec<MetricCard>,
    pub regulatory_alerts: Vec<AlertCard>,
    pub cross_system_insight: Option<String>,
    pub suggested_actions: Vec<String>,
    pub speakable: bool,       // whether voice output is available
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCard {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub change_pct: f64,
    pub benchmark: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCard {
    pub regulation: String,
    pub message: String,
    pub days_remaining: i64,
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity { Critical, High, Medium }

/// The Command Bar panel — always visible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBarPanel {
    pub placeholder: String,
    pub voice_enabled: bool,
    pub recent_queries: Vec<String>,
    pub suggested_queries: Vec<String>,
}

/// The Command Center panel — full agent monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCenterPanel {
    pub active_agents: u32,
    pub success_rate: f64,
    pub tool_calls_today: u64,
    pub alerts: u32,
}

impl UnifiedWorkspace {
    pub fn new() -> Self { Self }

    /// Build the unified workspace for a user session.
    ///
    /// The workspace adapts based on the user's role and adoption stage:
    ///   - New users: Brief mode with guided onboarding.
    ///   - Active users: Hybrid mode with Brief + Topology.
    ///   - Power users: Command Center mode with full monitoring.
    pub fn build(
        user_id: &str,
        brief: Option<BriefPanel>,
        command_center: Option<CommandCenterPanel>,
        topology: Option<super::agent_topology_view::AgentTopology>,
    ) -> Workspace {
        let mode = if command_center.is_some() && topology.is_some() {
            WorkspaceMode::Hybrid
        } else if command_center.is_some() {
            WorkspaceMode::CommandCenter
        } else {
            WorkspaceMode::Brief
        };

        Workspace {
            user_id: user_id.to_string(),
            workspace_mode: mode,
            morning_brief: brief,
            command_bar: CommandBarPanel {
                placeholder: "Ask anything across all systems... or use voice 🎤".into(),
                voice_enabled: true,
                recent_queries: vec![],
                suggested_queries: vec![
                    "Show me open work orders with PM due this week".into(),
                    "Compare maintenance cost vs budget for Q3".into(),
                ],
            },
            command_center,
            topology,
        }
    }

    /// Generate A2UI JSON for the unified workspace.
    pub fn to_a2ui(&self, workspace: &Workspace) -> serde_json::Value {
        serde_json::json!({
            "surface_id": uuid::Uuid::new_v4().to_string(),
            "workspace_mode": format!("{:?}", workspace.workspace_mode),
            "components": [
                {
                    "id": "command-bar",
                    "component_type": "CommandBar",
                    "properties": workspace.command_bar,
                },
                {
                    "id": "brief-panel",
                    "component_type": "MorningBrief",
                    "properties": workspace.morning_brief,
                },
            ],
        })
    }
}
