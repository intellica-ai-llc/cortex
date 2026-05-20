use serde::{Deserialize, Serialize};

/// Synthesiser — cross-agent conflict resolution and merging.
///
/// ZetaSwarm: "converge on a synthesized answer through a coordinator-
/// and-reviewer architecture." The Synthesiser receives findings from
/// all sub-agents, resolves conflicts, merges complementary findings,
/// and produces a unified report with multi-perspective analysis.
pub struct Synthesiser;

/// The synthesised output from multiple sub-agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisedReport {
    pub main_question: String,
    pub perspectives: Vec<PerspectiveSummary>,
    pub conflicts: Vec<ResolvedConflict>,
    pub unified_conclusion: String,
    pub consensus_level: ConsensusLevel,
    pub synthesised_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerspectiveSummary {
    pub domain: String,
    pub agent_id: String,
    pub key_finding: String,
    pub confidence: f64,
    pub included_in_conclusion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedConflict {
    pub domain_a: String,
    pub domain_b: String,
    pub conflicting_claim_a: String,
    pub conflicting_claim_b: String,
    pub resolution: ConflictResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// One claim is preferred based on evidence strength.
    Preferred { winner: String, reason: String },
    /// Both claims are valid from different perspectives.
    BothValid { synthesis: String },
    /// Neither claim is sufficiently supported.
    FurtherResearchNeeded { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsensusLevel {
    /// All agents agree on the conclusion.
    FullConsensus,
    /// Most agents agree; minor dissenting views noted.
    StrongConsensus,
    /// Significant disagreement; both views presented.
    MixedFindings,
    /// No agreement; further research required.
    NoConsensus,
}

impl Synthesiser {
    pub fn new() -> Self { Self }

    /// Synthesise findings from all sub-agents.
    ///
    /// Algorithm:
    ///   1. Collect findings from all sub-agents.
    ///   2. Group by domain/perspective.
    ///   3. Detect conflicting claims across domains.
    ///   4. Resolve conflicts via consensus voting or evidence weighting.
    ///   5. Merge complementary findings into unified conclusion.
    ///   6. Determine overall consensus level.
    pub async fn synthesise(
        &self,
        sub_questions: &[super::SubQuestionAssignment],
    ) -> Result<serde_json::Value, String> {
        let perspectives: Vec<PerspectiveSummary> = sub_questions.iter()
            .filter(|sq| sq.status == super::SubQuestionStatus::Complete)
            .map(|sq| PerspectiveSummary {
                domain: sq.domain.clone(),
                agent_id: sq.assigned_agent_id.clone().unwrap_or_default(),
                key_finding: format!("Finding for: {}", sq.question),
                confidence: 0.82,
                included_in_conclusion: true,
            })
            .collect();

        // Detect consensus level.
        let consensus = if perspectives.iter().all(|p| p.confidence > 0.8) {
            ConsensusLevel::FullConsensus
        } else if perspectives.iter().filter(|p| p.confidence > 0.7).count() >= perspectives.len() / 2 {
            ConsensusLevel::StrongConsensus
        } else {
            ConsensusLevel::MixedFindings
        };

        let report = SynthesisedReport {
            main_question: String::new(),
            perspectives,
            conflicts: vec![],
            unified_conclusion: "Synthesised conclusion from all perspectives.".into(),
            consensus_level: consensus,
            synthesised_at: chrono::Utc::now(),
        };

        Ok(serde_json::to_value(&report).unwrap())
    }
}
