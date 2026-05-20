//! Cortex Absorb — Progressive Data Absorption Engine (v12).
//!
//! Just‑in‑time field absorption driven by observation frequency.
//! Agent‑safe branching via hybrid CoW+MoR strategy evaluated
//! against BranchBench workloads. Write approval gate based on
//! the DZone Commit Boundary pattern for regulated industries.
//!
//! Key subsystems:
//!   just_in_time_absorption  — frequency‑driven field absorption
//!   write_approval_gate      — HITL approval for regulated writes
//!   branch_router            — BranchBench‑informed workload routing
//!   ephemeral_pool           — Neon/Stripe sub‑350ms branch provisioning

pub mod just_in_time_absorption;
pub mod write_approval_gate;
pub mod branch_router;
pub mod ephemeral_pool;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top‑level absorption orchestrator.
pub struct AbsorptionEngine {
    pub jit: Arc<just_in_time_absorption::JustInTimeAbsorption>,
    pub approval_gate: Arc<write_approval_gate::WriteApprovalGate>,
    pub branch_router: Arc<branch_router::BranchRouter>,
    pub ephemeral_pool: Arc<ephemeral_pool::EphemeralBranchPool>,
    /// Active absorption sessions per source system.
    active_sessions: RwLock<std::collections::HashMap<String, AbsorptionSession>>,
}

#[derive(Debug, Clone)]
pub struct AbsorptionSession {
    pub source: String,
    pub phase: AbsorptionPhase,
    pub fields_absorbed: u64,
    pub fields_total: u64,
    pub absorption_pct: f64,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbsorptionPhase {
    Observing,
    Mirroring,
    Absorbing,
    Genesis,
    Replacing,
    Retired,
}

impl AbsorptionEngine {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            jit: Arc::new(just_in_time_absorption::JustInTimeAbsorption::new(pool.clone())),
            approval_gate: Arc::new(write_approval_gate::WriteApprovalGate::new()),
            branch_router: Arc::new(branch_router::BranchRouter::new()),
            ephemeral_pool: Arc::new(ephemeral_pool::EphemeralBranchPool::new(pool, 10)),
            active_sessions: RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Begin absorbing a source system.
    pub async fn begin_absorption(&self, source: &str, total_fields: u64) {
        self.active_sessions.write().await.insert(source.to_string(), AbsorptionSession {
            source: source.to_string(),
            phase: AbsorptionPhase::Absorbing,
            fields_absorbed: 0,
            fields_total: total_fields,
            absorption_pct: 0.0,
            started_at: chrono::Utc::now(),
        });
    }

    /// Get absorption progress for a source.
    pub async fn progress(&self, source: &str) -> Option<AbsorptionSession> {
        self.active_sessions.read().await.get(source).cloned()
    }
}
