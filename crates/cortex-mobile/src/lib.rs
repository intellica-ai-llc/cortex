//! Cortex Mobile Brain — LFAB On‑Device Intelligence (v11).
//!
//! LFAB's entire cognitive runtime — the S‑HAI Core, Predictive World
//! Engine, token pruner, latent bridge, and WoVR‑safe dream engine —
//! becomes the on‑device intelligence layer for Cortex. Mobile TraceDB
//! (SQLite + Zvec + CRDT sync via ElectricSQL) brings the Observation
//! and Mirror phases to every smartphone, tablet, and edge device in
//! the enterprise.
//!
//! Based on LFM2.5-1.2B-Thinking (Liquid AI, Jan 2026): 900 MB RAM
//! on‑device reasoning model with agentic tool‑calling capability.
//! ElectricSQL (FOSDEM 2026) provides PostgreSQL↔SQLite bidirectional
//! CRDT sync with offline‑first writes and automatic reconnect.
//!
//! Architecture (ClawMobile pattern):
//!   LFAB S‑HAI Core (probabilistic planning) → deterministic control
//!   layer → Native UI Parsing | System APIs | Local TraceDB.
//! Simple tasks route to on‑device LFAB; complex subtasks escalate
//! to the Cortex server only when necessary (OpenPhone pattern).

pub mod hierarchical_controller;
pub mod device_cloud_router;
pub mod mobile_tracedb;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CortexMobileBrain {
    pub controller: Arc<hierarchical_controller::HierarchicalController>,
    pub cloud_router: Arc<device_cloud_router::DeviceCloudRouter>,
    pub mobile_db: Arc<mobile_tracedb::MobileTraceDB>,
    /// Active mobile sessions indexed by device ID.
    sessions: RwLock<std::collections::HashMap<String, MobileSession>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MobileSession {
    pub device_id: String,
    pub user_id: String,
    pub device_type: DeviceType,
    pub online: bool,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    pub synced_traces: u64,
    pub pending_uploads: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum DeviceType { Smartphone, Tablet, Wearable, EdgeGateway }

impl CortexMobileBrain {
    pub fn new(db_path: &str) -> Self {
        Self {
            controller: Arc::new(hierarchical_controller::HierarchicalController::new()),
            cloud_router: Arc::new(device_cloud_router::DeviceCloudRouter::new()),
            mobile_db: Arc::new(mobile_tracedb::MobileTraceDB::new(db_path)),
            sessions: RwLock::new(std::collections::HashMap::new()),
        }
    }
}
