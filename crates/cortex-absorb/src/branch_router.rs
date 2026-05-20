use serde::{Deserialize, Serialize};

/// Branch‑Strategy Router — BranchBench‑informed workload routing.
///
/// BranchBench (Ang et al., arXiv:2604.17180, Apr 2026) evaluated
/// five representative agentic workloads against Neon, DoltgreSQL,
/// Tiger Data, Xata, and PostgreSQL baselines. It found "a
/// fundamental tension: systems optimized for fast branching
/// suffer up to 5–4000× slower reads as branches deepen, while
/// systems optimized for fast data operations incur 25–1500×
/// higher branch creation and switching latency."
///
/// No single storage strategy passes all five workloads. The
/// solution: a three‑tier strategy selected at branch creation
/// based on estimated depth and workload type. This aligns with
/// the Cortex v9 TraceDB hybrid branching model.
pub struct BranchRouter;

/// The five BranchBench‑derived workload types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BranchWorkload {
    /// Monte‑Carlo tree search — many shallow branches, fast create/discard.
    MCTSExploration,
    /// Agentic software engineering — moderate depth, moderate writes.
    SoftwareEngineering,
    /// Data curation / simulation — wide fan‑out, cross‑branch comparison.
    DataCurationSimulation,
    /// Failure reproduction — deterministic replay, snapshot‑based.
    FailureReproduction,
    /// What‑if simulation — isolated sandbox, may be deep.
    WhatIfSimulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BranchStrategy {
    /// Shallow, exploratory — fast create/discard, CoW at storage layer (Neon/Xata).
    CopyOnWrite { max_depth: usize },
    /// Production, long‑running — fast reads, Merge‑on‑Read.
    MergeOnRead,
    /// Cross‑branch comparison — content‑addressed Merkle DAG (Dolt pattern).
    ContentAddressedDAG,
    /// Deterministic replay — vanilla PostgreSQL snapshot.
    Snapshot,
}

impl BranchRouter {
    pub fn new() -> Self { Self {} }

    /// Select the optimal branching strategy for a workload.
    ///
    /// Decision matrix (from BranchBench evaluation):
    ///   MCTS: shallow (< 5 levels), heavy create/discard → CoW (Neon/Xata).
    ///   Software Engineering: moderate depth (< 20), frequent reads → MoR.
    ///   Data Curation: wide fan‑out, cross‑branch joins → ContentAddressedDAG.
    ///   Failure Reproduction: exact state replay → Snapshot.
    ///   What‑If Simulation: may be deep, isolated → CoW if shallow else MoR.
    pub fn select(
        &self,
        workload: &BranchWorkload,
        estimated_depth: usize,
    ) -> BranchStrategy {
        match workload {
            BranchWorkload::MCTSExploration => {
                BranchStrategy::CopyOnWrite { max_depth: estimated_depth.max(1) }
            }
            BranchWorkload::SoftwareEngineering => {
                if estimated_depth < 10 {
                    BranchStrategy::CopyOnWrite { max_depth: estimated_depth }
                } else {
                    BranchStrategy::MergeOnRead
                }
            }
            BranchWorkload::DataCurationSimulation => {
                BranchStrategy::ContentAddressedDAG
            }
            BranchWorkload::FailureReproduction => {
                BranchStrategy::Snapshot
            }
            BranchWorkload::WhatIfSimulation => {
                if estimated_depth < 15 {
                    BranchStrategy::CopyOnWrite { max_depth: estimated_depth }
                } else {
                    BranchStrategy::MergeOnRead
                }
            }
        }
    }

    /// Generate a descriptive name for a branch based on workload.
    pub fn branch_name(
        source: &str,
        workload: &BranchWorkload,
    ) -> String {
        let prefix = match workload {
            BranchWorkload::MCTSExploration => "mcts",
            BranchWorkload::SoftwareEngineering => "eng",
            BranchWorkload::DataCurationSimulation => "curation",
            BranchWorkload::FailureReproduction => "replay",
            BranchWorkload::WhatIfSimulation => "whatif",
        };
        format!("{}_{}_{}", prefix, source, uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0"))
    }
}
