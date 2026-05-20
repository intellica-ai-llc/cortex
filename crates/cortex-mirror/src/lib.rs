//! Cortex Mirror Engine — Direct CDC, Kafka‑Free, Heavy‑Load Proven.
//!
//! Complete: all 25 modules implemented across four parts.
//! Part 4: dual‑write, activity camouflage, offline batch,
//! throughput monitor, queue guard, mirror config.

pub mod cdc_trait;
pub mod cdc_flink;
pub mod cdc_pgstream;
pub mod cdc_redpanda;
pub mod cdc_goldengate;
pub mod cdc_dbconvert;
pub mod cdc_risingwave;
pub mod column_level_cdc;
pub mod backpressure;
pub mod adaptive_throttle;
pub mod compaction_guard;
pub mod freshness_router;
pub mod validation_agent;
pub mod zero_copy_plane;
pub mod io_uring_writer;
pub mod cdc_append_log;
pub mod base_snapshot;
pub mod materialized_view;
pub mod consistency_watermark;
pub mod data_quality_provider;
// Part 4 modules
pub mod dual_write_propagator;
pub mod activity_camouflage;
pub mod offline_cdc_batch;
pub mod event_throughput;
pub mod queue_depth_guard;
pub mod mirror_config;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Top‑level Mirror orchestrator.
pub struct MirrorEngine {
    handles: RwLock<std::collections::HashMap<String, Box<dyn cdc_trait::CdcBackend>>>,
    pub backpressure: Arc<backpressure::MultiLayerBackpressure>,
    pub throttle: Arc<adaptive_throttle::AdaptiveThrottle>,
    pub compaction_guard: Arc<compaction_guard::CompactionGuard>,
    pub freshness_router: Arc<freshness_router::FreshnessRouter>,
    pub column_filter: Arc<column_level_cdc::ColumnLevelCdcFilter>,
    pub validation_agent: Arc<validation_agent::PostMirrorValidationAgent>,
    pub zero_copy: Arc<zero_copy_plane::ZeroCopyDataPlane>,
    pub io_writer: Arc<io_uring_writer::IoUringWriter>,
    pub cdc_log: Arc<cdc_append_log::CdcAppendLog>,
    pub base_snapshot: Arc<base_snapshot::BaseSnapshotManager>,
    pub materialized_views: Arc<materialized_view::MaterializedViewManager>,
    pub watermark: Arc<consistency_watermark::CrossSourceWatermark>,
    pub data_quality: Arc<RwLock<Option<Box<dyn data_quality_provider::DataQualityProvider>>>>,
    // Part 4
    pub dual_write: Arc<dual_write_propagator::MirrorDualWritePropagator>,
    pub camouflage: Arc<activity_camouflage::MirrorActivityCamouflage>,
    pub offline_batch: Arc<offline_cdc_batch::OfflineCdcBatchEngine>,
    pub throughput: Arc<event_throughput::EventThroughputMonitor>,
    pub queue_guard: Arc<queue_depth_guard::QueueDepthGuard>,
}

impl MirrorEngine {
    pub fn new(pool: sqlx::PgPool, batch_dir: &str) -> Self {
        Self {
            handles: RwLock::new(std::collections::HashMap::new()),
            backpressure: Arc::new(backpressure::MultiLayerBackpressure::new()),
            throttle: Arc::new(adaptive_throttle::AdaptiveThrottle::new()),
            compaction_guard: Arc::new(compaction_guard::CompactionGuard::new(20, 5)),
            freshness_router: Arc::new(freshness_router::FreshnessRouter::new()),
            column_filter: Arc::new(column_level_cdc::ColumnLevelCdcFilter::new()),
            validation_agent: Arc::new(validation_agent::PostMirrorValidationAgent::new()),
            zero_copy: Arc::new(zero_copy_plane::ZeroCopyDataPlane::new(true, 4096)),
            io_writer: Arc::new(io_uring_writer::IoUringWriter::new(true, 256, true, true)),
            cdc_log: Arc::new(cdc_append_log::CdcAppendLog::new(pool.clone())),
            base_snapshot: Arc::new(base_snapshot::BaseSnapshotManager::new(15)),
            materialized_views: Arc::new(materialized_view::MaterializedViewManager::new()),
            watermark: Arc::new(consistency_watermark::CrossSourceWatermark::new()),
            data_quality: Arc::new(RwLock::new(None)),
            dual_write: Arc::new(dual_write_propagator::MirrorDualWritePropagator::new()),
            camouflage: Arc::new(activity_camouflage::MirrorActivityCamouflage::new()),
            offline_batch: Arc::new(offline_cdc_batch::OfflineCdcBatchEngine::new(batch_dir, None)),
            throughput: Arc::new(event_throughput::EventThroughputMonitor::new(100_000)),
            queue_guard: Arc::new(queue_depth_guard::QueueDepthGuard::new(10_000, 5000)),
        }
    }

    pub async fn register(&self, source: &str, backend: Box<dyn cdc_trait::CdcBackend>) {
        self.handles.write().await.insert(source.to_string(), backend);
    }

    pub async fn set_data_quality_provider(
        &self,
        provider: Box<dyn data_quality_provider::DataQualityProvider>,
    ) {
        *self.data_quality.write().await = Some(provider);
    }
}
