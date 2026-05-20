//! Cortex Pulse™ – Multi‑Modal Wellness Engine (v5).
//!
//! Fuses EyeScan conjunctiva/pupillometry analysis with vocal
//! biomarker analysis into a single, holistic wellness score.
//! Processed entirely on‑device; only extracted feature vectors
//! (12‑20 floats for voice, 7‑12 for eyes) are stored.
//!
//! Key subsystems:
//!   voice_biomarker_extractor   – acoustic, prosodic, temporal, linguistic, nonlinear
//!   eye_integrator              – pallor, bilirubin, redness, neurological
//!   bayesian_fusion_model       – multi‑modal Bayesian network (Nature Scientific Reports)
//!   baseline_engine             – 30/45/90‑day personal baseline
//!   anomaly_detector            – deviation from baselines

pub mod voice_biomarker_extractor;
pub mod eye_integrator;
pub mod bayesian_fusion_model;
pub mod baseline_engine;
pub mod anomaly_detector;
