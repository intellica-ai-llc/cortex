use cortex_validate::result_aggregator::AnalysisReport;
use serde::{Deserialize, Serialize};

pub struct TableGenerator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedTable {
    pub name: String,
    pub latex_code: String,
    pub csv_content: String,
    pub caption: String,
    pub label: String,
}

impl TableGenerator {
    pub fn new() -> Self { Self }
    pub fn generate_from_report(&self, report: &AnalysisReport) -> Vec<RenderedTable> {
        let csv = std::iter::once("metric,value".to_string())
            .chain(report.metrics.iter().map(|m| format!("{},{:.4}", m.name, m.value)))
            .collect::<Vec<_>>().join("\n");

        vec![RenderedTable {
            name: format!("{}-metrics", report.experiment_id),
            latex_code: format!("% LaTeX table for {}\n\\begin{{tabular}}...\\end{{tabular}}", report.experiment_name),
            csv_content: csv,
            caption: format!("Metrics for {}", report.experiment_name),
            label: format!("tab:{}", report.experiment_id),
        }]
    }
}
