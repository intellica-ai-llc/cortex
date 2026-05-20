//! LFAB Sleep — WoVR‑Safe Dream Engine & Cross‑Phase Consolidation.
//!
//! Extends the DreamEngine with on‑device nightly consolidation.
//! Cross‑Phase Consolidation Pass reads decision traces from Observe
//! and Mirror phases, identifies behavioral patterns, consolidates
//! them into procedural memory, and updates the Cortex Forge skill
//! library with crystallised workflows.

pub mod dream_cycle;
pub mod cross_phase_consolidation;

use std::sync::Arc;

pub struct LFABSleep {
    pub dream: Arc<dream_cycle::WoVRSafeDream>,
    pub consolidation: Arc<cross_phase_consolidation::CrossPhaseConsolidation>,
}

impl LFABSleep {
    pub fn new() -> Self {
        Self {
            dream: Arc::new(dream_cycle::WoVRSafeDream::new()),
            consolidation: Arc::new(cross_phase_consolidation::CrossPhaseConsolidation::new()),
        }
    }
}
