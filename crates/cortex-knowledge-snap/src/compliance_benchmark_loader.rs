//! Loads industry benchmark data from f7i.ai, APQC, BPR Global, NERC GADS/OS.
//!
//! f7i.ai 2026 benchmarks: "Reactive work should be <10% of total maintenance
//! hours. … a 4‑8% reduction in total energy bills within 12 months. … Overall
//! Equipment Effectiveness (OEE) gold standard is 85%, national average 60‑65%.
//! … AI‑driven predictive maintenance facilities seeing 30‑50% reduction in
//! total machine downtime and 20‑40% extension in remaining useful life."
//!
//! APQC Open Standards Benchmarking: cross‑industry finance KPIs (total cost
//! per process, cycle time, efficiency ratios, staffing productivity).
//!
//! NERC GADS/OS: open‑source generating unit reliability benchmarks – frequency
//! and severity of forced outages, preventive maintenance compliance rates,
//! unit availability factors, planned outage duration benchmarks.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ComplianceBenchmarkLoader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkDataset {
    pub industry: String,
    pub benchmarks: HashMap<String, BenchmarkValue>,
    pub source: String,
    pub as_of_year: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkValue {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub target_direction: TargetDirection,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetDirection { Higher, Lower, Exact }

impl ComplianceBenchmarkLoader {
    pub fn new() -> Self { Self }

    /// Load all industry benchmark datasets.
    ///
    /// Sources:
    ///   - f7i.ai 2026: PMC, reactive work%, OEE, MTTD, energy reduction
    ///   - APQC 2026: finance cost, cycle time, productivity
    ///   - NERC GADS/OS: unit availability, forced outage rate, PM compliance
    ///   - BPR Global: cross‑sector financial medians
    pub fn load_all() -> Vec<BenchmarkDataset> {
        vec![
            // Energy & Utilities benchmarks (f7i.ai + NERC GADS/OS)
            BenchmarkDataset {
                industry: "energy_utilities".into(),
                benchmarks: HashMap::from([
                    ("pmc".into(), BenchmarkValue { name: "Preventive Maintenance Compliance".into(), value: 95.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "Top‑performing plants exceed 95% PMC".into() }),
                    ("reactive_work".into(), BenchmarkValue { name: "Reactive Work".into(), value: 10.0, unit: "%".into(), target_direction: TargetDirection::Lower, description: "Should be <10% of total maintenance hours".into() }),
                    ("oee".into(), BenchmarkValue { name: "Overall Equipment Effectiveness".into(), value: 85.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "Gold standard; national average 60‑65%".into() }),
                    ("mttd".into(), BenchmarkValue { name: "Mean Time to Detect".into(), value: 5.0, unit: "minutes".into(), target_direction: TargetDirection::Lower, description: "Target <5min for AI‑enabled PdM facilities".into() }),
                    ("forced_outage_rate".into(), BenchmarkValue { name: "Forced Outage Rate".into(), value: 1.0, unit: "%".into(), target_direction: TargetDirection::Lower, description: "NERC GADS top‑quartile benchmark".into() }),
                    ("energy_reduction".into(), BenchmarkValue { name: "Energy Reduction YoY".into(), value: 8.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "4‑8% reduction in total energy bills within 12 months".into() }),
                ]),
                source: "f7i.ai 2026 + NERC GADS/OS".into(),
                as_of_year: 2026,
            },
            // Manufacturing benchmarks (f7i.ai + APQC)
            BenchmarkDataset {
                industry: "manufacturing".into(),
                benchmarks: HashMap::from([
                    ("pmc".into(), BenchmarkValue { name: "Preventive Maintenance Compliance".into(), value: 95.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "Best‑in‑class manufacturing PMC".into() }),
                    ("reactive_work".into(), BenchmarkValue { name: "Reactive Work".into(), value: 10.0, unit: "%".into(), target_direction: TargetDirection::Lower, description: "World‑class manufacturers keep reactive work <10%".into() }),
                    ("oee".into(), BenchmarkValue { name: "Overall Equipment Effectiveness".into(), value: 85.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "Gold standard for discrete/process manufacturing".into() }),
                    ("supplier_compliance".into(), BenchmarkValue { name: "Supplier Compliance Rate".into(), value: 97.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "APQC top‑quartile manufacturing benchmark".into() }),
                ]),
                source: "f7i.ai 2026 + APQC Open Standards Benchmarking".into(),
                as_of_year: 2026,
            },
            // Banking benchmarks (APQC + BPR Global)
            BenchmarkDataset {
                industry: "banking".into(),
                benchmarks: HashMap::from([
                    ("roa".into(), BenchmarkValue { name: "Return on Assets".into(), value: 1.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "BPR Global median for banking sector".into() }),
                    ("roe".into(), BenchmarkValue { name: "Return on Equity".into(), value: 10.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "BPR Global median for banking sector".into() }),
                    ("nim".into(), BenchmarkValue { name: "Net Interest Margin".into(), value: 3.2, unit: "%".into(), target_direction: TargetDirection::Higher, description: "BPR Global median for banking sector".into() }),
                    ("cost_to_income".into(), BenchmarkValue { name: "Cost‑to‑Income Ratio".into(), value: 55.0, unit: "%".into(), target_direction: TargetDirection::Lower, description: "APQC banking benchmark".into() }),
                ]),
                source: "APQC 2026 + BPR Global".into(),
                as_of_year: 2026,
            },
            // Healthcare benchmarks (APQC)
            BenchmarkDataset {
                industry: "healthcare".into(),
                benchmarks: HashMap::from([
                    ("medical_loss_ratio".into(), BenchmarkValue { name: "Medical Loss Ratio".into(), value: 85.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "ACA minimum MLR for large group".into() }),
                    ("claims_cycle_time".into(), BenchmarkValue { name: "Claims Processing Cycle Time".into(), value: 5.0, unit: "days".into(), target_direction: TargetDirection::Lower, description: "APQC top‑quartile healthcare benchmark".into() }),
                ]),
                source: "APQC Open Standards Benchmarking 2026".into(),
                as_of_year: 2026,
            },
            // Insurance benchmarks (APQC + NAIC)
            BenchmarkDataset {
                industry: "insurance".into(),
                benchmarks: HashMap::from([
                    ("combined_ratio".into(), BenchmarkValue { name: "Combined Ratio".into(), value: 95.0, unit: "%".into(), target_direction: TargetDirection::Lower, description: "Industry benchmark; <100% indicates underwriting profit".into() }),
                    ("claims_severity".into(), BenchmarkValue { name: "Claims Severity Trend".into(), value: 3.0, unit: "% YoY".into(), target_direction: TargetDirection::Lower, description: "APQC insurance benchmark".into() }),
                ]),
                source: "APQC 2026 + NAIC".into(),
                as_of_year: 2026,
            },
            // Legal benchmarks (APQC)
            BenchmarkDataset {
                industry: "legal".into(),
                benchmarks: HashMap::from([
                    ("billable_hours_target".into(), BenchmarkValue { name: "Billable Hours Target".into(), value: 1800.0, unit: "hours/year".into(), target_direction: TargetDirection::Higher, description: "Industry standard for associates".into() }),
                    ("realisation_rate".into(), BenchmarkValue { name: "Realisation Rate".into(), value: 92.0, unit: "%".into(), target_direction: TargetDirection::Higher, description: "APQC legal benchmark".into() }),
                ]),
                source: "APQC Open Standards Benchmarking 2026".into(),
                as_of_year: 2026,
            },
        ]
    }

    /// Get benchmarks for a specific industry.
    pub fn for_industry(industry: &str) -> Option<BenchmarkDataset> {
        Self::load_all().into_iter().find(|d| d.industry == industry)
    }
}
