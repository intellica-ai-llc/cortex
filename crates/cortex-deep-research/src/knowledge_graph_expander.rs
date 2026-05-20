use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Knowledge Graph Expander — richer exploration paths.
///
/// OpenSeeker‑v2 modification #1: "Scaling knowledge graph size
/// for richer exploration." A larger KG provides more entities
/// and relationships for the agent to traverse, increasing the
/// diversity and depth of synthesised trajectories.
///
/// Cortex integrates with Knowledge Snap (v3) industry‑specific
/// templates to seed the KG, then expands it from the customer's
/// own documents, wikis, and regulatory filings.
pub struct KnowledgeGraphExpander {
    entities: HashMap<String, KnowledgeEntity>,
    relations: Vec<KnowledgeRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntity {
    pub id: String,
    pub name: String,
    pub entity_type: String,     // "company", "regulation", "product", "concept"
    pub properties: serde_json::Value,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    pub from_entity: String,
    pub to_entity: String,
    pub relation_type: String,   // "governs", "produces", "requires", "references"
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpansionResult {
    pub entities_added: usize,
    pub relations_added: usize,
    pub total_entities: usize,
    pub total_relations: usize,
}

impl KnowledgeGraphExpander {
    pub fn new() -> Self {
        Self { entities: HashMap::new(), relations: Vec::new() }
    }

    /// Expand the knowledge graph from a document corpus.
    ///
    /// In production: runs entity extraction (NER) and relation
    /// extraction (RE) over the customer's internal documents,
    /// regulatory filings, and prior research stored in Knowledge Snap.
    pub async fn expand_from_documents(
        &mut self,
        _documents: &[String],
    ) -> ExpansionResult {
        let before_entities = self.entities.len();
        let before_relations = self.relations.len();

        // Placeholder: in production, LLM‑powered extraction.
        self.entities.insert("e1".into(), KnowledgeEntity {
            id: "e1".into(), name: "NERC CIP-015-1".into(),
            entity_type: "regulation".into(), properties: serde_json::json!({}), embedding: None,
        });

        ExpansionResult {
            entities_added: self.entities.len() - before_entities,
            relations_added: self.relations.len() - before_relations,
            total_entities: self.entities.len(),
            total_relations: self.relations.len(),
        }
    }

    /// Get the current KG size (entities count).
    pub fn entity_count(&self) -> usize { self.entities.len() }
    pub fn relation_count(&self) -> usize { self.relations.len() }
}
