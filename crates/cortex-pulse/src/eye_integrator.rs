use serde::{Deserialize, Serialize};

/// Integrates with EyeScan pipeline for conjunctiva/pupillometry.
pub struct EyeIntegrator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeFeatures {
    pub pallor_score: f64,
    pub bilirubin_score: f64,
    pub redness_score: f64,
    pub neurological_score: f64,
}

impl EyeIntegrator {
    pub fn new() -> Self { Self }

    /// Obtain latest eye features (placeholder).
    pub async fn get_latest(&self, _user_id: &str) -> EyeFeatures {
        EyeFeatures {
            pallor_score: 82.0,
            bilirubin_score: 10.0,
            redness_score: 5.0,
            neurological_score: 95.0,
        }
    }
}
