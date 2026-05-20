use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Contribution-based credit and reward system.
pub struct CreditSystem {
    balances: RwLock<HashMap<String, f64>>,
}

impl CreditSystem {
    pub fn new() -> Self { Self { balances: RwLock::new(HashMap::new()) } }
    pub async fn credit(&self, contributor: &str, amount: f64) {
        *self.balances.write().await.entry(contributor.into()).or_default() += amount;
    }
}
