//! Cortex DreamEngine — Nightly Consolidation & Self‑Improvement.
//!
//! During nightly deep sleep, the Dream Engine reads new decision
//! traces accumulated during the day, transforms episodic memories
//! into consolidated semantic knowledge, resolves contradictions,
//! prunes low‑importance information, and compresses historical
//! data via hierarchical summarisation (10:1 ratio).
//!
//! All dream cycle activity is written into an append‑only,
//! ed25519‑signed dream journal for provenance.

pub mod scheduler;
pub mod consolidator;
pub mod contradiction;
pub mod compressor;
pub mod pruner;
pub mod journal;

use std::sync::Arc;

pub struct DreamEngine {
    pub scheduler: Arc<scheduler::DreamCycleScheduler>,
    pub consolidator: Arc<consolidator::Consolidator>,
    pub contradiction_resolver: Arc<contradiction::ContradictionResolver>,
    pub compressor: Arc<compressor::Compressor>,
    pub pruner: Arc<pruner::Pruner>,
    pub journal_writer: Arc<journal::JournalWriter>,
}

impl DreamEngine {
    pub fn new(signing_key: [u8; 32]) -> Self {
        Self {
            scheduler: Arc::new(scheduler::DreamCycleScheduler::new()),
            consolidator: Arc::new(consolidator::Consolidator::new()),
            contradiction_resolver: Arc::new(contradiction::ContradictionResolver::new()),
            compressor: Arc::new(compressor::Compressor::new()),
            pruner: Arc::new(pruner::Pruner::new()),
            journal_writer: Arc::new(journal::JournalWriter::new(signing_key)),
        }
    }

    /// Run the full nightly dream cycle.
    pub async fn dream(&self, memory: &cortex_memory::MemorySubstrate) {
        if !self.scheduler.should_dream() {
            return;
        }
        tracing::info!("Dream cycle starting");
        self.consolidator.consolidate(&memory.episodic, &memory.semantic).await;
        self.contradiction_resolver.resolve(&memory.semantic).await;
        self.compressor.compress(&memory.semantic).await;
        self.pruner.prune(&memory.episodic).await;
        let entry = self.journal_writer.sign_entry("Dream cycle completed");
        memory.provenance_index.append(&entry).await;
        tracing::info!("Dream cycle complete");
    }
}
