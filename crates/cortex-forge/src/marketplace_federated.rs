use serde::{Deserialize, Serialize};

pub struct FederatedMarketplace;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceListing {
    pub skill_id: String,
    pub publisher: String,
    pub price: f64,
}

impl FederatedMarketplace {
    pub fn new() -> Self { Self }
    pub fn list(&self, skill: &super::skill_synthesis::ForgeSkill, price: f64) -> MarketplaceListing {
        MarketplaceListing {
            skill_id: skill.id.clone(),
            publisher: "unknown".into(),
            price,
        }
    }
}
