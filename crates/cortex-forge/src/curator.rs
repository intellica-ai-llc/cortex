use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct Curator {
    managed_skills: RwLock<HashMap<String, super::skill_synthesis::ForgeSkill>>,
}

impl Curator {
    pub fn new() -> Self { Self { managed_skills: RwLock::new(HashMap::new()) } }
    pub async fn register(&self, skill: super::skill_synthesis::ForgeSkill) {
        self.managed_skills.write().await.insert(skill.id.clone(), skill);
    }
}
