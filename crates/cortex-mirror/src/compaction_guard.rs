use std::sync::atomic::{AtomicI64, Ordering};

/// Compaction‑aware admission control — prevents the LSM
/// Compaction Spiral of Death.
///
/// In LSM‑based systems (which power TraceDB’s absorption tables),
/// compaction is a background task. If sized for 90% utilisation
/// during normal hours, the system fails during a burst because
/// CPU resources required for compaction are stolen by ingestion.
/// This leads to exponential growth of unmerged files and eventual
/// system crash (Chursin et al., Tidehunter, Feb 2026).
///
/// The solution: when compaction debt exceeds a threshold, throttle
/// CDC ingestion to allow compaction to catch up.
pub struct CompactionGuard {
    /// Current compaction debt in GB (unmerged file volume).
    compaction_debt_gb: AtomicI64,
    /// Maximum tolerable debt before throttling.
    max_debt_gb: i64,
    /// Minimum debt for safe operation.
    safe_debt_gb: i64,
    /// Whether ingestion is currently throttled.
    throttled: std::sync::atomic::AtomicBool,
}

impl CompactionGuard {
    pub fn new(max_debt_gb: i64, safe_debt_gb: i64) -> Self {
        Self {
            compaction_debt_gb: AtomicI64::new(0),
            max_debt_gb,
            safe_debt_gb,
            throttled: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Update the observed compaction debt.
    pub fn update_debt(&self, debt_gb: i64) {
        self.compaction_debt_gb.store(debt_gb, Ordering::SeqCst);
    }

    /// Check if ingestion should be throttled.
    /// Returns true if CDC events should be paused.
    pub fn should_throttle(&self) -> bool {
        let debt = self.compaction_debt_gb.load(Ordering::SeqCst);
        if debt > self.max_debt_gb {
            self.throttled.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Check if it’s safe to resume.
    pub fn can_resume(&self) -> bool {
        let debt = self.compaction_debt_gb.load(Ordering::SeqCst);
        if debt <= self.safe_debt_gb {
            self.throttled.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Is the guard currently throttling?
    pub fn is_throttled(&self) -> bool {
        self.throttled.load(Ordering::SeqCst)
    }
}
