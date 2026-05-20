//! Cortex IntelligencePipeline – Meeting & Document Ingestion.
//!
//! Transforms unstructured enterprise data (calendar meetings,
//! documents, spreadsheets) into structured knowledge accessible
//! to the agent council. Based on MeetingMind MCP Server pattern:
//! Calendar → Transcript → Extraction → Action Items.

pub mod meeting_ingestor;
pub mod document_processor;
pub mod knowledge_graph;
pub mod llm_extractor;

use std::sync::Arc;

pub struct IntelligencePipeline {
    pub meeting_ingestor: Arc<meeting_ingestor::MeetingIngestor>,
    pub document_processor: Arc<document_processor::DocumentProcessor>,
    pub knowledge_graph: Arc<knowledge_graph::KnowledgeGraph>,
    pub llm_extractor: Arc<llm_extractor::LLMExtractor>,
}

impl IntelligencePipeline {
    pub fn new() -> Self {
        Self {
            meeting_ingestor: Arc::new(meeting_ingestor::MeetingIngestor::new()),
            document_processor: Arc::new(document_processor::DocumentProcessor::new()),
            knowledge_graph: Arc::new(knowledge_graph::KnowledgeGraph::new()),
            llm_extractor: Arc::new(llm_extractor::LLMExtractor::new()),
        }
    }
}
