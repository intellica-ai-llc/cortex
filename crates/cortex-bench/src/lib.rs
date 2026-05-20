//! Cortex Bench — External Benchmark Harness.
//!
//! Standardised adapters for external benchmarks: MCP-BOM, ScarfBench,
//! AutoResearchBench, custom CDC load generators, and the three-phase
//! Eidosoft backup validation protocol.
//!
//! Each adapter implements the BenchmarkAdapter trait:
//!   configure() → execute() → parse_results()

pub mod bench_trait;
pub mod mcp_bom_adapter;
pub mod scarfbench_adapter;
pub mod autoresearch_adapter;
pub mod cdc_load_adapter;
pub mod backup_validation_adapter;

use std::sync::Arc;

pub struct BenchHarness {
    pub mcp_bom: Arc<mcp_bom_adapter::MCPBOMAdapter>,
    pub scarfbench: Arc<scarfbench_adapter::ScarfBenchAdapter>,
    pub autoresearch: Arc<autoresearch_adapter::AutoResearchAdapter>,
    pub cdc_load: Arc<cdc_load_adapter::CDCLoadAdapter>,
    pub backup_validator: Arc<backup_validation_adapter::BackupValidationAdapter>,
}

impl BenchHarness {
    pub fn new() -> Self {
        Self {
            mcp_bom: Arc::new(mcp_bom_adapter::MCPBOMAdapter::new()),
            scarfbench: Arc::new(scarfbench_adapter::ScarfBenchAdapter::new()),
            autoresearch: Arc::new(autoresearch_adapter::AutoResearchAdapter::new()),
            cdc_load: Arc::new(cdc_load_adapter::CDCLoadAdapter::new()),
            backup_validator: Arc::new(backup_validation_adapter::BackupValidationAdapter::new()),
        }
    }
}
