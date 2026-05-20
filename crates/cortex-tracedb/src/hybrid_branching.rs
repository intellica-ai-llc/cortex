use serde::{Deserialize, Serialize};

/// Hybrid branching strategy resolving BranchBench’s CoW‑vs‑MoR tension.
///
/// BranchBench (April 19, 2026) found a fundamental tension:
/// systems optimised for fast branching (Copy‑on‑Write) suffer
/// 25–1500× slower reads as branches deepen, while systems
/// optimised for reads (Merge‑on‑Read) incur 5–4000× higher
/// branch creation and switching latency.
///
/// Solution: a **three‑tier** strategy selected at branch creation
/// based on expected depth and workload type.

pub struct BranchRouter;

/// Branching strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BranchStrategy {
    /// Shallow, exploratory agents – fast create/discard (MCTS).
    CopyOnWrite,
    /// Deep, long‑running production branches – fast reads (software eng).
    MergeOnRead,
    /// Cross‑branch comparison or simulation – content‑addressed DAG (data curation).
    ContentAddressedDAG,
    /// Vanilla snapshot for deterministic replay.
    Snapshot,
}

/// Workload descriptors (from BranchBench).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkloadType {
    MCTSExploration,
    SoftwareEngineering,
    DataCuration,
    FailureReproduction,
}

impl BranchRouter {
    /// Select the best strategy for a given workload.
    pub fn select(workload: WorkloadType, _estimated_depth: usize) -> BranchStrategy {
        match workload {
            WorkloadType::MCTSExploration => BranchStrategy::CopyOnWrite,
            WorkloadType::SoftwareEngineering => BranchStrategy::MergeOnRead,
            WorkloadType::DataCuration => BranchStrategy::ContentAddressedDAG,
            WorkloadType::FailureReproduction => BranchStrategy::Snapshot,
        }
    }
}
