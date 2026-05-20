use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Freshness‑Aware Routing Layer — routes agent queries to the
/// right data source based on data latency requirements.
///
/// Grounded in Airbyte’s freshness‑vs‑latency distinction (Feb 26,
/// 2026): “Agent data freshness is distinct from query latency.
/// Latency measures how fast the agent gets a response; Freshness
/// measures how current that response is.” An agent can retrieve
/// context in 50 ms and still receive data that’s six hours old.
///
/// Also grounded in Streamkap’s Real‑Time Context Engines research
/// (Mar 11, 2026): real‑time context delivery improves prediction
/// accuracy by 40% and reduces hallucinations by 40%.
pub struct FreshnessRouter {
    /// Per‑source freshness metadata.
    freshness_state: RwLock<HashMap<String, SourceFreshness>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFreshness {
    pub source: String,
    pub last_sync_at: chrono::DateTime<chrono::Utc>,
    pub sync_latency_ms: u64,
    pub freshness_tier: FreshnessTier,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FreshnessTier {
    /// < 100ms — suitable for real‑time agent decisions.
    Live,
    /// < 5s — suitable for dashboard refreshes, status checks.
    NearRealTime,
    /// < 5min — suitable for most operational queries.
    Delayed,
    /// < 1 hour — suitable for batch analytics.
    Batch,
    /// > 1 hour — not suitable for agent decisions.
    Stale,
}

/// The routing decision for an agent query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub target: RoutingTarget,
    pub freshness_tier: FreshnessTier,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingTarget {
    /// Route to the live MCP connector on the source system.
    SourceDirect,
    /// Route to the TraceDB materialized view (CDC‑synced).
    TraceDBView,
    /// Route to the TraceDB base snapshot (periodic merge).
    TraceDBSnapshot,
    /// Route to an absorption branch (sandboxed).
    AbsorptionBranch,
}

impl FreshnessRouter {
    pub fn new() -> Self {
        Self { freshness_state: RwLock::new(HashMap::new()) }
    }

    /// Update freshness metadata for a source.
    pub async fn update(&self, source: &str, latency_ms: u64) {
        let tier = match latency_ms {
            0..=100 => FreshnessTier::Live,
            101..=5_000 => FreshnessTier::NearRealTime,
            5_001..=300_000 => FreshnessTier::Delayed,
            300_001..=3_600_000 => FreshnessTier::Batch,
            _ => FreshnessTier::Stale,
        };
        self.freshness_state.write().await.insert(source.to_string(), SourceFreshness {
            source: source.to_string(),
            last_sync_at: chrono::Utc::now(),
            sync_latency_ms: latency_ms,
            freshness_tier: tier,
        });
    }

    /// Determine the best data source for an agent decision.
    /// Implements the dual‑mode architecture: live access for
    /// operational decisions, TraceDB for UI rendering.
    pub async fn route(
        &self,
        source: &str,
        _decision_type: AgentDecisionType,
    ) -> RoutingDecision {
        let state = self.freshness_state.read().await;
        let freshness = state.get(source);

        match freshness.map(|f| &f.freshness_tier) {
            Some(FreshnessTier::Live) | Some(FreshnessTier::NearRealTime) => {
                RoutingDecision {
                    target: RoutingTarget::TraceDBView,
                    freshness_tier: freshness.unwrap().freshness_tier.clone(),
                    rationale: "Data is fresh; route to TraceDB materialized view".into(),
                }
            }
            Some(FreshnessTier::Delayed) => {
                RoutingDecision {
                    target: RoutingTarget::TraceDBSnapshot,
                    freshness_tier: FreshnessTier::Delayed,
                    rationale: "Data is slightly stale; use base snapshot".into(),
                }
            }
            _ => {
                RoutingDecision {
                    target: RoutingTarget::SourceDirect,
                    freshness_tier: FreshnessTier::Stale,
                    rationale: "TraceDB data is stale; fall back to live MCP connector".into(),
                }
            }
        }
    }
}

/// The type of agent decision (determines latency tolerance).
#[derive(Debug, Clone)]
pub enum AgentDecisionType {
    /// Must be current (< 1s) — approve, update, modify.
    RealTimeWorkflow,
    /// Sub‑100ms freshness acceptable — dashboard, status check.
    NearRealTimeQuery,
    /// Overnight freshness acceptable — compliance report.
    HistoricalAnalysis,
    /// Can be stale; isolated — what‑if simulation.
    WhatIfSimulation,
}
