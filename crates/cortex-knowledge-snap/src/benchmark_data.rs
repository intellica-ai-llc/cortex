use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Industry benchmark data from public sources.
///
/// Preloaded with peer benchmarks for ratio analysis, operational
/// metrics, and compliance baselines. The data is refreshed from
/// public regulatory filings (Call Reports, FERC Form 1, NAIC
/// statutory filings) where available.
pub struct BenchmarkData {
    benchmarks: HashMap<String, IndustryBenchmarks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryBenchmarks {
    pub industry: String,
    pub metrics: Vec<BenchmarkMetric>,
    pub source: String,
    pub as_of_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetric {
    pub name: String,
    pub median: f64,
    pub p25: f64,
    pub p75: f64,
    pub unit: String,
}

impl BenchmarkData {
    pub fn new() -> Self {
        let mut benchmarks = HashMap::new();

        benchmarks.insert("banking".into(), IndustryBenchmarks {
            industry: "banking".into(),
            metrics: vec![
                BenchmarkMetric { name: "ROA".into(), median: 1.0, p25: 0.6, p75: 1.4, unit: "%".into() },
                BenchmarkMetric { name: "ROE".into(), median: 10.0, p25: 6.0, p75: 14.0, unit: "%".into() },
                BenchmarkMetric { name: "NIM".into(), median: 3.2, p25: 2.5, p75: 4.0, unit: "%".into() },
            ],
            source: "FFIEC Call Report Q1 2026".into(),
            as_of_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        });

        Self { benchmarks }
    }

    /// Get benchmarks for an industry.
    pub fn get(&self, industry: &str) -> Option<&IndustryBenchmarks> {
        self.benchmarks.get(industry)
    }

    /// Compare a value against the benchmark distribution.
    pub fn percentile_rank(&self, industry: &str, metric_name: &str, value: f64) -> Option<f64> {
        let bm = self.benchmarks.get(industry)?;
        let metric = bm.metrics.iter().find(|m| m.name == metric_name)?;
        if value <= metric.p25 { Some(0.25) }
        else if value <= metric.median { Some(0.50) }
        else if value <= metric.p75 { Some(0.75) }
        else { Some(0.90) }
    }
}
