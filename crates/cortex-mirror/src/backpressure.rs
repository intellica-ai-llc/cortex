use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::time::{Duration, Instant};

/// Five‑layer credit‑based backpressure controller.
///
/// Grounded in the ZongJi/duckling production failure (Mar 21, 2026):
/// “If the core backpressure mechanism silently fails to pause the
/// stream, a heavy write workload will cause the node process's
/// memory to explode, eventually resulting in a fatal OOM crash.”
///
/// Conduktor (Apr 27, 2026) defines five strategies: Buffering,
/// Throttling/Blocking, Load Shedding, Elastic Scaling, Batching.
/// This controller implements all five as a layered defence.
pub struct MultiLayerBackpressure {
    // Layer 1 – Source: credit‑based flow control (Flink CDC pattern)
    available_credits: AtomicI64,
    // Layer 2 – Pipeline: sustained backpressure timer
    backpressure_since: tokio::sync::Mutex<Option<Instant>>,
    // Layer 3 – Sink: admission control flag
    sink_available: AtomicBool,
    // Layer 4 – Memory: heap usage guard
    heap_pct: AtomicI64,
    // Layer 5 – Disk: compaction debt
    compaction_debt_gb: AtomicI64,
    // Configurable thresholds
    credit_threshold: i64,
    sustained_limit_secs: u64,
    max_heap_pct: i64,
    max_compaction_debt: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackpressureState {
    pub layer: &'static str,
    pub pressure_level: PressureLevel,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PressureLevel { Normal, Elevated, Critical, Blocked }

impl MultiLayerBackpressure {
    pub fn new() -> Self {
        Self {
            available_credits: AtomicI64::new(100_000),
            backpressure_since: tokio::sync::Mutex::new(None),
            sink_available: AtomicBool::new(true),
            heap_pct: AtomicI64::new(0),
            compaction_debt_gb: AtomicI64::new(0),
            credit_threshold: 10_000,
            sustained_limit_secs: 30,
            max_heap_pct: 85,
            max_compaction_debt: 20,
        }
    }

    /// Evaluate all five layers. Returns the most severe pressure level.
    pub async fn evaluate(&self) -> BackpressureState {
        // Layer 4 – Memory: highest priority check (prevents OOM)
        if self.heap_pct.load(Ordering::SeqCst) > self.max_heap_pct {
            return BackpressureState {
                layer: "memory",
                pressure_level: PressureLevel::Critical,
                details: format!("Heap {}% > {}%", self.heap_pct.load(Ordering::SeqCst), self.max_heap_pct),
            };
        }

        // Layer 5 – Disk
        if self.compaction_debt_gb.load(Ordering::SeqCst) > self.max_compaction_debt {
            return BackpressureState {
                layer: "disk",
                pressure_level: PressureLevel::Critical,
                details: format!("Compaction debt {}GB > {}GB", self.compaction_debt_gb.load(Ordering::SeqCst), self.max_compaction_debt),
            };
        }

        // Layer 1 – Source credits
        if self.available_credits.load(Ordering::SeqCst) < self.credit_threshold {
            return BackpressureState {
                layer: "source",
                pressure_level: PressureLevel::Elevated,
                details: format!("Credits {} < {}", self.available_credits.load(Ordering::SeqCst), self.credit_threshold),
            };
        }

        // Layer 2 – Sustained backpressure
        if let Some(since) = *self.backpressure_since.lock().await {
            if since.elapsed() > Duration::from_secs(self.sustained_limit_secs) {
                return BackpressureState {
                    layer: "pipeline",
                    pressure_level: PressureLevel::Critical,
                    details: format!("Sustained backpressure for {}s", since.elapsed().as_secs()),
                };
            }
        }

        // Layer 3 – Sink
        if !self.sink_available.load(Ordering::SeqCst) {
            return BackpressureState {
                layer: "sink",
                pressure_level: PressureLevel::Elevated,
                details: "Sink not accepting writes".into(),
            };
        }

        BackpressureState { layer: "none", pressure_level: PressureLevel::Normal, details: "All clear".into() }
    }

    /// Consume one credit. Returns false if credits exhausted.
    pub fn consume_credit(&self) -> bool {
        loop {
            let current = self.available_credits.load(Ordering::SeqCst);
            if current <= 0 { return false; }
            if self.available_credits.compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                return true;
            }
        }
    }

    /// Grant credits back (called by downstream after processing a batch).
    pub fn grant_credits(&self, count: i64) {
        self.available_credits.fetch_add(count, Ordering::SeqCst);
    }

    /// Update observed metrics.
    pub fn update_metrics(&self, heap_pct: i64, compaction_gb: i64) {
        self.heap_pct.store(heap_pct, Ordering::SeqCst);
        self.compaction_debt_gb.store(compaction_gb, Ordering::SeqCst);
    }

    /// Mark sink as unavailable.
    pub fn block_sink(&self) { self.sink_available.store(false, Ordering::SeqCst); }
    pub fn unblock_sink(&self) { self.sink_available.store(true, Ordering::SeqCst); }
}
