use serde::{Deserialize, Serialize};

/// Writer Agent (Aw) — text composition & AVR definition.
///
/// CogGen Micro-Cognitive Cycle: "Per-section processing where each
/// section undergoes independent search → plan → write → revise cycles
/// with format, factual, and cognitive revision."
///
/// Sub-agents: section_writer, write_loop, role_inference, content_cleanup.
pub struct WriterAgent;

/// A drafted section with citations and confidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftedSection {
    pub section_id: String,
    pub title: String,
    pub content: String,
    pub citations: Vec<super::Citation>,
    pub confidence: f64,
    pub word_count: usize,
    pub drafted_at: chrono::DateTime<chrono::Utc>,
}

/// The writing context passed to each section writer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingContext {
    pub question: String,
    pub section_title: String,
    pub sub_questions: Vec<String>,
    pub research_findings: Vec<ResearchFinding>,
    pub style_guide: WritingStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchFinding {
    pub source_url: Option<String>,
    pub source_text: String,
    pub relevance: f64,
    pub extracted_facts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingStyle {
    pub tone: String,        // "academic", "business", "technical"
    pub max_words: usize,
    pub require_citations: bool,
    pub language: String,    // "en"
}

impl WriterAgent {
    pub fn new() -> Self { Self }

    /// Draft a section using the research findings as evidence.
    ///
    /// The section_writer sub-agent: "Generates section content by
    /// synthesising research findings, maintaining inline citations,
    /// and producing a confidence score for each factual claim."
    pub async fn draft_section(
        &self,
        ctx: &WritingContext,
    ) -> DraftedSection {
        // In production: LLM-powered synthesis with citation grounding.
        let content = format!(
            "## {}\n\nThis section addresses: {}\n\nBased on {} research findings, \
             the analysis reveals several key insights.\n\n*Evidence from {} sources \
             supports the following conclusions.*",
            ctx.section_title,
            ctx.sub_questions.join(", "),
            ctx.research_findings.len(),
            ctx.research_findings.len(),
        );

        let citations: Vec<super::Citation> = ctx.research_findings.iter().map(|f| {
            super::Citation {
                source_url: f.source_url.clone(),
                source_text: f.source_text.chars().take(200).collect(),
                relevance_score: f.relevance,
                verified: f.relevance > 0.7,
            }
        }).collect();

        DraftedSection {
            section_id: uuid::Uuid::new_v4().to_string(),
            title: ctx.section_title.clone(),
            content,
            citations,
            confidence: 0.85,
            word_count: content.split_whitespace().count(),
            drafted_at: chrono::Utc::now(),
        }
    }

    /// Revise a section based on Reviewer feedback.
    /// Micro-Cognitive Cycle: section undergoes revision triggered
    /// by the Reviewer's format, factual, and cognitive feedback.
    pub async fn revise_section(
        &self,
        _original: &DraftedSection,
        _feedback: &super::reviewer_agent::SectionFeedback,
    ) -> DraftedSection {
        // In production: address reviewer's specific concerns,
        // add missing citations, correct factual errors, restructure.
        DraftedSection {
            section_id: uuid::Uuid::new_v4().to_string(),
            title: "Revised section".into(),
            content: "Revised content addressing reviewer feedback.".into(),
            citations: vec![],
            confidence: 0.92,
            word_count: 100,
            drafted_at: chrono::Utc::now(),
        }
    }
}
