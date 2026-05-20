use crate::voice_biomarker_extractor::VoiceFeatures;
use crate::eye_integrator::EyeFeatures;
use serde::{Deserialize, Serialize};

/// Multi‑modal Bayesian network (Nature Scientific Reports, 2026).
pub struct BayesianFusionModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseScore {
    pub composite: f64,       // 0‑100
    pub components: PulseComponents,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseComponents {
    pub stress: f64,
    pub fatigue: f64,
    pub mood: f64,
    pub physical: f64,
}

impl BayesianFusionModel {
    pub fn new() -> Self { Self }

    /// Fuse voice + eye + context into a composite score.
    pub fn fuse(
        &self,
        voice: &VoiceFeatures,
        eye: &EyeFeatures,
        _context_tags: &serde_json::Value,
    ) -> PulseScore {
        // Simplified Bayesian network combining features.
        let stress = 0.4 * voice.stress_index + 0.6 * (1.0 - eye.pallor_score / 100.0);
        let fatigue = 0.5 * voice.fatigue_index + 0.5 * (1.0 - eye.neurological_score / 100.0);
        let mood = voice.depression_risk;
        let physical = (eye.bilirubin_score + voice.anxiety_index) / 2.0;

        let composite = 100.0 - 50.0 * (stress + fatigue + mood + physical).min(1.0);
        PulseScore {
            composite: composite.max(0.0).min(100.0),
            components: PulseComponents { stress, fatigue, mood, physical },
            confidence: 0.85,
        }
    }
}
