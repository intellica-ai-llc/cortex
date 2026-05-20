use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Queue Depth Guard — protects against unbounded buffer growth.
///
/// ZongJi issue #73 (March 21, 2026) demonstrated that in the absence
/// of backpressure, a single heavy write workload can cause a node's
/// memory to explode. This guard monitors internal buffers and
/// employs bounded queues with TTL and drop‑oldest policies when
/// depth exceeds safe limits.
pub struct QueueDepthGuard {
    max_depth: u64,
    ttl_ms: u64,
    current_depth: AtomicU64,
    oldest_insert: tokio::sync::Mutex<Option<Instant>>,
    dropped_count: AtomicU64,
}

impl QueueDepthGuard {
    pub fn new(max_depth: u64, ttl_ms: u64) -> Self {
        Self {
            max_depth,
            ttl_ms,
            current_depth: AtomicU64::new(0),
            oldest_insert: tokio::sync::Mutex::new(None),
            dropped_count: AtomicU64::new(0),
        }
    }

    /// Attempt to enqueue. If queue is full and drop‑oldest policy
    /// is enabled, the oldest item will be evicted.
    pub async fn try_enqueue(&self) -> bool {
        let depth = self.current_depth.load(Ordering::SeqCst);
        if depth >= self.max_depth {
            // Drop oldest if TTL has expired.
            let mut oldest = self.oldest_insert.lock().await;
            if let Some(instant) = *oldest {
                if instant.elapsed() > Duration::from_millis(self.ttl_ms) {
                    self.current_depth.fetch_sub(1, Ordering::SeqCst);
                    self.dropped_count.fetch_add(1, Ordering::SeqCst);
                    *oldest = Some(Instant::now());
                    return true; // space freed
                }
            }
            false // queue full, oldest still valid
        } else {
            self.current_depth.fetch_add(1, Ordering::SeqCst);
            if self.oldest_insert.lock().await.is_none() {
                *self.oldest_insert.lock().await = Some(Instant::now());
            }
            true
        }
    }

    /// Dequeue completed, decrement depth.
    pub fn dequeue(&self) {
        let prev = self.current_depth.fetch_sub(1, Ordering::SeqCst);
        if prev == 1 {
            // queue empty, reset timer.
            if let Ok(mut oldest) = self.oldest_insert.try_lock() {
                *oldest = None;
            }
        }
    }

    pub fn dropped_count(&self) -> u64 {
        self.dropped_count.load(Ordering::SeqCst)
    }

    pub fn current_depth(&self) -> u64 {
        self.current_depth.load(Ordering::SeqCst)
    }
}
