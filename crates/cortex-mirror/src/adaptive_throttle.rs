use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Adaptive micro‑batching throttle — switches between streaming,
/// micro‑batch, and bulk‑batch modes based on system pressure.
///
/// Based on the IEEE Trans. Reliability adaptive mini‑batching
/// strategy (Wu et al., March 2026): dynamically adjusts batch
/// size and interval to maintain throughput under varying load.
/// Conduktor’s “Batching and Windowing” strategy confirms that
/// processing multiple events together improves throughput when
/// per‑event overhead dominates.
pub struct AdaptiveThrottle {
    /// Current processing mode.
    mode: RwLock<ProcessingMode>,
    /// When the current mode was entered.
    mode_entered_at: RwLock<Instant>,
    /// Consecutive pressure evaluations.
    pressure_history: RwLock<Vec<PressureReading>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingMode {
    /// Per‑row streaming, sub‑100ms latency (default).
    Streaming,
    /// 1‑second micro‑batches (moderate pressure).
    MicroBatch { batch_interval_ms: u64 },
    /// Full‑table snapshot batches, hourly/daily (high pressure / initial load).
    BulkBatch { batch_interval_secs: u64 },
}

#[derive(Debug, Clone)]
struct PressureReading {
    level: super::backpressure::PressureLevel,
    timestamp: Instant,
}

impl AdaptiveThrottle {
    pub fn new() -> Self {
        Self {
            mode: RwLock::new(ProcessingMode::Streaming),
            mode_entered_at: RwLock::new(Instant::now()),
            pressure_history: RwLock::new(Vec::new()),
        }
    }

    /// Evaluate system pressure and switch mode if needed.
    /// Conduktor’s principle: throttling slows producers when
    /// consumers can’t keep up — preventing unbounded queue growth.
    pub async fn evaluate(&self, pressure: &super::backpressure::BackpressureState) -> ProcessingMode {
        let mut history = self.pressure_history.write().await;
        history.push(PressureReading { level: pressure.pressure_level.clone(), timestamp: Instant::now() });
        // Keep last 30 readings.
        if history.len() > 30 { history.remove(0); }

        let critical_count = history.iter().filter(|r| r.pressure_level == super::backpressure::PressureLevel::Critical).count();
        let elevated_count = history.iter().filter(|r| r.pressure_level == super::backpressure::PressureLevel::Elevated).count();
        let mut current = self.mode.write().await;

        match pressure.pressure_level {
            super::backpressure::PressureLevel::Critical => {
                // Switch to micro‑batch or bulk‑batch to throttle ingestion.
                if critical_count >= 3 {
                    *current = ProcessingMode::BulkBatch { batch_interval_secs: 10 };
                } else {
                    *current = ProcessingMode::MicroBatch { batch_interval_ms: 2000 };
                }
            }
            super::backpressure::PressureLevel::Elevated => {
                if *current == ProcessingMode::Streaming && elevated_count >= 3 {
                    *current = ProcessingMode::MicroBatch { batch_interval_ms: 1000 };
                }
            }
            super::backpressure::PressureLevel::Normal => {
                // Gradually return to streaming after sustained calm.
                if history.iter().all(|r| r.pressure_level == super::backpressure::PressureLevel::Normal) {
                    *current = ProcessingMode::Streaming;
                }
            }
            super::backpressure::PressureLevel::Blocked => {
                *current = ProcessingMode::BulkBatch { batch_interval_secs: 30 };
            }
        }

        *self.mode_entered_at.write().await = Instant::now();
        current.clone()
    }

    /// Get the current mode.
    pub async fn current_mode(&self) -> ProcessingMode {
        self.mode.read().await.clone()
    }

    /// How long the current mode has been active.
    pub async fn mode_duration(&self) -> Duration {
        self.mode_entered_at.read().await.elapsed()
    }
}
