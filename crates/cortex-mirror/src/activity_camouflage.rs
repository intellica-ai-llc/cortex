use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Vendor Activity Camouflage — maintains normal usage metrics.
///
/// Large vendors (Oracle, IBM) monitor license utilisation,
/// session counts, and API call volumes. If these decline, they
/// can trigger audits or detect migration. This controller
/// generates synthetic read‑only activity on the source system
/// to keep metrics at historical levels until the official
/// retirement cutover.
pub struct MirrorActivityCamouflage {
    patterns: RwLock<Vec<CamouflagePattern>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamouflagePattern {
    pub source: String,
    pub min_sessions: u32,
    pub min_daily_queries: u32,
    pub synthetic_sessions: u32,
    pub active: bool,
}

impl MirrorActivityCamouflage {
    pub fn new() -> Self {
        Self { patterns: RwLock::new(Vec::new()) }
    }

    pub async fn register(&self, source: &str, min_sessions: u32, min_daily_queries: u32) {
        self.patterns.write().await.push(CamouflagePattern {
            source: source.to_string(),
            min_sessions,
            min_daily_queries,
            synthetic_sessions: 0,
            active: true,
        });
    }

    pub async fn generate_synthetic_load(&self, source: &str) {
        let mut patterns = self.patterns.write().await;
        if let Some(p) = patterns.iter_mut().find(|p| p.source == source && p.active) {
            while p.synthetic_sessions < p.min_sessions {
                // Production: open a read‑only connection, execute typical
                // queries (e.g., fetch recent records) matching user patterns.
                p.synthetic_sessions += 1;
            }
        }
    }

    pub async fn disable(&self, source: &str) {
        if let Some(p) = self.patterns.write().await.iter_mut().find(|p| p.source == source) {
            p.active = false;
            p.synthetic_sessions = 0;
        }
    }
}
