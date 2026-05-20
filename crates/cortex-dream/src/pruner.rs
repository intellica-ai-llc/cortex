use serde::{Deserialize, Serialize};

/// Importance‑weighted decay (Ebbinghaus forgetting curves).
pub struct Pruner;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruneResult {
    pub traces_pruned: u64,
    pub traces_retained: u64,
    pub bytes_freed: u64,
}

impl Pruner {
    pub fn new() -> Self { Self }

    /// Prune episodic traces based on importance and age.
    ///
    /// Ebbinghaus forgetting curve with reinforcement:
    ///   - Each trace has an initial importance weight.
    ///   - Weight decays exponentially with age unless the trace is
    ///     "reinforced" by being accessed again.
    ///   - When weight falls below a threshold, the trace is pruned.
    ///   - Regulatory‑required traces (audit, compliance) are exempt.
    pub async fn prune(
        &self,
        episodic: &cortex_memory::episodic::EpisodicStore,
    ) -> PruneResult {
        let (pruned, retained) = episodic.prune_aged(0.1).await;
        PruneResult {
            traces_pruned: pruned as u64,
            traces_retained: retained as u64,
            bytes_freed: pruned as u64 * 500, // estimate ~500 bytes per trace
        }
    }
}
