use serde::{Deserialize, Serialize};

pub struct SkillPublisher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedSkill {
    pub skill_id: String,
    pub name: String,
    pub publisher: String,
    pub price_usd: f64,
    pub domain: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
}

impl SkillPublisher {
    pub fn new() -> Self { Self }
    pub fn publish(&self, name: &str, domain: &str, price: f64) -> PublishedSkill {
        PublishedSkill {
            skill_id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            publisher: "anonymous".into(),
            price_usd: price,
            domain: domain.to_string(),
            published_at: chrono::Utc::now(),
        }
    }
}
