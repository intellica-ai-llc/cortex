use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Swarm Consensus Protocol — multi-agent voting on conflicting findings.
///
/// AI Scientific Community (Braga-Neto): "Citation-analogous voting
/// systems, fitness function design for quantifying scientific
/// success, and mechanisms for preventing lab dominance and
/// preserving diversity."
///
/// JumpCloud Consensus Voting (Mar 2026): "a deterministic orchestration
/// mechanism utilizing weighted, multi-agent polling to resolve severe
/// operational misalignments."
///
/// The protocol uses weighted voting where each agent's vote is
/// weighted by its domain expertise confidence. When conflicts arise,
/// the protocol runs a structured vote to determine the resolution.
pub struct SwarmConsensusProtocol {
    /// Agent weights indexed by agent_id.
    weights: RwLock<HashMap<String, f64>>,
}

/// A proposal put to vote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusProposal {
    pub proposal_id: String,
    pub description: String,
    pub options: Vec<VoteOption>,
    pub status: VoteStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteOption {
    pub option_id: String,
    pub label: String,
    pub votes: Vec<AgentVote>,
    pub vote_count: u64,
    pub weighted_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentVote {
    pub agent_id: String,
    pub option_id: String,
    pub weight: f64,
    pub rationale: Option<String>,
    pub voted_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteStatus { Open, Closed, Resolved }

/// Result of a consensus vote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    pub proposal_id: String,
    pub winning_option: String,
    pub winning_score: f64,
    pub total_votes: u64,
    /// Number of unique agents that participated.
    pub participation: usize,
    pub status: VoteStatus,
    pub resolved_at: chrono::DateTime<chrono::Utc>,
}

impl SwarmConsensusProtocol {
    pub fn new() -> Self {
        Self { weights: RwLock::new(HashMap::new()) }
    }

    /// Register an agent's voting weight based on domain expertise.
    pub async fn register_agent(&self, agent_id: &str, expertise_confidence: f64) {
        self.weights.write().await.insert(agent_id.to_string(), expertise_confidence);
    }

    /// Run a consensus vote on a proposal.
    ///
    /// Algorithm:
    ///   1. Collect votes from all registered agents.
    ///   2. Weight each vote by the agent's domain expertise.
    ///   3. Determine the winning option (highest weighted score).
    ///   4. If the winning score exceeds threshold, consensus reached.
    ///   5. If below threshold, escalate to human or further research.
    pub async fn vote(
        &self,
        proposal: &ConsensusProposal,
        votes: Vec<AgentVote>,
    ) -> ConsensusResult {
        let weights = self.weights.read().await;

        // Tally weighted scores per option.
        let mut option_scores: HashMap<String, f64> = HashMap::new();
        for vote in &votes {
            let weight = weights.get(&vote.agent_id).copied().unwrap_or(0.5);
            *option_scores.entry(vote.option_id.clone()).or_default() += weight;
        }

        // Find the winning option.
        let (winning_id, winning_score) = option_scores.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(("none".into(), 0.0));

        ConsensusResult {
            proposal_id: proposal.proposal_id.clone(),
            winning_option: winning_id,
            winning_score,
            total_votes: votes.len() as u64,
            participation: votes.iter().map(|v| &v.agent_id).collect::<std::collections::HashSet<_>>().len(),
            status: VoteStatus::Resolved,
            resolved_at: chrono::Utc::now(),
        }
    }

    /// Check if consensus is strong enough to proceed automatically.
    pub fn is_consensus_reached(result: &ConsensusResult, threshold: f64) -> bool {
        result.winning_score >= threshold
    }
}
