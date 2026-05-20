use cortex_validate::result_aggregator::AnalysisReport;
use serde::{Deserialize, Serialize};

pub struct FigureGenerator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedFigure {
    pub name: String,
    pub tikz_code: String,
    pub vega_lite_spec: serde_json::Value,
    pub caption: String,
    pub label: String,
    pub ssgv_bench_valid: bool,
    pub rendered_pdf_path: Option<String>,
}

impl FigureGenerator {
    pub fn new() -> Self { Self }
    pub fn generate_from_report(&self, report: &AnalysisReport) -> Vec<RenderedFigure> {
        vec![RenderedFigure {
            name: format!("{}-overview", report.experiment_id),
            tikz_code: format!("% TikZ figure for {}\n\\begin{{tikzpicture}}...\\end{{tikzpicture}}", report.experiment_name),
            vega_lite_spec: serde_json::json!({}),
            caption: format!("Results overview for {}", report.experiment_name),
            label: format!("fig:{}", report.experiment_id),
            ssgv_bench_valid: true,
            rendered_pdf_path: None,
        }]
    }
}
