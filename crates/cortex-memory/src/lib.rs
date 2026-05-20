//! Cortex MemorySubstrate — Persistent, Searchable, Decay‑Aware.
//!
//! Eight‑layer memory architecture supporting the full agent lifecycle:
//!   L1 – Episodic (event log with temporal chain)
//!   L2 – Semantic (consolidated facts with ontology links)
//!   L3 – Procedural (skills, workflows, tool patterns)
//!   L5 – Federated (CRDT‑backed cross‑instance sharing)
//!   L7 – Provenance Index (self‑anchored, Merkle‑proofed)
//!   UX – Preference Store (per‑user interface preferences)
//!
//! With governance (tri‑path router), coherency (MESI + CRDT),
//! and Merkle integrity.

pub mod layer;
pub mod episodic;
pub mod semantic;
pub mod procedural;
pub mod federated;
pub mod provenance_index;
pub mod ux_preference_store;
pub mod decay;
pub mod governance;
pub mod coherency;
pub mod merkle;

use std::sync::Arc;

pub struct MemorySubstrate {
    pub episodic: Arc<episodic::EpisodicStore>,
    pub semantic: Arc<semantic::SemanticStore>,
    pub procedural: Arc<procedural::ProceduralStore>,
    pub federated: Arc<federated::FederatedStore>,
    pub provenance_index: Arc<provenance_index::ProvenanceIndex>,
    pub ux_preferences: Arc<ux_preference_store::UXPreferenceStore>,
    pub decay_manager: Arc<decay::DecayManager>,
    pub governance: Arc<governance::GovernanceRouter>,
    pub coherency: Arc<coherency::CoherencyManager>,
    pub merkle_verifier: Arc<merkle::MerkleIntegrity>,
}

impl MemorySubstrate {
    pub fn new() -> Self {
        Self {
            episodic: Arc::new(episodic::EpisodicStore::new()),
            semantic: Arc::new(semantic::SemanticStore::new()),
            procedural: Arc::new(procedural::ProceduralStore::new()),
            federated: Arc::new(federated::FederatedStore::new()),
            provenance_index: Arc::new(provenance_index::ProvenanceIndex::new()),
            ux_preferences: Arc::new(ux_preference_store::UXPreferenceStore::new()),
            decay_manager: Arc::new(decay::DecayManager::new()),
            governance: Arc::new(governance::GovernanceRouter::new()),
            coherency: Arc::new(coherency::CoherencyManager::new()),
            merkle_verifier: Arc::new(merkle::MerkleIntegrity::new()),
        }
    }
}
