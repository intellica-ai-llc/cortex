use serde::{Deserialize, Serialize};

/// Planner Agent (Aₚ) — information retrieval & structural planning.
///
/// CogGen Macro-Cognitive Loop: "The Planner generates a global outline,
/// sections are written in parallel, and the Reviewer provides structural/
/// content feedback (Δ) to trigger replanning iterations."
///
/// Sub-agents: init_research, outline, section_plan, section_search,
/// replan_loop, combine_plan.
pub struct PlannerAgent;

/// A research plan produced by the Planner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPlan {
    pub question: String,
    pub outline: Vec<OutlineSection>,
    pub search_queries: Vec<SearchQuery>,
    pub estimated_sections: usize,
    pub planned_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineSection {
    pub id: String,
    pub title: String,
    pub parent_id: Option<String>,
    pub depth: u32,
    pub assigned_sub_questions: Vec<String>,
    pub required_sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub target_section_id: String,
    pub priority: u8,
}

impl PlannerAgent {
    pub fn new() -> Self { Self }

    /// Decompose a research question into sections and sub-questions.
    ///
    /// Phase 1 — init_research: broad search to understand the domain.
    /// Phase 2 — outline: generate hierarchical section structure.
    /// Phase 3 — section_plan: assign sub-questions and required sources.
    /// Phase 4 — section_search: generate targeted search queries.
    pub async fn plan(&self, question: &str, _domain: &str) -> ResearchPlan {
        // In production: LLM-powered decomposition with constraint validation.
        let sections = vec![
            OutlineSection {
                id: "s1".into(), title: "Executive Summary".into(),
                parent_id: None, depth: 0,
                assigned_sub_questions: vec![format!("What is the core answer to: {}", question)],
                required_sources: vec![],
            },
            OutlineSection {
                id: "s2".into(), title: "Background & Context".into(),
                parent_id: None, depth: 0,
                assigned_sub_questions: vec![format!("What is the background of: {}", question)],
                required_sources: vec![],
            },
            OutlineSection {
                id: "s3".into(), title: "Analysis & Evidence".into(),
                parent_id: None, depth: 0,
                assigned_sub_questions: vec![format!("What evidence supports the answer to: {}", question)],
                required_sources: vec![],
            },
            OutlineSection {
                id: "s4".into(), title: "Conclusions & Recommendations".into(),
                parent_id: None, depth: 0,
                assigned_sub_questions: vec![format!("What should be done about: {}", question)],
                required_sources: vec![],
            },
        ];

        ResearchPlan {
            question: question.to_string(),
            outline: sections,
            search_queries: vec![
                SearchQuery { query: question.to_string(), target_section_id: "s1".into(), priority: 1 },
            ],
            estimated_sections: 4,
            planned_at: chrono::Utc::now(),
        }
    }

    /// Replan based on Reviewer feedback (Δ).
    ///
    /// The replan_loop sub-agent: receives the Reviewer's structural and
    /// content feedback, modifies the outline, spawns new sub-questions,
    /// and triggers additional research for sections that need revision.
    pub async fn replan(
        &self,
        _current_plan: &ResearchPlan,
        feedback: &super::reviewer_agent::ReviewFeedback,
    ) -> ResearchPlan {
        // Add new sections for any gaps identified by the Reviewer.
        ResearchPlan {
            question: String::new(),
            outline: feedback.gaps.iter().enumerate().map(|(i, gap)| OutlineSection {
                id: format!("gap_{}", i),
                title: gap.clone(),
                parent_id: None, depth: 0,
                assigned_sub_questions: vec![gap.clone()],
                required_sources: vec![],
            }).collect(),
            search_queries: vec![],
            estimated_sections: feedback.gaps.len(),
            planned_at: chrono::Utc::now(),
        }
    }
}
