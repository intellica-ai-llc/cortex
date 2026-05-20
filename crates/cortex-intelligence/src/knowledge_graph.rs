use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Cross‑entity relationship mapping.
pub struct KnowledgeGraph {
    entities: HashMap<String, Entity>,
    relations: Vec<Relation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub from_entity_id: String,
    pub to_entity_id: String,
    pub relation_type: String,
    pub weight: f64,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            relations: Vec::new(),
        }
    }

    /// Add an entity to the graph.
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.clone(), entity);
    }

    /// Add a relation between two entities.
    pub fn add_relation(&mut self, rel: Relation) {
        self.relations.push(rel);
    }

    /// Query entities related to a given entity.
    pub fn query_related(&self, entity_id: &str) -> Vec<&Entity> {
        let related_ids: HashSet<&str> = self.relations.iter()
            .filter(|r| r.from_entity_id == entity_id || r.to_entity_id == entity_id)
            .flat_map(|r| [r.from_entity_id.as_str(), r.to_entity_id.as_str()])
            .filter(|id| *id != entity_id)
            .collect();
        related_ids.iter()
            .filter_map(|id| self.entities.get(*id))
            .collect()
    }
}
