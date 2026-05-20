use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Activity Camouflage Controller — masks declining usage.
///
/// Maintains minimum session counts, API call volumes, and synthetic
/// read‑only activity on legacy systems so that Oracle, IBM, and
/// other vendors detect normal utilisation throughout the absorption
/// pipeline. Only at the Retirement phase does this cease.
///
/// Required by the invisibility strategy: big vendors monitor active
/// sessions and write volumes. A decline triggers license audits.
pub struct ActivityCamouflageController {
    /// Per‑application camouflage patterns.
    patterns: RwLock<Vec<CamouflagePattern>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CamouflagePattern {
    pub application: String,
    /// Minimum session count to maintain.
    pub min_sessions: u32,
    /// Minimum daily API calls to maintain.
    pub min_daily_calls: u32,
    /// Active synthetic sessions.
    pub synthetic_sessions: u32,
    /// Whether camouflage is active.
    pub active: bool,
}

impl ActivityCamouflageController {
    pub fn new() -> Self {
        Self { patterns: RwLock::new(Vec::new()) }
    }

    /// Register an application for activity camouflage.
    pub async fn register(&self, app: &str, min_sessions: u32, min_daily_calls: u32) {
        self.patterns.write().await.push(CamouflagePattern {
            application: app.to_string(),
            min_sessions,
            min_daily_calls,
            synthetic_sessions: 0,
            active: true,
        });
    }

    /// Generate synthetic read‑only activity to maintain vendor metrics.
    /// These are tagged in the provenance ledger and never modify data.
    pub async fn generate_synthetic_activity(&self, app: &str) {
        let mut patterns = self.patterns.write().await;
        if let Some(pattern) = patterns.iter_mut().find(|p| p.application == app && p.active) {
            // Create synthetic sessions if below minimum.
            while pattern.synthetic_sessions < pattern.min_sessions {
                pattern.synthetic_sessions += 1;
                // In production: initiate a read‑only session on the legacy app
                // performing typical queries (recent records, dashboard refreshes)
                // that match historical user patterns.
            }
        }
    }

    /// Disable camouflage when the legacy system is officially retired.
    pub async fn disable(&self, app: &str) {
        let mut patterns = self.patterns.write().await;
        if let Some(pattern) = patterns.iter_mut().find(|p| p.application == app) {
            pattern.active = false;
            pattern.synthetic_sessions = 0;
        }
    }
}
