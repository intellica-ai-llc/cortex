//! Cortex Retire – Cryptographic Decommissioning (v15).
//!
//! When a source system reaches 95%+ absorption, the Retirement Engine
//! captures full‑context evidence, cryptographically signs a Retirement
//! Certificate proving all data, workflows, and compliance requirements
//! have been migrated. The certificate is Merkle‑provenanced and
//! SCITT‑anchored. The legacy system license is cancelled.
//!
//! Key subsystems:
//!   full_context_capture         – screens, rules, interaction patterns
//!   equivalence_replay_engine    – functional equivalence proof
//!   retirement_certificate_signer – cryptographic signing & SCITT

pub mod full_context_capture;
pub mod equivalence_replay_engine;
pub mod retirement_certificate_signer;
