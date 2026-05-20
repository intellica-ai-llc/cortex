//! Cortex Replace – Progressive Weaning Engine (v14).
//!
//! When a source system reaches 80% absorption, the engine begins
//! proactively surfacing Cortex panels when users attempt to open
//! the legacy application. The Weaning Engine tracks which workflows
//! have been migrated and which still run in the source system.
//! The Absorption Score dashboard shows CFOs exactly how much they
//! are saving in legacy license costs.
//!
//! Key subsystems:
//!   absorption_score_dashboard  – real‑time percentage per source
//!   hybrid_rollback_handler     – atomic fallback to legacy UI
//!   license_savings_calculator  – ROI tracking

pub mod absorption_score_dashboard;
pub mod hybrid_rollback_handler;
pub mod license_savings_calculator;
