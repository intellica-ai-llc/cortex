//! Cortex ObservabilityStack — OpenTelemetry‑native monitoring.
//!
//! Auto‑instrumented inference, tool, memory, effect, decision,
//! federation spans. Token usage, latency, error rates, tool call
//! patterns metrics. Pattern‑based anomaly detection on agent
//! behaviour. Outcome metrics for Results‑as‑a‑Service billing.

pub mod spans;
pub mod metrics;
pub mod anomaly;
pub mod outcome_metrics;

use std::sync::Arc;

pub struct ObservabilityStack {
    pub spans: Arc<spans::SpanEmitter>,
    pub metrics: Arc<metrics::MetricCollector>,
    pub anomaly: Arc<anomaly::AnomalyDetector>,
    pub outcome: Arc<outcome_metrics::OutcomeMetrics>,
}

impl ObservabilityStack {
    pub fn new() -> Self {
        Self {
            spans: Arc::new(spans::SpanEmitter::new()),
            metrics: Arc::new(metrics::MetricCollector::new()),
            anomaly: Arc::new(anomaly::AnomalyDetector::new()),
            outcome: Arc::new(outcome_metrics::OutcomeMetrics::new()),
        }
    }
}
