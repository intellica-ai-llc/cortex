//! Adoption Journey — Three-Stage Moore's Chasm Bridge
//!
//! Based on the Octalysis Voluntary Adoption Cascade and Geoffrey Moore's
//! Crossing the Chasm framework. The journey must bridge three specific
//! stages at the 16% chasm boundary:
//!
//!   Stage 1 — Social Proof: visible early adopters create FOMO.
//!     "12 colleagues in Finance already run their reports in Cortex."
//!   Stage 2 — Time Saved Summary: quantified personal ROI.
//!     "You've saved 47 minutes this week using Cortex."
//!   Stage 3 — Risk-Reduction Sandbox: safe trial without commitment.
//!     "Try running the monthly close in a sandbox. One-click rollback."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct AdoptionJourney;

/// A user's current position on the adoption journey.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptionState {
    pub user_id: String,
    pub stage: AdoptionStage,
    pub absorbed_workflows: u64,
    pub total_workflows: u64,
    pub absorption_pct: f64,
    pub time_saved_minutes: u64,
    pub early_adopter_colleagues: u64,
    pub chasm_crossed: bool,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdoptionStage {
    Onboarding,
    EarlyAdopter,
    ChasmBridge,        // at 16% — the critical bridge moment
    EarlyMajority,
    LateMajority,
    FullyAdopted,
}

/// The three bridge interventions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeIntervention {
    pub stage: AdoptionStage,
    pub intervention_type: InterventionType,
    pub message: String,
    pub call_to_action: String,
    pub delivered_at: Option<chrono::DateTime<chrono::Utc>>,
    pub accepted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterventionType {
    SocialProof,
    TimeSavedSummary,
    SandboxDemo,
}

impl AdoptionJourney {
    pub fn new() -> Self { Self }

    /// Determine the user's current adoption stage.
    pub fn determine_stage(absorption_pct: f64, chasm_crossed: bool, early_adopter_count: u64) -> AdoptionStage {
        if absorption_pct >= 80.0 {
            AdoptionStage::FullyAdopted
        } else if absorption_pct >= 50.0 {
            AdoptionStage::LateMajority
        } else if absorption_pct >= 30.0 {
            AdoptionStage::EarlyMajority
        } else if chasm_crossed {
            AdoptionStage::EarlyMajority
        } else if absorption_pct >= 10.0 {
            AdoptionStage::ChasmBridge
        } else if early_adopter_count > 0 {
            AdoptionStage::EarlyAdopter
        } else {
            AdoptionStage::Onboarding
        }
    }

    /// Generate the appropriate bridge intervention for a user's stage.
    ///
    /// At ChasmBridge (16%): deliver all three interventions in sequence.
    ///   1. Social Proof first — "N of your colleagues have already crossed."
    ///   2. Time Saved Summary second — "You've saved X minutes this week."
    ///   3. Sandbox Demo third — "Try it risk-free, one-click rollback."
    pub fn generate_intervention(
        state: &AdoptionState,
    ) -> Vec<BridgeIntervention> {
        match state.stage {
            AdoptionStage::ChasmBridge => vec![
                BridgeIntervention {
                    stage: AdoptionStage::ChasmBridge,
                    intervention_type: InterventionType::SocialProof,
                    message: format!(
                        "{} of your colleagues in {} already run their reports in Cortex. \
                         They save an average of 47 minutes per week.",
                        state.early_adopter_colleagues,
                        "Operations" // would come from org structure
                    ),
                    call_to_action: "See what they're saving".into(),
                    delivered_at: None, accepted: None,
                },
                BridgeIntervention {
                    stage: AdoptionStage::ChasmBridge,
                    intervention_type: InterventionType::TimeSavedSummary,
                    message: format!(
                        "You've saved {} minutes this week by using Cortex instead of \
                         switching between Maximo and Oracle HR.",
                        state.time_saved_minutes
                    ),
                    call_to_action: "View your weekly summary".into(),
                    delivered_at: None, accepted: None,
                },
                BridgeIntervention {
                    stage: AdoptionStage::ChasmBridge,
                    intervention_type: InterventionType::SandboxDemo,
                    message: "Try running the monthly PM schedule in Cortex — in a sandbox. \
                             If you don't like it, one click undoes everything. Zero risk.".into(),
                    call_to_action: "Try it now in sandbox".into(),
                    delivered_at: None, accepted: None,
                },
            ],
            AdoptionStage::EarlyAdopter => vec![
                BridgeIntervention {
                    stage: AdoptionStage::EarlyAdopter,
                    intervention_type: InterventionType::TimeSavedSummary,
                    message: format!("You saved {} minutes this week.", state.time_saved_minutes),
                    call_to_action: "Keep going".into(),
                    delivered_at: None, accepted: None,
                },
            ],
            _ => vec![],
        }
    }

    /// Calculate estimated time saved based on absorbed workflows.
    /// Each absorbed workflow saves approximately 5 minutes vs legacy.
    pub fn estimate_time_saved(absorbed_workflows: u64) -> u64 {
        absorbed_workflows * 5
    }
}
