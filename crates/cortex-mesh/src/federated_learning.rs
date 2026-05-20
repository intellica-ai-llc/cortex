use serde::{Deserialize, Serialize};

pub struct FederatedLearning;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FLModelUpdate {
    pub node_id: String,
    pub model_hash: String,
    pub dp_epsilon: f64,
}

impl FederatedLearning {
    pub fn new() -> Self { Self }
    pub fn aggregate_updates(&self, updates: &[FLModelUpdate]) -> FLModelUpdate {
        updates.first().cloned().unwrap_or(FLModelUpdate { node_id: "none".into(), model_hash: String::new(), dp_epsilon: 1.0 })
    }
}
