//! Behavioural Fidelity Scorer — Legacy Screen Equivalence Measurement
//!
//! Based on Moonello (Feb 2026) "Permanent Hybrid Trap": the Strangler
//! Fig pattern is effective only if the strangulation is completed.
//! Stopping at partial completion creates a Permanent Hybrid state.
//! To prevent this, leadership must define kill-switch criteria.
//!
//! The Fidelity Scorer measures how close each Cortex-generated panel
//! is to the original legacy screen and generates absorption scores
//! that feed into the Weaning Engine. It also detects the "Pareto Stall"
//! at ~80% absorption where migration effort skyrockets and business
//! value appears to diminish — enabling leadership to enforce the
//! kill-switch before the Permanent Hybrid trap closes.

use serde::{Deserialize, Serialize};

pub struct FidelityScorer;

/// Fidelity score for a single reconstructed screen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenFidelity {
    pub source_application: String,
    pub screen_name: String,
    /// Overall fidelity (0-100%). 100% = behaviourally identical.
    pub overall_score: f64,
    /// Sub-scores for different dimensions.
    pub dimensions: FidelityDimensions,
    /// Whether this screen meets the "absorbed" threshold.
    pub meets_threshold: bool,
    pub assessed_at: chrono::DateTime<chrono::Utc>,
    pub recommended_action: FidelityAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FidelityDimensions {
    /// How accurately field positions match the original layout (0-100).
    pub layout_accuracy: f64,
    /// How accurately field labels match the original (0-100).
    pub label_accuracy: f64,
    /// How faithfully validation rules are replicated (0-100).
    pub validation_accuracy: f64,
    /// Whether tab order matches the original.
    pub tab_order_match: f64,
    /// Whether keyboard shortcuts match the original.
    pub keyboard_shortcut_match: f64,
    /// How closely response times match the legacy app.
    pub response_time_parity: f64,
    /// Whether error messages match the original.
    pub error_message_match: f64,
    /// Data completeness: what percentage of legacy fields are absorbed.
    pub data_completeness: f64,
    /// Workflow completeness: what percentage of workflow steps are automated.
    pub workflow_completeness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FidelityAction {
    /// Screen is ready for user-facing replacement.
    Deploy,
    /// Screen needs improvement before deployment.
    Improve { dimensions_to_fix: Vec<String> },
    /// Screen is below minimum threshold; cannot replace yet.
    Blocked { reason: String },
}

/// Absorption health check — detects the Permanent Hybrid Trap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsorptionHealth {
    pub source_application: String,
    pub overall_absorption_pct: f64,
    /// Whether the Pareto Stall is detected (80%+ absorbed but stalled).
    pub pareto_stall_detected: bool,
    /// Days since absorption progress was last recorded.
    pub days_since_last_progress: i64,
    /// Recommended kill-switch date (must be defined before migration starts).
    pub kill_switch_date: Option<chrono::NaiveDate>,
    /// Whether the absorption is on track.
    pub on_track: bool,
}

impl FidelityScorer {
    pub fn new() -> Self { Self }

    /// Score a reconstructed screen against the original legacy screen.
    ///
    /// Dimensions assessed:
    ///   1. Layout accuracy (field positions in rows/columns).
    ///   2. Label accuracy (semantic_label vs original label).
    ///   3. Validation rules (absorbed validation_rules vs observed errors).
    ///   4. Tab order match.
    ///   5. Keyboard shortcut match.
    ///   6. Response time parity.
    ///   7. Error message fidelity.
    ///   8. Data completeness.
    ///   9. Workflow completeness.
    pub fn score(
        &self,
        source: &str,
        screen_name: &str,
        dimensions: FidelityDimensions,
    ) -> ScreenFidelity {
        // Weighted average: layout (20%), labels (10%), validation (20%),
        // data completeness (25%), workflow (15%), remainder (10%).
        let overall = dimensions.layout_accuracy * 0.20
            + dimensions.label_accuracy * 0.10
            + dimensions.validation_accuracy * 0.20
            + dimensions.data_completeness * 0.25
            + dimensions.workflow_completeness * 0.15
            + dimensions.tab_order_match * 0.03
            + dimensions.keyboard_shortcut_match * 0.02
            + dimensions.response_time_parity * 0.03
            + dimensions.error_message_match * 0.02;

        let meets_threshold = overall >= 90.0; // 90% threshold for deployment.

        let recommended_action = if overall >= 95.0 {
            FidelityAction::Deploy
        } else if overall >= 70.0 {
            let mut dims_to_fix = Vec::new();
            if dimensions.layout_accuracy < 90.0 { dims_to_fix.push("layout_accuracy".into()); }
            if dimensions.validation_accuracy < 90.0 { dims_to_fix.push("validation_accuracy".into()); }
            if dimensions.data_completeness < 90.0 { dims_to_fix.push("data_completeness".into()); }
            FidelityAction::Improve { dimensions_to_fix: dims_to_fix }
        } else {
            FidelityAction::Blocked {
                reason: format!("Overall fidelity {:.0}% below minimum 70% threshold", overall),
            }
        };

        ScreenFidelity {
            source_application: source.to_string(),
            screen_name: screen_name.to_string(),
            overall_score: overall,
            dimensions,
            meets_threshold,
            assessed_at: chrono::Utc::now(),
            recommended_action,
        }
    }

    /// Check for the Permanent Hybrid Trap (Moonello pattern).
    ///
    /// The Pareto Stall occurs when ~80% of features are absorbed
    /// but progress stalls. The remaining 20% (core business logic)
    /// requires exponentially more effort. If left unchecked, the
    /// organisation enters Permanent Hybrid: supporting two systems
    /// indefinitely.
    pub fn check_absorption_health(
        &self,
        source: &str,
        absorption_pct: f64,
        days_since_last_progress: i64,
    ) -> AbsorptionHealth {
        let pareto_stall = absorption_pct >= 75.0 && absorption_pct < 95.0
            && days_since_last_progress > 30;

        AbsorptionHealth {
            source_application: source.to_string(),
            overall_absorption_pct: absorption_pct,
            pareto_stall_detected: pareto_stall,
            days_since_last_progress,
            kill_switch_date: Some(chrono::Utc::now().date_naive() + chrono::Duration::days(180)),
            on_track: !pareto_stall,
        }
    }
}
