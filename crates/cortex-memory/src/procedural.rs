use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Procedural Store (L3) — skills, workflows, tool patterns.
pub struct ProceduralStore {
    skills: RwLock<HashMap<String, SkillEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEntry {
    pub skill_id: String,
    pub name: String,
    pub tokens: Vec<String>,
    pub success_rate: f64,
    pub total_executions: u64,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub deprecated: bool,
}

impl ProceduralStore {
    pub fn new() -> Self {
        Self { skills: RwLock::new(HashMap::new()) }
    }

    pub async fn register_skill(&self, skill: SkillEntry) {
        self.skills.write().await.insert(skill.skill_id.clone(), skill);
    }

    pub async fn get_skill(&self, id: &str) -> Option<SkillEntry> {
        self.skills.read().await.get(id).cloned()
    }
}
