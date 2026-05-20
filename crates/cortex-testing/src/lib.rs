//! Cortex Testing — Pipeline Chaos Monkey & Integration Harness.
//!
//! Injects failures at every phase boundary of the Obsolescence
//! Pipeline: schema change during Mirror→Absorb, CDC backpressure
//! during Absorb, AI Microservice outage during PII redaction,
//! network partition between Mobile Brain and server TraceDB.
//! Each phase transition must survive the chaos monkey before
//! deployment.

pub mod pipeline_chaos_monkey;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TestHarness {
    pub chaos: Arc<pipeline_chaos_monkey::PipelineChaosMonkey>,
    results: RwLock<Vec<TestRunResult>>,
}

#[derive(Debug, Clone)]
pub struct TestRunResult {
    pub test_id: String,
    pub phase_boundary: PhaseBoundary,
    pub injection: ChaosInjectionOutcome,
    pub recovery: RecoveryOutcome,
    pub passed: bool,
}

#[derive(Debug, Clone)]
pub enum PhaseBoundary {
    ObserveMirror,
    MirrorAbsorb,
    AbsorbGenesis,
    GenesisReplace,
    ReplaceRetire,
}

#[derive(Debug, Clone)]
pub struct ChaosInjectionOutcome {
    pub fault_type: String,
    pub injected_at: chrono::DateTime<chrono::Utc>,
    pub detected: bool,
    pub detection_latency_ms: u64,
}

#[derive(Debug, Clone)]
pub struct RecoveryOutcome {
    pub recovered: bool,
    pub recovery_time_ms: u64,
    pub data_integrity_preserved: bool,
}

impl TestHarness {
    pub fn new() -> Self {
        Self {
            chaos: Arc::new(pipeline_chaos_monkey::PipelineChaosMonkey::new()),
            results: RwLock::new(Vec::new()),
        }
    }

    pub async fn run_boundary_test(&self, boundary: PhaseBoundary) -> TestRunResult {
        self.chaos.inject_failure(boundary).await
    }
}
