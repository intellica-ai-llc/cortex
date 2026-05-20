//! Cortex Self‑Validate — Autonomous Technical Due‑Diligence Engine.
//!
//! Based on the Ultra Lab Technical Transparency Manifesto (Apr 2026):
//! "How Do We Prove We Actually Do AI? … In 2026, open any startup's
//! website and you'll see 'AI‑Powered' plastered everywhere."
//! [reference:13]
//!
//! The Agathon AI Due Diligence framework warns of "The Validation
//! Vacuum" and "The Demo‑to‑Production Gap" as the two fatal flaws
//! that kill AI startup acquisitions. [reference:14]
//!
//! Cortex Self‑Validate closes both gaps:
//!   1. Runs all 12 validation experiments (X1‑X12) in sequence.
//!   2. Produces empirically verifiable pass/fail results per experiment.
//!   3. Signs every result with Ed25519 (non‑repudiable).
//!   4. Anchors the aggregate Merkle root to SCITT.
//!
//! One command. Zero human intervention. Mathematical proof.

pub mod self_validator;
pub mod result_aggregator;
pub mod report_generator;
pub mod dell_blueprint_generator;
