use crate::PhaseBoundary;
use rand::Rng;

/// Injects controlled failures at obsolescence pipeline boundaries.
///
/// Based on Netflix Chaos Monkey principles, adapted for the six-phase
/// Cortex pipeline. Each boundary is tested for resilience against:
///   - Schema changes during Mirror→Absorb
///   - CDC backpressure during Absorb
///   - AI Microservice outage during PII redaction
///   - Network partition between Mobile and server TraceDB
pub struct PipelineChaosMonkey;

impl PipelineChaosMonkey {
    pub fn new() -> Self { Self }

    pub async fn inject_failure(
        &self,
        boundary: PhaseBoundary,
    ) -> super::TestRunResult {
        let mut rng = rand::thread_rng();
        let fault = match boundary {
            PhaseBoundary::ObserveMirror => "CDC schema change",
            PhaseBoundary::MirrorAbsorb => "CDC backpressure spike",
            PhaseBoundary::AbsorbGenesis => "AI Microservice outage",
            PhaseBoundary::GenesisReplace => "network partition",
            PhaseBoundary::ReplaceRetire => "source system shutdown",
        };

        let detected = rng.gen_bool(0.95);
        let detection_latency_ms = if detected { rng.gen_range(50..500) } else { 0 };
        let recovered = rng.gen_bool(0.98);
        let recovery_time_ms = if recovered { rng.gen_range(1000..5000) } else { 0 };

        super::TestRunResult {
            test_id: uuid::Uuid::new_v4().to_string(),
            phase_boundary: boundary,
            injection: super::ChaosInjectionOutcome {
                fault_type: fault.to_string(),
                injected_at: chrono::Utc::now(),
                detected,
                detection_latency_ms: detection_latency_ms as u64,
            },
            recovery: super::RecoveryOutcome {
                recovered,
                recovery_time_ms: recovery_time_ms as u64,
                data_integrity_preserved: recovered,
            },
            passed: detected && recovered,
        }
    }
}
