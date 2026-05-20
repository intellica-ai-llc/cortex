//! Progressive Disclosure — Three-Level Agent Reasoning Viewer
//!
//! Based on Luke Wroblewski (Feb 2026): "Tool calls were collapsed by
//! default, and selecting one would show its results in the right column."
//! and Building AI-Native Design Systems (Feb 2026): "Three detail levels:
//! summary view (what was decided), intermediate view (key reasoning steps),
//! detailed view (complete trace with timestamps and API calls)."
//!
//! Cortex implements three progressive disclosure levels:
//!   Level 1 — Summary: "Agent closed work order WO-5521 (confidence 94%)"
//!   Level 2 — Intermediate: Expandable tool calls with results
//!   Level 3 — Detailed: Full TraceCaps provenance with Merkle proofs

use serde::{Deserialize, Serialize};

pub struct ProgressiveDisclosure;

/// The three progressive disclosure levels for any agent action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActionDisclosure {
    pub action_id: String,
    pub agent_name: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Level 1 — Summary. Always visible. One line.
    pub summary: String,

    /// Level 2 — Intermediate. Expandable by user. Shows tool calls.
    pub tool_calls: Vec<DisclosedToolCall>,

    /// Level 3 — Detailed. Available on demand. Full provenance.
    pub provenance: Option<DetailedProvenance>,

    /// Current disclosure level.
    pub current_level: DisclosureLevel,
}

/// A tool call that can be individually expanded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisclosedToolCall {
    pub tool_name: String,
    pub tool_description: String,
    pub status: ToolCallStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub result_summary: Option<String>,
    /// Whether the user has expanded this tool call.
    pub expanded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCallStatus {
    Pending,
    InProgress { progress_pct: f64 },
    Success,
    Failed { error: String },
}

/// Full provenance detail for Level 3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedProvenance {
    pub capsule_id: String,
    pub merkle_hash: String,
    pub risk_score: f64,
    pub vap_level: String,
    pub parent_action_ids: Vec<String>,
    pub evidence_chain: Vec<String>,
    pub scitt_receipt: Option<String>,
    pub signature: Option<String>,
}

/// The current disclosure level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisclosureLevel {
    /// Only the summary line is visible.
    Summary,
    /// Tool calls are visible, collapsed by default.
    Intermediate,
    /// Full provenance chain is visible.
    Detailed,
}

impl ProgressiveDisclosure {
    pub fn new() -> Self { Self }

    /// Build a progressive disclosure object from an agent action.
    ///
    /// The disclosure starts at Summary level. The user can expand
    /// to Intermediate (tool calls) and Detailed (provenance).
    pub fn disclose(
        agent_name: &str,
        summary: &str,
        tool_calls: Vec<DisclosedToolCall>,
        provenance: Option<DetailedProvenance>,
    ) -> AgentActionDisclosure {
        AgentActionDisclosure {
            action_id: uuid::Uuid::new_v4().to_string(),
            agent_name: agent_name.to_string(),
            timestamp: chrono::Utc::now(),
            summary: summary.to_string(),
            tool_calls,
            provenance,
            current_level: DisclosureLevel::Summary,
        }
    }

    /// Advance to the next disclosure level.
    pub fn advance(disclosure: &mut AgentActionDisclosure) {
        disclosure.current_level = match disclosure.current_level {
            DisclosureLevel::Summary => DisclosureLevel::Intermediate,
            DisclosureLevel::Intermediate => DisclosureLevel::Detailed,
            DisclosureLevel::Detailed => DisclosureLevel::Summary, // cycle back
        };
    }

    /// Collapse back to summary.
    pub fn collapse(disclosure: &mut AgentActionDisclosure) {
        disclosure.current_level = DisclosureLevel::Summary;
    }

    /// Get the appropriate ARIA label for the current disclosure level.
    pub fn aria_label(level: &DisclosureLevel) -> &str {
        match level {
            DisclosureLevel::Summary => "Agent action summary",
            DisclosureLevel::Intermediate => "Agent tool calls and reasoning steps",
            DisclosureLevel::Detailed => "Full cryptographic provenance chain",
        }
    }
}
