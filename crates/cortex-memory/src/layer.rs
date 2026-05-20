use serde::{Deserialize, Serialize};

/// Memory layer enumeration (v1 eight‑layer model).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MemoryLayer {
    Episodic,       // L1 – event log with temporal chain
    Semantic,       // L2 – consolidated facts with ontology links
    Procedural,     // L3 – skills, workflows, tool patterns
    ShortTerm,      // L4 – working memory (current session)
    Federated,      // L5 – CRDT‑backed cross‑instance sharing
    LongTerm,       // L6 – archived, cold storage
    Provenance,     // L7 – self‑anchored, Merkle‑proofed audit log
    UXPreference,   // L8 – per‑user interface preferences
}

impl MemoryLayer {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Episodic => "episodic",
            Self::Semantic => "semantic",
            Self::Procedural => "procedural",
            Self::ShortTerm => "short_term",
            Self::Federated => "federated",
            Self::LongTerm => "long_term",
            Self::Provenance => "provenance",
            Self::UXPreference => "ux_preference",
        }
    }
}
