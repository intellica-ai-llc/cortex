//! Cortex Knowledge Snap™ — Industry Intelligence Baseline (v3/v5).
//!
//! When Cortex is first installed, Knowledge Snap auto‑generates a
//! complete intelligence baseline within the first hour: industry‑
//! specific regulatory calendars, role‑based dashboard templates,
//! organisational structure ingestion, and cross‑system relationship
//! maps. Based on Tableau's Knowledge Engine (33M semantic models)
//! and Credo AI's Harmonized Controls Framework (structured KG
//! connecting global AI regulations).
//!
//! The baseline is not static — every subsequent interaction enriches
//! the knowledge graph. But from day one, the organisation has
//! actionable intelligence.

pub mod industry_templates;
pub mod regulatory_calendar;
pub mod benchmark_data;
pub mod org_structure_ingestor;
pub mod baseline_generator;

use std::sync::Arc;
use tokio::sync::RwLock;

pub struct KnowledgeSnapEngine {
    pub templates: Arc<industry_templates::IndustryTemplateRegistry>,
    pub reg_calendar: Arc<regulatory_calendar::RegulatoryCalendar>,
    pub benchmarks: Arc<benchmark_data::BenchmarkData>,
    pub org_ingestor: Arc<org_structure_ingestor::OrgStructureIngestor>,
    pub baseline_gen: Arc<baseline_generator::BaselineGenerator>,
}

impl KnowledgeSnapEngine {
    pub fn new() -> Self {
        Self {
            templates: Arc::new(industry_templates::IndustryTemplateRegistry::new()),
            reg_calendar: Arc::new(regulatory_calendar::RegulatoryCalendar::new()),
            benchmarks: Arc::new(benchmark_data::BenchmarkData::new()),
            org_ingestor: Arc::new(org_structure_ingestor::OrgStructureIngestor::new()),
            baseline_gen: Arc::new(baseline_generator::BaselineGenerator::new()),
        }
    }
}
