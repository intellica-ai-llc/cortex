//! Agent Topology View — Interactive Spatial Agent Relationship Graph
//!
//! Based on OpenClaw Office / ClawProwl (Mar 2026): isometric SVG
//! rendering of agent workspaces. "It renders Agent work status,
//! collaboration links, tool calls, and resource consumption through
//! an isometric-style virtual office scene."
//!
//! Core metaphor: Agent = Digital Employee | Desk = Session |
//! Meeting Pod = Collaboration Context.
//!
//! Also inspired by VisCritic-GIS (Mar 2026): multi-agent visualisation
//! with explicit spatial relations externalised in visual form.
//! And Space Agents! (Feb 2026): visualising codebase topology and
//! the swarm of agents operating on it in real time.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AgentTopologyView;

/// The complete agent topology for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTopology {
    pub nodes: Vec<AgentNode>,
    pub edges: Vec<CollaborationEdge>,
    pub layout: TopologyLayout,
    pub metadata: TopologyMetadata,
}

/// A single agent node in the topology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNode {
    pub agent_id: String,
    pub agent_name: String,
    pub agent_role: String,          // "MAE", "MI", "PCA", etc.
    pub status: AgentVisualStatus,
    pub position: (f64, f64),        // x, y in layout coordinates
    pub resource_usage: ResourceUsage,
    pub active_tool_calls: u32,
    /// Deterministically generated avatar from agent_id (ClawProwl pattern).
    pub avatar_seed: String,
}

/// Visual status mapped from agent lifecycle events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentVisualStatus {
    Idle,
    Working,
    Speaking,
    ToolCalling { tool_name: String },
    Error { message: String },
    Offline,
}

/// Agent resource consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub tokens_used: u64,
    pub tokens_per_minute: f64,
    pub memory_mb: f64,
    pub cpu_pct: f64,
}

/// A collaboration edge between two agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationEdge {
    pub from_agent_id: String,
    pub to_agent_id: String,
    pub edge_type: CollaborationType,
    pub message_count: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationType {
    Delegation,         // formal handoff (Tether)
    Message,            // direct communication
    SharedToolCall,     // both agents called same tool
    ReviewFeedback,     // QC agent reviewed another's output
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyLayout {
    Isometric,          // 2D isometric SVG (OpenClaw/ClawProwl)
    ForceDirected,      // physics-based graph layout
    Hierarchical,       // tree based on reporting structure
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyMetadata {
    pub total_agents: u32,
    pub active_agents: u32,
    pub total_edges: u32,
    pub total_tool_calls_24h: u64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl AgentTopologyView {
    pub fn new() -> Self { Self }

    /// Build the agent topology from the council's current state.
    ///
    /// OpenClaw Office pattern: agents are rendered as SVG avatars
    /// on a 2D isometric grid with collaboration lines connecting
    /// agents that have communicated.
    pub fn build_topology(
        &self,
        agents: &[AgentNode],
        edges: &[CollaborationEdge],
    ) -> AgentTopology {
        let active = agents.iter().filter(|a| a.status != AgentVisualStatus::Offline).count() as u32;
        let total_calls = agents.iter().map(|a| a.active_tool_calls as u64).sum();

        AgentTopology {
            nodes: agents.to_vec(),
            edges: edges.to_vec(),
            layout: TopologyLayout::Isometric,
            metadata: TopologyMetadata {
                total_agents: agents.len() as u32,
                active_agents: active,
                total_edges: edges.len() as u32,
                total_tool_calls_24h: total_calls,
                generated_at: chrono::Utc::now(),
            },
        }
    }

    /// Generate the A2UI spec for rendering the topology.
    ///
    /// Renders as an interactive SVG component where:
    ///   - Agent nodes show avatars with status-coloured borders.
    ///   - Collaboration lines pulse when active.
    ///   - Hovering a node shows tool call details.
    ///   - Clicking a node opens the agent's detail panel.
    pub fn to_a2ui_spec(&self, topology: &AgentTopology) -> serde_json::Value {
        serde_json::json!({
            "surface_id": uuid::Uuid::new_v4().to_string(),
            "components": [{
                "id": "topology-canvas",
                "component_type": "AgentTopology",
                "properties": {
                    "nodes": topology.nodes.iter().map(|n| serde_json::json!({
                        "id": n.agent_id,
                        "name": n.agent_name,
                        "role": n.agent_role,
                        "status": format!("{:?}", n.status),
                        "x": n.position.0,
                        "y": n.position.1,
                        "avatarSeed": n.avatar_seed,
                        "tokensPerMin": n.resource_usage.tokens_per_minute,
                    })).collect::<Vec<_>>(),
                    "edges": topology.edges.iter().map(|e| serde_json::json!({
                        "from": e.from_agent_id,
                        "to": e.to_agent_id,
                        "type": format!("{:?}", e.edge_type),
                        "active": e.active,
                        "messageCount": e.message_count,
                    })).collect::<Vec<_>>(),
                    "layout": format!("{:?}", topology.layout),
                }
            }]
        })
    }
}
