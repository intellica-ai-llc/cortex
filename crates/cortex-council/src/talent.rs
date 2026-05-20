use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// OMC Talent — a portable agent identity.
///
/// OMC (arXiv:2604.22446): "encapsulates skills, tools, and runtime
/// configurations into portable agent identities called Talents,
/// orchestrated through typed organisational interfaces that
/// abstract over heterogeneous backends."
///
/// Each Talent wraps a cognitive identity (role, capabilities,
/// boundaries) with procedural memory (skills) and runtime state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Talent {
    /// Unique portable identifier (DID-based for self-hostability).
    pub id: String,
    /// Organisational role: "mae", "mi", "pca", etc.
    pub role: String,
    /// Human-readable name.
    pub name: String,
    /// What this agent is designed to do.
    pub description: String,
    /// Core capabilities (natural language descriptions).
    pub capabilities: Vec<String>,
    /// Operational boundaries — what this agent must NOT do.
    pub boundaries: Vec<String>,
    /// Acquired skills (procedural memory, Tether skill system).
    pub skills: HashSet<String>,
    /// Runtime state (live context).
    pub state: AgentState,
    /// Performance metrics over recent missions.
    pub performance: PerformanceMetrics,
    /// Decentralised Identifier (portable, self-hostable).
    pub did: String,
    /// When this talent was created.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether this talent is currently active.
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// Short-term working memory (current task).
    pub short_term: serde_json::Value,
    /// Long-term consolidated memory.
    pub long_term: serde_json::Value,
    /// Current session identifier.
    pub session_id: Option<String>,
    /// Last state save timestamp.
    pub last_saved: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total missions completed.
    pub missions_completed: u64,
    /// Success rate (0.0–1.0).
    pub success_rate: f64,
    /// Average latency per mission (ms).
    pub avg_latency_ms: f64,
    /// Total tool calls executed.
    pub total_tool_calls: u64,
    /// Quality score from QC agent reviews.
    pub quality_score: f64,
}

impl Talent {
    /// Create a new Talent with a fresh DID.
    pub fn new(role: &str, name: &str, description: &str) -> Self {
        let did = format!("did:cortex:{}", uuid::Uuid::new_v4());
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            role: role.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            capabilities: Vec::new(),
            boundaries: vec![
                "Never modify production data without approval".into(),
                "Never exfiltrate data outside Cortex".into(),
                "Never disable security controls".into(),
            ],
            skills: HashSet::new(),
            state: AgentState {
                short_term: serde_json::json!({}),
                long_term: serde_json::json!({}),
                session_id: None,
                last_saved: chrono::Utc::now(),
            },
            performance: PerformanceMetrics {
                missions_completed: 0,
                success_rate: 1.0,
                avg_latency_ms: 0.0,
                total_tool_calls: 0,
                quality_score: 1.0,
            },
            did,
            created_at: chrono::Utc::now(),
            active: true,
        }
    }

    /// Add a capability description.
    pub fn add_capability(&mut self, cap: &str) {
        self.capabilities.push(cap.to_string());
    }

    /// Add a boundary constraint.
    pub fn add_boundary(&mut self, boundary: &str) {
        self.boundaries.push(boundary.to_string());
    }

    /// Acquire a new skill (procedural memory).
    pub fn acquire_skill(&mut self, skill: &str) {
        self.skills.insert(skill.to_string());
    }

    /// Check if this talent possesses a given skill.
    pub fn has_skill(&self, skill: &str) -> bool {
        self.skills.contains(skill)
    }

    /// Record a completed mission.
    pub fn record_mission(&mut self, success: bool, latency_ms: f64, tool_calls: u64) {
        let n = self.performance.missions_completed as f64;
        let old_rate = self.performance.success_rate;
        self.performance.missions_completed += 1;
        self.performance.success_rate = (old_rate * n + if success { 1.0 } else { 0.0 }) / (n + 1.0);
        self.performance.avg_latency_ms =
            (self.performance.avg_latency_ms * n + latency_ms) / (n + 1.0);
        self.performance.total_tool_calls += tool_calls;
    }

    /// Update the quality score based on QC review.
    pub fn update_quality(&mut self, score: f64) {
        self.performance.quality_score =
            self.performance.quality_score * 0.7 + score * 0.3; // EMA
    }
}
