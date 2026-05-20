use serde::{Deserialize, Serialize};

/// Swarm Leader — task decomposition & orchestration.
///
/// ZetaSwarm (Lantern Pharma): "a coordinated network of specialist AI
/// agents that operate in parallel on scientific sub-problems." The
/// leader decomposes the main question into sub-questions, assigns each
/// to a specialist agent, and orchestrates the overall campaign.
pub struct SwarmLeader;

/// Decomposition result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompositionResult {
    pub main_question: String,
    pub sub_questions: Vec<super::SubQuestionAssignment>,
    pub strategy: DecompositionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecompositionStrategy {
    /// By domain: each sub-agent covers a different knowledge domain.
    ByDomain,
    /// By perspective: competitive, regulatory, technology, etc.
    ByPerspective,
    /// By time period: historical, current, future.
    ByTimePeriod,
    /// By geography: regional analysis.
    ByGeography,
}

impl SwarmLeader {
    pub fn new() -> Self { Self }

    /// Decompose a complex research question into sub-questions.
    ///
    /// AI Scientific Community (Braga-Neto): "Each particle in the swarm
    /// represents a complete virtual laboratory instance." The leader
    /// decides how many sub-agents to spawn and what each should focus on.
    pub async fn decompose(
        &self,
        question: &str,
    ) -> Vec<super::SubQuestionAssignment> {
        // By-perspective decomposition: competitive, regulatory, technology.
        let perspectives = vec![
            ("competitive", "Competitive Landscape"),
            ("regulatory", "Regulatory Framework"),
            ("technology", "Technology Assessment"),
            ("market", "Market Analysis"),
        ];

        perspectives.into_iter().enumerate().map(|(i, (domain, label))| {
            super::SubQuestionAssignment {
                id: format!("sq_{}", i),
                question: format!("[{}] {}", label, question),
                assigned_agent_id: None,
                domain: domain.to_string(),
                status: super::SubQuestionStatus::Pending,
                findings: None,
            }
        }).collect()
    }

    /// Assign sub-questions to specialist agents.
    pub async fn assign(
        &self,
        sub_questions: &mut [super::SubQuestionAssignment],
        available_agents: &[String],
    ) {
        for (i, sq) in sub_questions.iter_mut().enumerate() {
            if i < available_agents.len() {
                sq.assigned_agent_id = Some(available_agents[i].clone());
                sq.status = super::SubQuestionStatus::Assigned;
            }
        }
    }
}
